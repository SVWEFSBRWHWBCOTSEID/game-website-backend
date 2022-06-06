package com.example.plugins

import com.example.GameDoesNotExistException
import com.example.GameManager
import com.example.TicTacToe
import com.example.UltimateTicTacToe
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.SharedFlow
import kotlinx.coroutines.withContext

fun Application.configureRouting() {
    val tttGameManager = GameManager { TicTacToe() }
    val utttGameManager = GameManager { UltimateTicTacToe() }

    routing {
        post("/api/new/{gameType}/") {
            val gameType = call.parameters["gameType"]
                ?: return@post call.respond(HttpStatusCode.BadRequest, "gameType is missing")

            if (gameType == "ttt") {
                val gameId = tttGameManager.createGame()
                call.respond(HttpStatusCode.Accepted, gameId)
            } else if (gameType == "uttt") {
                val gameId = utttGameManager.createGame()
                call.respond(HttpStatusCode.Accepted, gameId)
            }
        }
    }
    routing {
        get("/api/{gameType}/{gameId}") {
            val gameType = call.parameters["gameType"]
                ?: return@get call.respond(HttpStatusCode.BadRequest, "gameType is missing")
            val gameId = call.parameters["gameId"]
                ?: return@get call.respond(HttpStatusCode.BadRequest, "gameId is missing")

            try {
                if (gameType == "ttt") {
                    val game = tttGameManager.getGame(gameId)
                    call.sendSseEvents(game.getFlow())
                } else if (gameType == "uttt") {
                    val game = utttGameManager.getGame(gameId)
                    call.sendSseEvents(game.getFlow())
                }
            } catch (e: GameDoesNotExistException) {
                call.respond(HttpStatusCode.BadRequest, "invalid gameId")
            }
        }
    }
}

// https://github.com/ktorio/ktor-samples/blob/main/sse/src/SseApplication.kt#L109-L137
/**
 * The data class representing a SSE Event that will be sent to the client.
 */
data class SseEvent(val data: String, val event: String? = null, val id: String? = null)

suspend fun ApplicationCall.sendSseEvents(flow: SharedFlow<SseEvent>) {
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