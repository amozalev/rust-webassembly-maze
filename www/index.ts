import init, {Maze} from "rust-webassembly-maze";

init().then(_ => {
    const maze = Maze.new();
    console.log('==maze width:', maze.width);
})