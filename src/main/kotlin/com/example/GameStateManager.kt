package com.example

import com.example.plugins.SseEvent
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import java.util.UUID

class GameFullException: Exception("game has no spots left for new players")
class InvalidMoveException(message: String): Exception(message)

abstract class Move

// TODO make sure the game has enough players before starting
abstract class GameStateManager<out T: Move> {
    val gameId: UUID = UUID.randomUUID()
    protected val flow = MutableStateFlow(SseEvent("game has not started"))  // TODO figure out better initial val

    fun getFlow(): StateFlow<SseEvent> {
        return flow.asStateFlow()
    }

    abstract fun playMove(playerId: UUID, move: @UnsafeVariance T)  // TODO find a better way

    abstract fun addPlayer(): UUID
}