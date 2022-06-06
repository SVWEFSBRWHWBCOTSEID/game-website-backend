package com.example

import com.example.plugins.configureRouting
import io.ktor.server.engine.*
import io.ktor.server.netty.*

fun main() {
    embeddedServer(Netty, port = 3000, host = "localhost") {
        configureRouting()
    }.start(wait = true)
}
