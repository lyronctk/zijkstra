import PriorityQueue from "priority-queue-typescript";
const PARAMS = require("./examples/small.json");

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

function dijkstra(maze: number[][]) {
    console.log(maze);

    const pq = new PriorityQueue<Candidate>(
        1,
        (a: Candidate, b: Candidate) => a.cost - b.cost
    );
    
}

function resize(arr: number[][], w: number, h: number): number[][] {
    return arr.slice(0, h).map((r: number[]) => r.slice(0, w));
}

dijkstra(resize(PARAMS["maze"], PARAMS["width"], PARAMS["height"]));
