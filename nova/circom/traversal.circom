/*
 * Verifies one step of a purported path through a grid. Designed to be used
 * recursively to prove proper traversal of entire path. 
 */
pragma circom 2.1.1;

include "./node_modules/circomlib/circuits/comparators.circom";
include "./node_modules/circomlib/circuits/gates.circom";
include "./node_modules/circomlib/circuits/poseidon.circom";

include "./utils.circom";

/* Input / output signals 
 *
 *   Recursive signals step_in and step_out store the following elements. 
 *   Note that though these signals are public in reference to this step 
 *   circuit, the final succinct proof for the entire path only reveals the 
 *   first step_in and the last step_out. 
 *     - Index [0]: H_poseidon(grid || height || width), identical for in & out
 *     - Index [1]: Row value of location, before & after the step 
 *     - Index [2]: Col value of location, before & after the step
 *     - Index [3]: Cost accured to get to the location, before & after the step 
 *
 *   Remaining signals are private:
 *     - grid: 2D array that is traversed, padded with 0s to fill max [H x W]
 *     - height: Bounded height for the grid considered in this traversal
 *     - width: Bounded width for the grid considered in this traversal
 *     - move: Direction of this step [dR, dC]
 * 
 */
template Main(MAX_HEIGHT, MAX_WIDTH, DIM_BITS){
    var VALID_MOVES[4][2] = [[0, 1], [0, -1], [1, 0], [-1, 0]];

    signal input step_in[4];
    signal output step_out[4];

    signal input grid[MAX_HEIGHT][MAX_WIDTH];
    signal input height;
    signal input width; 
    signal input move[2];

    // NOTE: This constraint is temporarily deactivated 
    // Bounded grid must match hash in step_in / step_out
    // signal gridHash <== GridHash(MAX_HEIGHT, MAX_WIDTH)(grid);
    // signal boundedGridHash <== Poseidon(3)([gridHash, height, width]);
    // step_in[0] === boundedGridHash;
    step_out[0] <== step_in[0];

    // Proposed move must be one step in the cardinal directions
    signal isValidMove <== PairArrayContains(4)(VALID_MOVES, move);
    isValidMove === 1;

    // Update location and verify that it is in bounds 
    step_out[1] <== step_in[1] + move[0];
    step_out[2] <== step_in[2] + move[1];
    InBounds(DIM_BITS)([step_out[1], step_out[2]], height, width);

    // Update accrued cost 
    signal stepCost <== 
        GridSelector(MAX_HEIGHT, MAX_WIDTH)(grid, step_out[1], step_out[2]);
    step_out[3] <== step_in[3] + stepCost; 

    // log("- Constraints satisfied -");
}

component main { public [ step_in ] } = Main(5, 5, 3);
