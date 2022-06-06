package com.example

import com.example.plugins.SseEvent
import com.example.plugins.configureRouting
import io.ktor.server.engine.*
import io.ktor.server.netty.*
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.MutableSharedFlow
import kotlinx.coroutines.flow.asSharedFlow
import kotlinx.coroutines.launch

suspend fun main() {
    val flow = MutableSharedFlow<SseEvent>()
    val channels = mutableMapOf("asdf" to flow.asSharedFlow())

    CoroutineScope(SupervisorJob()).launch {
        var i = 1
        while (true) {
            flow.emit(SseEvent("your mother $i"))
            println(i)
            i++
            delay(500)
        }
    }

    embeddedServer(Netty, port = 3000, host = "localhost") {
        configureRouting(channels)
    }.start(wait = true)
}
