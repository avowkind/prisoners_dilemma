# Iterated Prisoners Dilemma Simulatation with spatial effects.

Learning activity for the Rust Language. We want to use parallelism and minimal data storage so that we can run very large numbers of players and iterations. We also want strategies to be easily programmable. - e.g a trait.

The prisoners dilemma is based on the idea of two prisoners kepts in separate cells being interrogated. If either confesses and implicates the other he is given a light sentance while the other is executed. If both stay quiet then both get off free but if both talk then they are both executed. Being executed precludes playing the game again so the simulation is modified to gain or lose points.

This is represented by a scoring matrix 0 = dead, 100 = free

| P1 | Talks  | Silent |
-----------|--------|--------|--------
P2         | Talks  | 0,0    | 0, 20
P2         | Silent | 20, 0  | 100,100

This game tests out various strategies for engaging with the prisoners dilemma.
A range of strategies are generated e.g tit for tat, tit for two tats, always cheat etc.
Individual rounds are played between two players each given a random strategy. A strategy consists of ones choice of first move and then subsequent moves based on the previous result.
e.g In tit for tat the player initially stays silent but if the other player talks then on the next round they will switch.

Having players play multiple rounds favour collusion and loyalty but the only form of communication available is each previous move.

There are three ways we can select players
1. Comprehensive Competition. - Each strategy plays every other strategy a few times.
2. Population Competition - Each strategy plays a random other strategy. The number of players of each type is randomised to provide a population of potential encounters.
3. Spatial competition - Each play occupies a location on a line or in 2d space, they compete with nearest neighbours. New players are generated from winners.

## Main logic
1. create or locate 2 players
2. obtain two moves
3. score the moves and accumulate result
4. inform the players of their opponents move
5. Iterate
6. stop after n rounds or some other criterion.
7. decide on a winner, tabulate results by strategy, or strategy type.
8. repeat with new players.

## Data Structures
- enum moves
- Score matrix
- game board - list or array of players

## Traits
- Player - generates a sequence of moves.

## function
- round

## Questions
- Are strategies and players separate? players are instances of strategies. Instances are required if the player needs a memory which it does to know its own previous moves and that of the opponent.
- How much memory? some simple strategies need only a couple of values so are fixed in size, others may hold a list of previous moves and so be variable size.
- What
