/*
 * Verifies one step of a purported path through a grid. Designed to be used
 * recursively to prove proper traversal of entire path. 
 */
pragma circom 2.1.1;

include "./node_modules/circomlib/circuits/comparators.circom";
include "./node_modules/circomlib/circuits/gates.circom";

include "./utils.circom";

template Main(MAX_HEIGHT, MAX_WIDTH, DIM_BITS){
    var VALID_MOVES[4][2] = [[0, 1], [0, -1], [1, 0], [-1, 0]];

    signal input grid[MAX_HEIGHT][MAX_WIDTH];
    signal input height;
    signal input width;
    signal input loc2[2];
    signal input cost2;

    signal input loc1[2];
    signal input cost1;

    // Accrued cost must be updated correctly
    signal stepCost <== 
        GridSelector(MAX_HEIGHT, MAX_WIDTH)(grid, loc2[0], loc2[1]);
    cost2 === cost1 + stepCost;

    // loc2 must be a valid location on the grid
    InBounds(DIM_BITS)(loc2, height, width);

    // Delta from loc1->loc2 must be at most one step in the cardinal directions
    var delta[2] = [loc2[0] - loc1[0], loc2[1] - loc1[1]];
    signal isValidMove <== PairArrayContains(4)(VALID_MOVES, delta);
    isValidMove === 1;
}

component main { public [ grid, loc2, cost2 ] } = Main(5, 5, 3);
