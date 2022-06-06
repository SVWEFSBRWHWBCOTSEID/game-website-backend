package com.example

import com.example.plugins.SseEvent
import kotlinx.coroutines.flow.MutableSharedFlow
import kotlinx.coroutines.flow.SharedFlow
import kotlinx.coroutines.flow.asSharedFlow
import java.util.UUID

class InvalidMoveException(message: String): Exception(message)

abstract class Move

abstract class GameStateManager<out T: Move> {
    val gameId = UUID.randomUUID().toString()
    private val flow = MutableSharedFlow<SseEvent>()

    fun getFlow(): SharedFlow<SseEvent> {
        return flow.asSharedFlow()
    }

    abstract fun playMove(move: @UnsafeVariance T)  // TODO find a better way
}