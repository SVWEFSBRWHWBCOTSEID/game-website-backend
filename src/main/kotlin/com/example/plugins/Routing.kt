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
import kotlinx.serialization.decodeFromString
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import java.util.*

fun Application.configureRouting() {
    val tttGameManager = GameManager { TicTacToe() }
    val utttGameManager = GameManager { UltimateTicTacToe() }

    // create new game
    routing {
        post("/api/new/{gameType}") {
            val gameType = call.parameters["gameType"]
                ?: return@post call.respond(HttpStatusCode.BadRequest, "gameType is missing")

            if (gameType == "ttt") {
                val game = tttGameManager.createGame()
                val playerId = game.addPlayer()
                val info = GamePlayerInfo(game.gameId.toString(), playerId.toString())
                call.respond(HttpStatusCode.Accepted, Json.encodeToString(info))
            } else if (gameType == "uttt") {
                val game = utttGameManager.createGame()
                val playerId = game.addPlayer()
                val info = GamePlayerInfo(game.gameId.toString(), playerId.toString())
                call.respond(HttpStatusCode.Accepted, Json.encodeToString(info))
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

            try {
                if (gameType == "ttt") {
                    val game = tttGameManager.getGame(gameId)
                    call.sendSseEvents(game.getFlow())
                } else if (gameType == "uttt") {
                    val game = utttGameManager.getGame(gameId)
                    call.sendSseEvents(game.getFlow())
                }
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
            val params = call.receiveParameters()
            val playerIdStr = params["playerId"]
                ?: return@post call.respond(HttpStatusCode.BadRequest, "playerId is missing")
            val playerId = try {
                UUID.fromString(playerIdStr)
            } catch (e: IllegalArgumentException) {
                return@post call.respond(HttpStatusCode.BadRequest, "playerId is invalid")
            }
            val moveStr = params["move"]
                ?: return@post call.respond(HttpStatusCode.BadRequest, "move is missing")

            try {
                if (gameType == "ttt") {
                    val game = tttGameManager.getGame(gameId)
                    val move: TicTacToeMove = Json.decodeFromString(moveStr)
                    game.playMove(playerId, move)
                } else if (gameType == "uttt") {
                    val game = utttGameManager.getGame(gameId)
                    val move: UltimateTicTacToeMove = Json.decodeFromString(moveStr)
                    game.playMove(playerId, move)
                }
            } catch (e: Exception) {
                return@post call.respond(HttpStatusCode.BadRequest, "gameId not found or move is invalid")
            }

            call.respond(HttpStatusCode.Accepted)
        }
    }
}

@Serializable
data class GamePlayerInfo(val gameId: String, val playerId: String)

// https://github.com/ktorio/ktor-samples/blob/main/sse/src/SseApplication.kt#L109-L137
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

// http --form POST localhost:3000/api/game/ttt/<gameId> playerId=<playerId> move='{"tile":0, "symbol":"✕"}'
