pragma circom 2.0.3;

include "./node_modules/circomlib/circuits/comparators.circom";
include "./node_modules/circomlib/circuits/gates.circom";

/*
 * Computes sum of input array. Implementation from MACI project. 
 */
template CalculateTotal(n) {   
    signal input nums[n];
    signal output sum;

    signal sums[n];
    sums[0] <== nums[0];

    for (var i=1; i < n; i++) {
        sums[i] <== sums[i - 1] + nums[i];
    }

    sum <== sums[n - 1];
}

template Traversal(GRID_HEIGHT, GRID_WIDTH){
    signal input grid[GRID_HEIGHT][GRID_WIDTH];
    signal input loc2[2];
    signal input cost2;

    signal input loc1[2];
    signal input cost1;

    component GridSelector

    component costDelta = CalculateTotal(GRID_HEIGHT * GRID_WIDTH);
    component rEq[GRID_HEIGHT][GRID_WIDTH];
    component cEq[GRID_HEIGHT][GRID_WIDTH];
    component mask[GRID_HEIGHT][GRID_WIDTH];
    for (var i = 0; i < GRID_HEIGHT; i++) {
        for (var j = 0; j < GRID_WIDTH; j++) {
            rEq[i][j] = IsEqual();
            rEq[i][j].in[0] <== i;
            rEq[i][j].in[1] <== loc2[0];

            cEq[i][j] = IsEqual();
            cEq[i][j].in[0] <== j;
            cEq[i][j].in[1] <== loc2[1];

            mask[i][j] = AND();
            mask[i][j].a <== rEq[i][j].out;
            mask[i][j].b <== cEq[i][j].out;

            costDelta.nums[i * GRID_WIDTH + j] <== grid[i][j] * mask[i][j].out;
        }
    }
    cost2 === cost1 + costDelta.sum;
}

component main { public [ grid, loc2, cost2 ] } = Traversal(5, 5);
