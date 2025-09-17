# Rock Paper Scissors

Runs a simulation with 5 different strategies duking it out over 1000 matches.

## Running

Go to the root directory and run

    $ cargo run

## Strategies

 1. Player 1 is completely random
 2. Player 2 is heavily weighted to scissors and otherwise random
 3. Player 3 will choose the most common move in their opponents losing record
 4. Player 4 will cycle going rock-paper-scissors ad nauseum
 5. Player 5 will copy their opponents most recent successful move

**Note**: For bounding performance reasons, player 3 will only search the most recent 100 moves. This
has almost zero impact on results at 1000 matches and there is no significant observed difference in
results at even up to 1,000,000 matches but it changes complexity from exponential to bounded linear.