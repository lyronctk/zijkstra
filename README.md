# Zijkstra

Educational tool. Proving the shortest path through a maze with recursive SNARKs. 

NOTE: Updating the README after wrapping up another project this week. If you want to look around before then, all the magic happens in `nova/src/main.rs` and `nova/circom/traversal.circom`. Hopefully wrote everything so it's easy to follow. Also note that if you clone this repo to run, you have to clone my fork of `nova-snark` and `nova-scotia` in the parent directory (mainly made a few structs public to dissect).

## Motivation

Meant for learning more about recent proving systems- [Nova](https://github.com/microsoft/Nova) and [Plonky2](https://github.com/mir-protocol/plonky2) in particular- for recursive SNARKs. Verifying shortest paths found via Dijsktra's Algorithm. Ideal for focusing on the basic SNARK mechanics since maze traversal is a familiar problem that doesn't involve many constraint-heavy operations. 

Why Dijkstra's insead of a simpler DFS? *Because Zijkstra is catchier than Maze Zolver of course.*

## Step Circuit
PUBLIC INPUTS 
1. *H*(*grid*): Hash of the grid / maze
1. L<sub>1</sub>: Location before stepping
1. C<sub>1</sub>: Cost accrued to get to the location 

PUBLIC OUTPUTS (symmetric, same as public inputs)
1. *H*(*grid*): Hash of the same grid / maze
1. L<sub>2</sub>: Location after stepping
1. C<sub>2</sub>: Cost accrued + additional cost for moving to location 

PRIVATE INPUTS 
1. *A*: Grid that is traversed
1. *m*: Move for this step [*dRow*, *dCol*]

LOGIC
1. Checks that the proposed move is valid 
1. Updates location and accrued cost 
