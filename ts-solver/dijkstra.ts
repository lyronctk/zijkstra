import PriorityQueue from "priority-queue-typescript";
const PARAMS = require("./examples/small.json");

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

    constructor(prefixPath: Loc[], stepCostTotal: number, stepLoc: Loc) {
        this.path = JSON.parse(JSON.stringify(prefixPath));
        this.path.push(stepLoc);
        this.cost = stepCostTotal;
    }
}

function init2DArr(height: number, width: number, fillVal: number): number[][] {
    return new Array(height)
        .fill(null)
        .map(() => new Array(width).fill(fillVal));
}

function inBounds(r: number, c: number, maze: number[][]): boolean {
    return r >= 0 && c >= 0 && r < maze.length && c < maze[0].length;
}

function dijkstra(
    maze: number[][],
    height: number,
    width: number
): Loc[] | null {
    if (height === 0 || width === 0)
        throw new Error("Maze must have nonzero dimensions.");

    let minCosts = init2DArr(height, width, INF);
    let minPath: Loc[] | null = null;

    let pq = new PriorityQueue<Candidate>(
        1,
        (a: Candidate, b: Candidate) => a.cost - b.cost
    );
    pq.add(new Candidate([], maze[0][0], { row: 0, col: 0 }));
    while (!pq.empty()) {
        const cand: Candidate = pq.poll()!;
        const candR: number = cand.path[cand.path.length - 1].row;
        const candC: number = cand.path[cand.path.length - 1].col;
        if (candR === height - 1 && candC === width - 1)
            if (cand.cost < minCosts[height - 1][width - 1]) {
                minPath = JSON.parse(JSON.stringify(cand.path));
            }

        MOVES.forEach((move: number[]) => {
            const stepR: number = candR + move[0];
            const stepC: number = candC + move[1];
            if (!inBounds(stepR, stepC, maze)) return;

            const stepCostTotal: number = cand.cost + maze[stepR][stepC];
            if (stepCostTotal < minCosts[stepR][stepC]) {
                minCosts[stepR][stepC] = stepCostTotal;
                pq.add(
                    new Candidate(cand.path, stepCostTotal, {
                        row: stepR,
                        col: stepC,
                    })
                );
            }
        });
    }

    return minPath;
}

const minPath: Loc[] | null = dijkstra(
    PARAMS["maze"],
    PARAMS["height"],
    PARAMS["with"]
);
console.log(minPath);
