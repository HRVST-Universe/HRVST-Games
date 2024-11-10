import * as wasm from "../pkg";

// Initialize the game from the WASM module
let game;
try {
    game = wasm.init_game();
    console.log("Game initialized");
} catch (e) {
    console.error("Error initializing game: ", e);
}

// Handle click events on the document
function onTap(event) {
    if (!game) {
        console.error("Game is not initialized");
        return;
    }
    let x = event.clientX; // Get x-coordinate of the click
    let y = event.clientY; // Get y-coordinate of the click
    console.log(`Click detected at x=${x}, y=${y}`);
    try {
        game.on_tap(x, y); // Call the WASM function to handle the tap
        console.log("Harvest Coins: " + game.get_harvest_coins()); // Log the current harvest coins
    } catch (e) {
        console.error("Error handling tap: ", e);
    }
}

document.addEventListener("click", onTap); // Attach the onTap function to click events
