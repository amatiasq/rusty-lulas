import * as wasm from "wasm-game-of-life";

wasm.start();
requestAnimationFrame(update)

function update() {
    wasm.update();
    requestAnimationFrame(update);
}