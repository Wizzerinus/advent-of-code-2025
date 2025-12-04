import * as fs from "node:fs";

const content = fs.readFileSync("input4.txt", { encoding: "utf-8" }).split("\n").filter(x => x).map(x => x.split(""));
const getAround = (x, y) => [1, 0, -1]
        .flatMap(t => [1, 0, -1].map(s => ({t, s})))
        .filter(({t, s}) => t !== 0 || s !== 0)
        .map(({t, s}) => ({x: x + t, y: y + s}))
        .map(({x, y}) => x >= 0 && x < content.length && y >= 0 && y < content[0].length ? content[x][y] : ".")
        .filter(x => x === "@");
const getForkliftable = () => content
        .flatMap((s, x) => s.map((c, y) => ({c, x, y})))
        .filter(x => x.c === "@" && getAround(x.x, x.y).length < 4);

const task1 = getForkliftable().length;
console.log(`Task 1: ${task1}`);

let total = 0, oldTotal = 0;
do {
    oldTotal = total;
    const fl = getForkliftable();
    fl.forEach(({x, y}) => content[x][y] = ".");
    total += fl.length;
} while (total > oldTotal);
console.log(`Task 2: ${total}`);

