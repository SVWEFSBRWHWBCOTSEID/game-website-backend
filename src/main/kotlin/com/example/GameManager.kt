package com.example

typealias GameId = String

class GameExistsException : Exception("game already exists! + L + ratio + get better + skill issue + bozo")
class GameDoesNotExistException: Exception("game does not exist! + L + ratio + get better + skill issue + bozo")

class GameManager<out T: GameStateManager<Move>>(private val gameFactory: () -> T) {
    private val games = mutableMapOf<GameId, T>()

    fun createGame(): GameId {
        val newGame = gameFactory()
        games[newGame.gameId] = newGame
        return newGame.gameId
    }

    fun getGame(gameId: GameId): T {
        return games[gameId] ?: throw GameDoesNotExistException()
    }

    fun playMove(gameId: GameId, move: Move) {
        return getGame(gameId).playMove(move)
    }
}
