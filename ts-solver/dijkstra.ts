import PriorityQueue from "priority-queue-typescript";
const PARAMS = require("./examples/small.json");

function dijkstra(maze: number[][]) {
    console.log(maze);

    const pq = new PriorityQueue<number>(1, (a: number, b: number) => a - b);
    pq.add(10);
    pq.add(5);
    pq.add(15);
    pq.add(150);
    pq.add(2);
    console.log(pq.poll());
    console.log(pq.poll());
    console.log(pq.poll());
}

function resize(arr: number[][], w: number, h: number): number[][] {
    return arr.slice(0, h).map((r: number[]) => r.slice(0, w));
}

dijkstra(resize(PARAMS["maze"], PARAMS["width"], PARAMS["height"]));
