# tic-tac-toe

Rules for tic-tac-toe:

* All state of the game should live on-chain. State includes open games, games currently in progress and completed games.
* Any user can submit a transaction to the network to invite others to start a game (i.e. create an open game).
* Other users may submit transactions to accept invitations. When an invitation is accepted, the game starts.
* The roles of “X” and “O” are decided as follows. The user's public keys are concatenated and the result is hashed. If the first bit of the output is 0, then the game's initiator (whoever posted the invitation) plays "O" and the second player plays "X" and vice versa. “X” has the first move.
* Both users submit transactions to the network to make their moves until the game is complete.
* The game needs to support multiple concurrent games sessions/players.
