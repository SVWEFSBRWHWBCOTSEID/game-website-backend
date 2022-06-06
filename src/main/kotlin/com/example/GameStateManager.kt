package com.example

import com.example.plugins.SseEvent
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.serialization.KSerializer
import kotlinx.serialization.descriptors.PrimitiveKind
import kotlinx.serialization.descriptors.PrimitiveSerialDescriptor
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import java.util.*

class GameFullException : Exception("game has no spots left for new players")
class InvalidMoveException(message: String) : Exception(message)

object UUIDSerializer : KSerializer<UUID> {
    override val descriptor: SerialDescriptor = PrimitiveSerialDescriptor("UUID", PrimitiveKind.STRING)
    override fun serialize(encoder: Encoder, value: UUID) = encoder.encodeString(value.toString())
    override fun deserialize(decoder: Decoder): UUID = UUID.fromString(decoder.decodeString())
}

abstract class Move

// TODO make sure the game has enough players before starting
abstract class GameStateManager<out T : Move> {
    val gameId: UUID = UUID.randomUUID()
    protected val flow = MutableStateFlow(SseEvent("game has not started"))  // TODO figure out better initial val

    fun getFlow(): StateFlow<SseEvent> {
        return flow.asStateFlow()
    }

    abstract fun canStart(): Boolean

    abstract fun playMove(move: @UnsafeVariance T)  // TODO find a better way

    abstract fun addPlayer(): UUID
}