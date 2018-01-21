# prisoners_dilemma
Learning Rust using the Iterated Prisoners Dilemma Game https://en.wikipedia.org/wiki/Prisoner%27s_dilemma

The Iterated Prisoners Dilemma game pits different strategies against each other in a sequence of trails in order to find out which ones have the best long term viability. Do cooperative strategies win out against cheating ones?

This example uses a Strategy Trait to encapsulate the different algorithms. Some polymorphism is involved when we collect all the different strategies into a list in order to iterate over the different trials. 

Exercises: library, modules, tests, Trait Objects, Vectors, Iterators, Tuples, Types.

Although some generic strategies are defined in the library, the system should be extensible allowing new strategies to be added. 

## Phase 1 - Simple games
In the simple game we simply play each algorithm against all the others to see which score highest.

## Phase 2 - Game of Life - Success changes the landscape.
In the complex game the list of strategies represents occupants on a 1d or 2d field. 
Higher scores are rewarded with offspring of the same type so we should see pockets of cooperation and cheating.

## Phase 3 - Genetic algorithms
Here we implement some strategies that implement a rule that can be passed on modified to descendents and see if we can get an optimising rule.

