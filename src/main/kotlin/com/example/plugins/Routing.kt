package com.example.plugins

import com.example.*
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.withContext
import kotlinx.serialization.Serializable
import java.util.*

fun Application.configureRouting() {
    val tttGameManager = GameManager { TicTacToe() }
    val utttGameManager = GameManager { UltimateTicTacToe() }
    val gameManagers = mapOf("ttt" to tttGameManager, "uttt" to utttGameManager)

    /**
     * Creates a new game. All required information is in the URL.
     * Returns a GamePlayerInfo instance as a JSON upon success or a 400 status code upon error.
     */
    routing {
        post("/api/new/{gameType}") {
            try {
                val gameType = call.getPathParameter("gameType")
                val gameManager = gameManagers[gameType] ?: throw IllegalArgumentException("gameType is invalid")

                val game = gameManager.createGame()
                val playerId = game.addPlayer()
                val info = GamePlayerInfo(game.gameId.toString(), playerId.toString())
                call.respond(HttpStatusCode.Accepted, info)
            } catch (e: IllegalArgumentException) {
                call.respond(HttpStatusCode.BadRequest, e.message ?: "invalid request")
            }
        }
    }
    /**
     * Adds a player to an existing game. All required information is in the URL.
     * Returns a GamePlayerInfo instance as a JSON upon success or a 400 status code upon error.
     */
    routing {
        post("/api/join/{gameType}/{gameId}") {
            try {
                val gameType = call.getPathParameter("gameType")
                val gameManager = gameManagers[gameType]
                    ?: return@post call.respond(HttpStatusCode.BadRequest, "gameType is invalid")

                val gameIdStr = call.getPathParameter("gameId")
                val gameId = UUID.fromString(gameIdStr)

                val game = gameManager.getGame(gameId)
                val playerId = game.addPlayer()
                val info = GamePlayerInfo(game.gameId.toString(), playerId.toString())
                call.respond(HttpStatusCode.Accepted, info)
            } catch (e: Exception) {
                when (e) {
                    is IllegalArgumentException -> call.respond(HttpStatusCode.BadRequest, e.message ?: "invalid request")
                    is GameFullException -> call.respond(HttpStatusCode.BadRequest, "game is already full")
                    else -> throw e
                }
            }
        }
    }
    /**
     * Opens an SSE connection for an existing game. All required information is in the URL.
     */
    routing {
        get("/api/game/{gameType}/{gameId}") {
            try {
                val gameType = call.getPathParameter("gameType")
                val gameIdStr = call.getPathParameter("gameId")
                val gameId = UUID.fromString(gameIdStr)
                val gameManager = gameManagers[gameType] ?: throw IllegalArgumentException("gameType is invalid")

                val game = gameManager.getGame(gameId)
                call.sendSseEvents(game.getFlow())
            } catch (e: Exception) {
                when (e) {
                    is IllegalArgumentException -> call.respond(HttpStatusCode.BadRequest, e.message ?: "invalid request")
                    is GameDoesNotExistException -> call.respond(HttpStatusCode.BadRequest, "game not found")
                    else -> throw e
                }
            }
        }
    }
    /**
     * Makes a move for the given playerId.
     * gameType and gameId are provided in the URL; move information is provided in request body.
     * Returns 200 with empty body upon success or 400 upon failure
     */
    routing {
        post("/api/game/{gameType}/{gameId}") {
            try {
                val gameType = call.getPathParameter("gameType")
                val gameIdStr = call.getPathParameter("gameId")
                val gameId = UUID.fromString(gameIdStr)
                if (gameType == "ttt") {
                    val game = tttGameManager.getGame(gameId)
                    val move = call.receive<TicTacToeMove>()
                    game.playMove(move)
                } else if (gameType == "uttt") {
                    val game = utttGameManager.getGame(gameId)
                    val move = call.receive<UltimateTicTacToeMove>()
                    game.playMove(move)
                }
            } catch (e: Exception) {
                when (e) {
                    is InvalidMoveException -> call.respond(HttpStatusCode.BadRequest, e.message ?: "move is invalid")
                    is IllegalArgumentException -> call.respond(HttpStatusCode.BadRequest, e.message ?: "invalid request")
                    is GameDoesNotExistException -> call.respond(HttpStatusCode.BadRequest, "game not found")
                    else -> throw e
                }
            }

            call.respond(HttpStatusCode.Accepted)
        }
    }
}

/**
 * Retrieves the specified parameter from the URL. Throws IllegalArgumentException if gameType is not found.
 * This is meant to reduce duplicate code.
 */
fun ApplicationCall.getPathParameter(paramName: String): String {
    return parameters[paramName] ?: throw IllegalArgumentException("$paramName is missing")
}

@Serializable
data class GamePlayerInfo(val gameId: String, val playerId: String)

// based on https://github.com/ktorio/ktor-samples/blob/main/sse/src/SseApplication.kt#L109-L137 but with StateFlow
/**
 * Data class representing an SSE Event that will be sent to the client.
 */
data class SseEvent(val data: String, val event: String? = null, val id: String? = null)

suspend fun ApplicationCall.sendSseEvents(flow: StateFlow<SseEvent>) {
    response.cacheControl(CacheControl.NoCache(null))
    respondTextWriter(contentType = ContentType.Text.EventStream) {
        flow.collect { event ->
            withContext(Dispatchers.IO) {
                if (event.id != null) {
                    write("id: ${event.id}\n")
                }
                if (event.event != null) {
                    write("event: ${event.event}\n")
                }
                for (dataLine in event.data.lines()) {
                    write("data: $dataLine\n")
                }
                write("\n")
                flush()
            }
        }
    }
}
