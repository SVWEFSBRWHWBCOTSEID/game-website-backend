package com.example

import java.util.UUID

class GameExistsException : Exception("game already exists! + L + ratio + get better + skill issue + bozo")
class GameDoesNotExistException: Exception("game does not exist! + L + ratio + get better + skill issue + bozo")

class GameManager<out T: GameStateManager<Move>>(private val gameFactory: () -> T) {
    private val games = mutableMapOf<UUID, T>()

    fun createGame(): T {
        val newGame = gameFactory()
        games[newGame.gameId] = newGame
        return newGame
    }

    fun getGame(gameId: UUID): T {
        return games[gameId] ?: throw GameDoesNotExistException()
    }
}
