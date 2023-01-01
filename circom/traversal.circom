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

/*
 * Selector for 2d array of size (h x w). Returns grid value at [r, c].
 */
template GridSelector(h, w) {
    signal input grid[h][w];
    signal input r;
    signal input c; 
    signal output out;

    component rowEq[h][w];
    component colEq[h][w];
    component mask[h][w];
    component total = CalculateTotal(h * w);
    for (var i = 0; i < h; i++) {
        for (var j = 0; j < w; j++) {
            rowEq[i][j] = IsEqual();
            rowEq[i][j].in[0] <== i;
            rowEq[i][j].in[1] <== r;

            colEq[i][j] = IsEqual();
            colEq[i][j].in[0] <== j;
            colEq[i][j].in[1] <== c;

            mask[i][j] = AND();
            mask[i][j].a <== rowEq[i][j].out;
            mask[i][j].b <== colEq[i][j].out;

            total.nums[i * w + j] <== grid[i][j] * mask[i][j].out;
        }
    }
    out <== total.sum;
}

template Traversal(GRID_HEIGHT, GRID_WIDTH){
    signal input grid[GRID_HEIGHT][GRID_WIDTH];
    signal input loc2[2];
    signal input cost2;

    signal input loc1[2];
    signal input cost1;

    // Check that accrued cost is updated correctly
    component stepCost = GridSelector(GRID_HEIGHT, GRID_WIDTH);
    stepCost.grid <== grid;
    stepCost.r <== loc2[0];
    stepCost.c <== loc2[1];
    cost2 === cost1 + stepCost.out;
}

component main { public [ grid, loc2, cost2 ] } = Traversal(5, 5);
