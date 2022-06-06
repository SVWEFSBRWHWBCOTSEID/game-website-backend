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

    // create new game
    routing {
        post("/api/new/{gameType}") {
            val gameType = call.parameters["gameType"]
                ?: return@post call.respond(HttpStatusCode.BadRequest, "gameType is missing")
            val gameManager = gameManagers[gameType]
                ?: return@post call.respond(HttpStatusCode.BadRequest, "gameType is invalid")

            val game = gameManager.createGame()
            val playerId = game.addPlayer()
            val info = GamePlayerInfo(game.gameId.toString(), playerId.toString())
            call.respond(HttpStatusCode.Accepted, info)
        }
    }
    // join existing game
    routing {
        post("/api/join/{gameType}/{gameId}") {
            val gameType = call.parameters["gameType"]
                ?: return@post call.respond(HttpStatusCode.BadRequest, "gameType is missing")
            val gameIdStr = call.parameters["gameId"]
                ?: return@post call.respond(HttpStatusCode.BadRequest, "gameId is missing")
            val gameId = try {
                UUID.fromString(gameIdStr)
            } catch (e: IllegalArgumentException) {
                return@post call.respond(HttpStatusCode.BadRequest, "gameId is invalid")
            }

            val gameManager = gameManagers[gameType]
                ?: return@post call.respond(HttpStatusCode.BadRequest, "gameType is invalid")
            val game = gameManager.getGame(gameId)
            try {
                val playerId = game.addPlayer()
                val info = GamePlayerInfo(game.gameId.toString(), playerId.toString())
                call.respond(HttpStatusCode.Accepted, info)
            } catch (e: GameFullException) {
                call.respond(HttpStatusCode.BadRequest, "game is already full")
            }
        }
    }
    // fetch game state
    routing {
        get("/api/game/{gameType}/{gameId}") {
            val gameType = call.parameters["gameType"]
                ?: return@get call.respond(HttpStatusCode.BadRequest, "gameType is missing")
            val gameIdStr = call.parameters["gameId"]
                ?: return@get call.respond(HttpStatusCode.BadRequest, "gameId is missing")
            val gameId = try {
                UUID.fromString(gameIdStr)
            } catch (e: IllegalArgumentException) {
                return@get call.respond(HttpStatusCode.BadRequest, "gameId is invalid")
            }
            val gameManager = gameManagers[gameType]
                ?: return@get call.respond(HttpStatusCode.BadRequest, "gameType is invalid")

            try {
                val game = gameManager.getGame(gameId)
                call.sendSseEvents(game.getFlow())
            } catch (e: GameDoesNotExistException) {
                call.respond(HttpStatusCode.BadRequest, "gameId not found")
            }
        }
    }
    // make move
    routing {
        post("/api/game/{gameType}/{gameId}") {
            val gameType = call.parameters["gameType"]
                ?: return@post call.respond(HttpStatusCode.BadRequest, "gameType is missing")
            val gameIdStr = call.parameters["gameId"]
                ?: return@post call.respond(HttpStatusCode.BadRequest, "gameId is missing")
            val gameId = try {
                UUID.fromString(gameIdStr)
            } catch (e: IllegalArgumentException) {
                return@post call.respond(HttpStatusCode.BadRequest, "gameId is invalid")
            }
            try {
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
                when (e) {  // apparently kotlin doesn't have multicatch: https://discuss.kotlinlang.org/t/does-kotlin-have-multi-catch/486/20
                    is GameDoesNotExistException ->
                        return@post call.respond(HttpStatusCode.BadRequest, "gameId not found")
                    is InvalidMoveException, is IllegalArgumentException ->
                        return@post call.respond(HttpStatusCode.BadRequest, "move is invalid")
                    else -> throw e
                }
            }

            call.respond(HttpStatusCode.Accepted)
        }
    }
}

@Serializable
data class GamePlayerInfo(val gameId: String, val playerId: String)

// based on https://github.com/ktorio/ktor-samples/blob/main/sse/src/SseApplication.kt#L109-L137 but with StateFlow
/**
 * The data class representing a SSE Event that will be sent to the client.
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

// http POST localhost:3000/api/new/ttt
// http --form POST localhost:3000/api/game/ttt/<gameId> playerId=<playerId> move='{"tile":0, "symbol":"✕"}'
