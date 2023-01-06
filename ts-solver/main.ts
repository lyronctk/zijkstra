/*
 * Finds the minimum cost solution to traverse a maze with Dijkstra's Algorithm.
 * Start is set to the top-left corner of the grid and end to the bottom-right
 * corner. The value at each cell denotes the cost of stepping onto it. In other
 * words, if the grid were represented as a graph, then each node would have
 * four edges with identical weights set to the corresponding cell value.
 */
import PriorityQueue from "priority-queue-typescript";
import fs from "fs";

// Label of .json parameters in examples/
const IN_LABEL = "large";

// Writes solution to this file
const OUT_F = `./solutions/${IN_LABEL}.soln.json`;

const PARAMS = require(`./examples/${IN_LABEL}.json`);
const MOVES: number[][] = [
    [-1, 0],
    [1, 0],
    [0, -1],
    [0, 1],
];
const INF: number = Number.MAX_SAFE_INTEGER;

type Loc = {
    row: number;
    col: number;
};

class Candidate {
    cost: number;
    path: Loc[];

    /*
     * A candidate path to consider in the search. Accumulated cost walk the
     * path is attached.
     */
    constructor(prefixPath: Loc[], stepCostTotal: number, stepLoc: Loc) {
        this.path = JSON.parse(JSON.stringify(prefixPath));
        this.path.push(stepLoc);
        this.cost = stepCostTotal;
    }
}

/*
 * Instatiate a 2D array of given size with a set value.
 */
function init2DArr(height: number, width: number, fillVal: number): number[][] {
    return new Array(height)
        .fill(null)
        .map(() => new Array(width).fill(fillVal));
}

/*
 * Check if a given location is within the bounds of the grid.
 */
function inBounds(l: Loc, height: number, width: number): boolean {
    return l.row >= 0 && l.col >= 0 && l.row < height && l.col < width;
}

/*
 * Run Dijkstra's to find the lowest cost path from the top left to the bottom
 * right of the grid. Moves only along the cardinal directions.
 */
function dijkstra(maze: number[][], height: number, width: number): Loc[] {
    if (height === 0 || width === 0)
        throw new Error("Maze must have nonzero dimensions.");

    let minCosts = init2DArr(height, width, INF);
    let minPath: Loc[] = [];

    let pq = new PriorityQueue<Candidate>(
        1,
        (a: Candidate, b: Candidate) => a.cost - b.cost
    );
    pq.add(new Candidate([], maze[0][0], { row: 0, col: 0 }));
    while (!pq.empty()) {
        const cand: Candidate = pq.poll()!;
        const l: Loc = cand.path[cand.path.length - 1];
        if (l.row === height - 1 && l.col === width - 1)
            minPath = JSON.parse(JSON.stringify(cand.path));

        MOVES.forEach((move: number[]) => {
            const stepL: Loc = { row: l.row + move[0], col: l.col + move[1] };
            if (!inBounds(stepL, height, width)) return;

            const stepAccum: number = cand.cost + maze[stepL.row][stepL.col];
            if (stepAccum < minCosts[stepL.row][stepL.col]) {
                minCosts[stepL.row][stepL.col] = stepAccum;
                pq.add(new Candidate(cand.path, stepAccum, stepL));
            }
        });
    }

    return minPath;
}

/*
 * Write the lowest cost path to a json file. This file stores the maze, along
 * with an enumeration of the path that has the accumulated cost attached to
 * each step.
 */
function writeSolution(mz: number[][], h: number, w: number, sol: Loc[]) {
    let out: {
        maze: number[][];
        height: number;
        width: number;
        solution: Array<[number, number]>;
    } = {
        maze: mz,
        height: h,
        width: w,
        solution: sol.map((l: Loc) => [l.row, l.col]),
    };
    fs.writeFile(OUT_F, JSON.stringify(out), "utf-8", () =>
        console.log(`Done! Solution written to: ${OUT_F}`)
    );
}

function main() {
    const sol: Loc[] = dijkstra(
        PARAMS["maze"],
        PARAMS["height"],
        PARAMS["width"]
    );
    writeSolution(PARAMS["maze"], PARAMS["height"], PARAMS["width"], sol);
}

main();
