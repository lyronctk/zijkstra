# Zijkstra

Educational tool. Proving the shortest path through a maze with recursive SNARKs. 

## Overview

Meant for learning more about recent proving systems- [Nova](https://github.com/microsoft/Nova) and [Plonky2](https://github.com/mir-protocol/plonky2) in particular- for recursive SNARKs. Verifying shortest paths found via [Dijsktra's Algorithm](https://www.geeksforgeeks.org/dijkstras-shortest-path-algorithm-greedy-algo-7/). Ideal for focusing on the basic mechanics of these SNARKs since maze traversal 1) is an easy-to-visualize problem that most are familiar with and 2) doesn't involve any more complex operations such as EC math or hashing. 

Why Dijkstra's insead of a simpler DFS? *Because Zijkstra is catchier than Maze Zolver of course.*

## Circuit
Definitions 
  - L_1 := location before stepping
  - L_2 := location after stepping
  - C_1 := cost accrued before stepping
  - C_2 := cost accrued after stepping

PUBLIC INPUTS 
- grid
- L2
- C2
- vk, inner SNARK verification key (?)

PRIVATE INPUTS 
- L1
- C1
- π, proof that there is a path to L1 that costs C1

CHECKS
- L1 -> L2 is a valid transition that costs C2 - C1
- π verifies with inputs (Grid, L1, C1, vk)
