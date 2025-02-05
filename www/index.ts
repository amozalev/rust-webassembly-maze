import init, {Maze} from "rust-webassembly-maze";

init().then(_ => {
    const WORLD_WIDTH = 20;
    const CELL_SIZE = 20;

    const maze = Maze.new(WORLD_WIDTH);
    console.log('==maze width:', maze.width);

    const canvas = <HTMLCanvasElement>document.getElementById("maze");
    canvas.height = WORLD_WIDTH * CELL_SIZE;
    canvas.width = WORLD_WIDTH * CELL_SIZE;

    const ctx = canvas.getContext("2d");

    const drawGrid = () => {
        ctx.beginPath();

        for (let i = 0; i <= WORLD_WIDTH; i++) {
            ctx.moveTo(0, i * CELL_SIZE);
            ctx.lineTo(WORLD_WIDTH * CELL_SIZE, i * CELL_SIZE);
        }

        for (let j = 0; j <= WORLD_WIDTH; j++) {
            ctx.moveTo(j * CELL_SIZE, 0);
            ctx.lineTo(j * CELL_SIZE, WORLD_WIDTH * CELL_SIZE);
        }

        ctx.stroke();
    }

    const paint = () => {
        drawGrid();
    }

    paint();
})