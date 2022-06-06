package com.example.plugins

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.SharedFlow
import kotlinx.coroutines.withContext

fun Application.configureRouting(gameChannels: MutableMap<String, SharedFlow<SseEvent>>) {

    // Starting point for a Ktor app:
    routing {
        get("/") {
            call.respondText("Hello World!")
        }
    }
    routing {
        get("/uttt/{gameId}") {
            val gameId = call.parameters["gameId"]
                ?: return@get call.respond(HttpStatusCode.BadRequest, "L + ratio + no game id")

            call.response.cacheControl(CacheControl.NoCache(null))
            call.respondTextWriter(contentType = ContentType.Text.EventStream) {
                gameChannels[gameId]?.collect { event ->
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
    }
}

/**
 * The data class representing a SSE Event that will be sent to the client.
 */
data class SseEvent(val data: String, val event: String? = null, val id: String? = null)
