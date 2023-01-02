pragma circom 2.0.3;

include "./node_modules/circomlib/circuits/comparators.circom";
include "./node_modules/circomlib/circuits/gates.circom";

include "./traversal_utils.circom";

template Traversal(MAX_HEIGHT, MAX_WIDTH, DIM_BITS){
    var VALID_MOVES[4][2] = [[0, 1], [0, -1], [1, 0], [-1, 0]];

    signal input grid[MAX_HEIGHT][MAX_WIDTH];
    signal input height;
    signal input width;
    signal input loc2[2];
    signal input cost2;

    signal input loc1[2];
    signal input cost1;

    // Accrued cost must be updated correctly
    component stepCost = GridSelector(MAX_HEIGHT, MAX_WIDTH);
    stepCost.grid <== grid;
    stepCost.r <== loc2[0];
    stepCost.c <== loc2[1];
    cost2 === cost1 + stepCost.out;

    // loc2 must be a valid location on the grid
    component inBounds = InBounds(DIM_BITS);
    inBounds.coord <== loc2;
    inBounds.h <== height;
    inBounds.w <== width;

    // Delta b/w loc1 to loc2 must at mostone step in the cardinal directions
    var delta = [loc2[0] - loc1[0], loc2[1] - loc1[1]];
    signal isValidMove <== PairArrayContains(4)(VALID_MOVES, delta);
    isValidMove === 1;
}

component main { public [ grid, loc2, cost2 ] } = Traversal(5, 5, 3);
