import init, { PumpkinSmashGame } from "./pkg/pumpkin_smash_game.js";

async function run() {
    await init();
    console.log("WASM Initialized");
    const game = new PumpkinSmashGame(10);
    game.spawn_bubbles();
    window.initGame = () => game;
    window.onTap = (x, y) => game.on_tap(x, y);
    window.getHarvestCoins = () => game.get_harvest_coins();
}

run();
