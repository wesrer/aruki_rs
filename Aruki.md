# Aruki

Aruki is a chess like game inspired by many chess variants. 
It was designed to be harder for both humans and computers to play than chess, and thus brute force algorithms should not be viable solutions for a game engine.

## Goal

The Goal of the game is to checkmate your opponent's king, just like in chess (i.e. there should be no legal move left for the opponent where is relived of a check).

## Board

Aruki is played on 12 x 12 board, with alternating black and white squares, just like chess.

This is the starting layout:

TODO: insert starting image

## Pieces

The pieces that each player starts with are:

- 12 pawns
- 2 rooks
- 2 arrows
- 2 lances
- 2 pikes
- 1 golden dragon
- 1 silver dragon
- 1 jester
- 1 king

There are more pieces that can be acquired, either by promoting pawns or by evolving your pieces.

## Capturing

When a piece is captured in Aruki, depending on the piece, it is either completely removed from the game or placed in what is known as Buffer Trays. Each player gets a Buffer Tray that keeps the opponent's pieces that the player has captured throughout the game. Pieces in the buffer tray can be later used to evolve pieces or to promote pawns. 

The following pieces go into Buffer Trays:

1. Pawns
2. Rooks
3. Ministers
4. Pikes
5. Lances
6. Greater Lances
7. Greater Pikes
8. Swords
9. Long Swords
10. Javelins

The following pieces do NOT go into the Buffer Trays:

1. Jester
2. Golden Dragon
3. Silver Dragon

## Promotions

Any pawn or javelin that enters the pawn row of the opponent, i.e. the second to last row on the board viewed from the player's perspective can be promoted into a piece that is the Buffer Tray. Promotions cannot happen if there are no leftover pieces in the Buffer tray. Javelins cannot be promoted to pawns, however, even if there are pawns on the Buffer Tray.

## Evolution

Pieces in the Buffer Tray can be used to evolve the current pieces in the board.

Here are the list of evolutions:

- Pawn + Pawn = Javelin
- Rook + Arrow = Minister
- Pike + Pike = Greater Pike
- Lance + Lance = Greater Lance
- Lance + Pike = Sword
- Sword + Sword = Long Sword
- Greater Pike + Greater Lance = Long Sword

## Movement

### Pawn

Pawns move one square in the forward direction, and attack the squares to the right and left of the square it can move to, just like chess.

### Javelin

A Javelin can move one or two squares forward.
It can still attack both the diagonal squares that it could as if it were a pawn.
On top of that, it can now attack the square directly in front of it.
A Javelin evolves from combining two pawns.

### King

The King can move and attack one square in any direction.

### Lance

The Lance can move and attack one square in four directions - up, down, left, right.

### Pike

The pike can move and attack one square along the diagonals

### Sword

The sword moves very much like the king, one square in each direction.
It's also useful to thing of the sword as having the movements of both the Lance and the Pike combined as it is evolved from those pieces.

### Greater Lance

The Greater Lance moves exactly like the lance, except it cna go both one or two squares in the up, down, left or right direction.
It can be useful to think of it as two lances combining their powers, which is what the greater lance evolves from.

### Greater Pike

The Greater Pike moves exactly like the pike, except it can go both one or two squares along the diagonals.
It can be useful to thing of two pikes combining their powers, which is what the greater pike evolves from.

### Long Sword

The Long Sword moves exactly like a sword, except it can go one or two squares in any direction.
The Long Sword can be evolved from either two swords, or from a greater lance and a greater pike, and it mimics that behavior.

### Rooks

The Rook moves exactly like a chess rook, i.e. it can move along rows and columns.

### Arrows

The Arrows are equivalent to the chess bishops. Each player gets a light colored arrow and a dark colored arrow that can move along diagonals, and only keeps to the squares of it's respective color.

### Ministers

The Minister is equivalent to a chess Queen, and can move along both rows and columns and diagonals.
A Minister can be thought of as the combined power of a Rook and an Arrow, which is also what it evolves from.

### Jesters

The Jester by itself, moves and captures only one square in each direction, like a king or a sword.
However, the Jester can mimic the movement of any piece that it is adjacent to, regardless of the player that the piece belongs to.
That is, if a piece is blocking one of the squares that the jester could usually move to, it gains the movement patterns of that piece.
Thus, the possible moves of the jester at any turn is the combined moves of itself, and all of it's adjacent pieces mimiced projected from the jester's square.

### Golden Dragon

The Golden Dragon also moves and captures by one square in each direction, like the king and the sword.
But the Golden Dragon has the unique property that one time per game, it can wipe out an entire column of pieces (the column it's sittign on at that point in time), including itself if the column doesn't have the friendly king in it.

Therefore, the Golden dragon can check an opponent king across a column regardless of how many pieces stand between them. The only exception to this rule is that if both kings are in the same column as the Golden Dragon, then that is not considered a check.

### Silver Dragon

The Silver Dragon is basically the same as the Golden Dragon except it can wipe out a row instead of a column. All other properties carry over.






