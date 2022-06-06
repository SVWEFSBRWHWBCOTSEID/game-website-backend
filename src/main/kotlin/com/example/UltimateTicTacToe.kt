package com.example

import com.example.plugins.SseEvent
import kotlinx.serialization.Serializable
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import java.util.*


// there are 9 boards with 9 tiles each
@Serializable
data class UltimateTicTacToeMove(
    @Serializable(with = UUIDSerializer::class)
    val playerId: UUID, val board: Int,
    val tile: Int, val symbol: String) : Move()

// use String instead of Char since empty string isn't a Char and JS doesn't have Char
private const val EMPTY = ""
private const val X = "✕"
private const val O = "◯"

private const val ANY_BOARD = -1

class UltimateTicTacToe : GameStateManager<UltimateTicTacToeMove>() {
    private val board = Array(9) { TicTacToe(independentGame = false) }
    private var lastMove = EMPTY
    private var activeBoard = 4  // first move must go in middle board
    private var status = TicTacToeStatus.PLAYING
    private val playerIds = mutableMapOf<Symbol, UUID>()

    override fun addPlayer(): UUID {
        val playerId = UUID.randomUUID()
        if (!playerIds.containsKey(X)) {
            playerIds[X] = playerId
            return playerId
        }
        if (!playerIds.containsKey(O)) {
            playerIds[O] = playerId
            return playerId
        }

        throw GameFullException()
    }

    override fun canStart(): Boolean {
        return playerIds.containsKey(X) && playerIds.containsKey(O)
    }

    override fun playMove(move: UltimateTicTacToeMove) {
        if (!canStart())
            throw InvalidMoveException("game is not ready to start")
        if (!(0 until 9).contains(move.board))
            throw InvalidMoveException("board is not between 0 and 8")
        if (activeBoard != ANY_BOARD && activeBoard != move.board)
            throw InvalidMoveException("board is not the active board ($activeBoard)")
        if (move.symbol != X && move.symbol != O)
            throw InvalidMoveException("symbol is not $X or $O")
        if (board[move.board].status != TicTacToeStatus.PLAYING)
            throw InvalidMoveException("board ${move.board} is no longer active")
        if (move.symbol == lastMove)
            throw InvalidMoveException("$lastMove made a move last turn")
        if (status != TicTacToeStatus.PLAYING)
            throw InvalidMoveException("game is already over")
        if (playerIds[move.symbol] != move.playerId)
            throw InvalidMoveException("cannot move for other player")

        // check for tile selection validity is done in TicTacToe.playMove
        board[move.board].playMove(TicTacToeMove(move.playerId, move.tile, move.symbol))
        lastMove = move.symbol

        if (checkRow(move.tile) || checkCol(move.tile) || checkDiag(move.tile)) {
            status = if (move.symbol == X) TicTacToeStatus.X_VICTORY else TicTacToeStatus.O_VICTORY
        } else if (TicTacToeStatus.PLAYING !in board.map { it.status }) {
            status = TicTacToeStatus.TIED
        }

        flow.tryEmit(SseEvent(Json.encodeToString(board)))
    }

    private fun checkRow(tile: Int): Boolean {
        val row = tile / 3  // get the row number
        return check(board[3 * row], board[3 * row + 1], board[3 * row + 2])
    }

    private fun checkCol(tile: Int): Boolean {
        val col = tile % 3
        return check(board[col], board[col + 3], board[col + 6])
    }

    private fun checkDiag(tile: Int): Boolean {
        // 0 1 2
        // 3 4 5
        // 6 7 8
        // main diagonal is all multiples of 4
        if (tile % 4 == 0 && check(board[0], board[4], board[8])) return true

        // minor diagonal is 2, 4, 6
        if (tile in listOf(2, 4, 6) && check(board[2], board[4], board[6])) return true

        return false
    }

    private fun check(a: TicTacToe, b: TicTacToe, c: TicTacToe): Boolean {
        return a.status == b.status && b.status == c.status
    }
}
