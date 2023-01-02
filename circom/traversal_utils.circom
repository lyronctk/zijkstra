pragma circom 2.1.1;

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
    component total = ArraySum(H * W);
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

/*
 * Computes sum of input array of length N. Implementation from MACI project. 
 * Reference: https://github.com/privacy-scaling-explorations/maci/blob/v1/circuits/circom/trees/calculateTotal.circom
 */
template ArraySum(N) {   
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
 * Checks equality for two input arrays of length N.  
 */
template ArrayEqual(N) {
    signal input arr1[N];
    signal input arr2[N];
    signal output out;

    signal accumulator[N];
    accumulator[0] <== IsEqual()([arr1[0], arr2[0]]);
    for (var i = 1; i < N; i++)
        accumulator[i] <== 
            AND()(accumulator[i - 1], IsEqual()([arr1[i], arr2[i]]));
    out <== accumulator[N-1];
}

/*
 * Checks whether a pair array contains a given pair. A pair is represented as 
 * an array of length 2. Equivalently, checks whether a pair is present in an 
 * array. Implementation inspired by ZKHunt. 
 * Reference: https://github.com/FlynnSC/zk-hunt/blob/40455327102618ba4f8f629e1ae094a5b072a3c1/packages/circuits/src/utils/isEqualToAny.circom
 */
template PairArrayContains(N) {
    signal input arr[N][2];
    signal input pair[2];
    signal output out;

    signal accumulator[N];
    accumulator[0] <== ArrayEqual(2)(arr[0], pair);
    for (var i = 1; i < N; i++) 
        accumulator[i] <== 
            OR()(accumulator[i - 1], ArrayEqual(2)(arr[i], pair));

    out <== accumulator[N - 1];
}
