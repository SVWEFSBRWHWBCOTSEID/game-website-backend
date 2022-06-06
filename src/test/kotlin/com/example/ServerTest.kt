package com.example

import com.example.plugins.GamePlayerInfo
import io.ktor.client.*
import io.ktor.client.call.*
import io.ktor.client.engine.cio.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.client.request.*
import io.ktor.http.*
import io.ktor.http.ContentType.Application.Json
import io.ktor.serialization.kotlinx.json.*
import kotlinx.coroutines.*
import java.util.*
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotEquals


const val API = "http://localhost:3000/api"


// this isn't really a unit test; i just got tired of copy-pasting gameIds over and over
internal class ServerTest {
    private val client = HttpClient(CIO) {
        install(ContentNegotiation) {
            json()
        }
    }

    @Test
    fun tictactoe() {
        val X = "✕"
        val O = "◯"
        runBlocking {
            val gamePlayerInfo1 = client.post("$API/new/ttt").body<GamePlayerInfo>()
            val gameId = UUID.fromString(gamePlayerInfo1.gameId)
            val firstPlayerId = UUID.fromString(gamePlayerInfo1.playerId)

            val gamePlayerInfo2 = client.post("$API/join/ttt/$gameId").body<GamePlayerInfo>()
            val secondPlayerId = UUID.fromString(gamePlayerInfo2.playerId)

            assertNotEquals(firstPlayerId, secondPlayerId)
            assertEquals(gamePlayerInfo1.gameId, gamePlayerInfo2.gameId)

            println("$gameId $firstPlayerId $secondPlayerId")

            val moveStatus = client.post("$API/game/ttt/$gameId") {
                contentType(Json)
                setBody(TicTacToeMove(firstPlayerId, 0, X))
            }.status
            assertEquals(HttpStatusCode.Accepted, moveStatus)
        }
    }
}