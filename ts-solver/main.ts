import PriorityQueue from "priority-queue-typescript";
import fs from "fs";

const IN_LABEL = "medium";
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

function inBounds(
    r: number,
    c: number,
    height: number,
    width: number
): boolean {
    return r >= 0 && c >= 0 && r < height && c < width;
}

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
        const candR: number = cand.path[cand.path.length - 1].row;
        const candC: number = cand.path[cand.path.length - 1].col;
        if (candR === height - 1 && candC === width - 1)
            minPath = JSON.parse(JSON.stringify(cand.path));

        MOVES.forEach((move: number[]) => {
            const stepR: number = candR + move[0];
            const stepC: number = candC + move[1];
            if (!inBounds(stepR, stepC, height, width)) return;

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

function main() {
    const sol: Loc[] = dijkstra(
        PARAMS["maze"],
        PARAMS["height"],
        PARAMS["width"]
    );


    let out: { maze: number[][]; solution: Array<[[number, number], number]> } =
        {
            maze: PARAMS["maze"],
            solution: [],
        };
    sol.forEach((l: Loc, idx: number) => {
        const accumulated: number = idx !== 0 ? out.solution[idx - 1][1] : 0;
        out.solution.push([
            [l.row, l.col],
            accumulated + out.maze[l.row][l.col],
        ]);
    });
    fs.writeFile(OUT_F, JSON.stringify(out, null, 4), "utf-8", () =>
        console.log(`Done! Solution written to: ${OUT_F}`)
    );
}

main();
