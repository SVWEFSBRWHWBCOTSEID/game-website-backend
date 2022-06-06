package com.example

enum class TicTacToeStatus {
    PLAYING, TIED, X_VICTORY, O_VICTORY
}

data class TicTacToeMove(val tile: Int, val symbol: String): Move()

// use String instead of Char since empty string isn't a Char and JS doesn't have Char
private const val EMPTY = ""
private const val X = "✕"
private const val O = "◯"

class TicTacToe(private val allowDoubleMove: Boolean = false) : GameStateManager<TicTacToeMove>() {
    private val board = Array(9) { EMPTY }
    private var lastMove = EMPTY
    var status = TicTacToeStatus.PLAYING
    override fun playMove(move: TicTacToeMove) {
        if (!(0 until 9).contains(move.tile)) {
            throw InvalidMoveException("tile is not between 0 and 8")
        }
        if (move.symbol != X && move.symbol != O) {
            throw InvalidMoveException("symbol is not $X or $O")
        }
        if (board[move.tile] != EMPTY) {
            throw InvalidMoveException("tile ${move.tile} is already occupied")
        }
        if (!allowDoubleMove && move.symbol == lastMove) {
            throw InvalidMoveException("$lastMove made a move last turn")
        }
        if (status != TicTacToeStatus.PLAYING) {
            throw InvalidMoveException("game is already over")
        }

        board[move.tile] = move.symbol
        lastMove = move.symbol

        if (checkRow(move.tile) || checkCol(move.tile) || checkDiag(move.tile)) {
            status = if (move.symbol == X) TicTacToeStatus.X_VICTORY else TicTacToeStatus.O_VICTORY
        } else if (EMPTY !in board) {
            status = TicTacToeStatus.TIED
        }
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
    private fun check(a: String, b: String, c: String): Boolean {
        return a == b && b == c
    }
}
