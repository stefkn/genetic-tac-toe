# genetic-tac-toe

## genetic algorithms in Rust

### using Rust to implement a genetic algorithm to play Tic Tac Toe

###### It would have been called Naughts and Crosses, but it's a bit wordy, and it doesn't pun as well either

**Genetic algorithms** sound complicated, but they're actually fairly simple theoretically. You create a population of game "players" (agents), and if you can use evolution to make a "survival of the fittest" thing happen, then you'll eventually end up with a population of players that are (in theory) *better at the game than the ones you started with.*

You just need a few things to make this happen:

- A game that is simple enough to be simulated a lot of times, and has a clear win condition.
  - ideally it should be deterministic (i.e. the same moves should always lead to the same outcome, no "luck of the draw" type stuff)
  - ideally it should be turn based (but this isn't strictly necessary as you could [quantize](https://en.wikipedia.org/wiki/Quantization) a continuous game into discrete turns)
  - even more ideally there should be a function that can evaluate how well a player is doing at any given time.
    - Games like Chess, Go, and Tic Tac Toe are therefore good candidates for this.

- A way to represent a player's behavior in the game.
  - This is usually done with a "genome" of some kind, which is an encoding of the player's behavior in the game.
    - This sounds complicated (and it can be depending on the game) as we need a way of encoding the player's reaction to any given state of the game. 
    - For example, in Tic Tac Toe, we could represent a player's behavior as a list of 9 numbers, where each number represents the player's preference for playing in a given square.

- A way to make the players' genomes "evolve"
  - This is the part that makes the whole thing work. We need a way to take two players, and combine their genomes in some way to create a new player.
    - This is usually done by taking two players, and combining their genomes in some way to create a new player.
    - This can be done in a number of ways, but the most common way is to take a random point in the genome, and swap the two players' genomes at that point.
    - This is called "crossover", and it's the main way that genetic algorithms work.
    - There are other ways to make the players evolve, such as "mutation", where you randomly change a player's genome in some way, or "selection", where you only keep the best players in the population.

and that's it! By playing with the way that players' genomes evolve, you can create a population of players that are (in theory) better at the game than the ones you started with.

Note that I am not a subject matter expert in genetic algorithms, and I'm sure there are many ways to improve this implementation. I'm just doing this as a way to learn Rust, and to have some fun with genetic algorithms. If I'm doing this horribly wrong, please let me know! I'm always looking to learn more.
