package com.example

import com.example.plugins.SseEvent
import kotlinx.coroutines.flow.MutableSharedFlow
import kotlinx.coroutines.flow.SharedFlow
import kotlinx.coroutines.flow.asSharedFlow
import java.util.UUID

class GameFullException: Exception("game has no spots left for new players")
class InvalidMoveException(message: String): Exception(message)

abstract class Move

abstract class GameStateManager<out T: Move> {
    val gameId: UUID = UUID.randomUUID()
    private val flow = MutableSharedFlow<SseEvent>()

    fun getFlow(): SharedFlow<SseEvent> {
        return flow.asSharedFlow()
    }

    abstract fun playMove(move: @UnsafeVariance T)  // TODO find a better way

    abstract fun addPlayer(): UUID
}