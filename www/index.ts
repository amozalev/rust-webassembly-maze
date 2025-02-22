import init, {Maze, BinaryTree} from "rust-webassembly-maze";

enum Border {
    North = 0b0001,
    East = 0b0010,
    South = 0b0100,
    West = 0b1000,
}

init().then(wasm => {
    const WORLD_WIDTH = 20;
    const CELL_SIZE = 20;

    const maze = Maze.new(WORLD_WIDTH);
    BinaryTree.generate_maze(maze);

    const canvas = <HTMLCanvasElement>document.getElementById("maze");
    canvas.height = WORLD_WIDTH * CELL_SIZE;
    canvas.width = WORLD_WIDTH * CELL_SIZE;

    const ctx = canvas.getContext("2d");

    const drawGrid = () => {
        ctx.beginPath();

        const mazePtr = maze.get_maze();
        const mazeCells = new Uint32Array(wasm.memory.buffer, mazePtr, maze.size);

        mazeCells.forEach((cellBorder, ind) => {
            const x = ind % WORLD_WIDTH * CELL_SIZE;
            const y = Math.floor(ind / WORLD_WIDTH) * CELL_SIZE;

            if (~cellBorder & Border.North) {
                ctx.moveTo(x, y);
                ctx.lineTo(x + CELL_SIZE, y);
            }
            if (~cellBorder & Border.South) {
                ctx.moveTo(x, y + CELL_SIZE);
                ctx.lineTo(x + CELL_SIZE, y + CELL_SIZE);
            }
            if (~cellBorder & Border.West) {
                ctx.moveTo(x, y);
                ctx.lineTo(x, y + CELL_SIZE);
            }
            if (~cellBorder & Border.East) {
                ctx.moveTo(x + CELL_SIZE, y);
                ctx.lineTo(x + CELL_SIZE, y + CELL_SIZE);
            }

        });

        ctx.stroke();
    }

    const paint = () => {
        drawGrid();
    }

    paint();
})