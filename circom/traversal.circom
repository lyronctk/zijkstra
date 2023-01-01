pragma circom 2.0.3;

include "./node_modules/circomlib/circuits/comparators.circom";
include "./node_modules/circomlib/circuits/gates.circom";

/*
 * Computes sum of input array of length N. Implementation from MACI project. 
 */
template CalculateTotal(N) {   
    signal input arr[N];
    signal output out;

    signal sums[N];
    sums[0] <== arr[0];

    for (var i=1; i < N; i++) {
        sums[i] <== sums[i - 1] + arr[i];
    }

    out <== sums[N - 1];
}

/*
 * Selector for 2d array of size [H x W]. Returns grid value at (r, c).
 */
template GridSelector(H, W) {
    signal input grid[H][W];
    signal input r;
    signal input c; 
    signal output out;

    component rowEq[H][W];
    component colEq[H][W];
    component mask[H][W];
    component total = CalculateTotal(H * W);
    for (var i = 0; i < H; i++) {
        for (var j = 0; j < W; j++) {
            rowEq[i][j] = IsEqual();
            rowEq[i][j].in[0] <== i;
            rowEq[i][j].in[1] <== r;

            colEq[i][j] = IsEqual();
            colEq[i][j].in[0] <== j;
            colEq[i][j].in[1] <== c;

            mask[i][j] = AND();
            mask[i][j].a <== rowEq[i][j].out;
            mask[i][j].b <== colEq[i][j].out;

            total.arr[i * W + j] <== grid[i][j] * mask[i][j].out;
        }
    }
    out <== total.out;
}

/*
 * Verify that an (r, c) coordinate is within some height (h) and width (w). 
 * Values of the bounds must fit in DIM_BITS or under. 
 */
template InBounds(DIM_BITS) {
    signal input coord[2];
    signal input h;
    signal input w;

    component boundR = LessThan(DIM_BITS);
    boundR.in[0] <== coord[0];
    boundR.in[1] <== h;
    boundR.out === 1;

    component boundC = LessThan(DIM_BITS);
    boundC.in[0] <== coord[1];
    boundC.in[1] <== w;
    boundC.out === 1;
}

template Traversal(MAX_HEIGHT, MAX_WIDTH, DIM_BITS){
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
}

component main { public [ grid, loc2, cost2 ] } = Traversal(5, 5, 3);
