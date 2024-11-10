import 'package:flutter/material.dart';
import 'dart:js' as js;

class PumpkinSmashScreen extends StatefulWidget {
  @override
  _PumpkinSmashScreenState createState() => _PumpkinSmashScreenState();
}

class _PumpkinSmashScreenState extends State<PumpkinSmashScreen> {
  int harvestCoins = 0;

  @override
  void initState() {
    super.initState();
    try {
      js.context.callMethod('initGame'); // Initialize the game from Rust/WASM
      print("Game initialized");
    } catch (e) {
      print("Error initializing game: $e");
    }
  }

  // Handles tap events on the screen
  void _onTap(TapDownDetails details) {
    if (!js.context.hasProperty('onTap')) { // Verify if the game instance is correctly initialized
      print("Game is not initialized");
      return;
    }
    final double x = details.localPosition.dx; // Get the x-coordinate of the tap
    final double y = details.localPosition.dy; // Get the y-coordinate of the tap
    print("Screen tapped at: x=$x, y=$y");
    try {
      String updatedBubbles = js.context.callMethod('onTap', [x, y]); // Call the WASM function to process the tap
      print("Updated bubbles: $updatedBubbles");
      setState(() {
        harvestCoins = js.context.callMethod('getHarvestCoins'); // Update the harvest coin balance
        print("Harvest coins updated: $harvestCoins");
      });
    } catch (e) {
      print("Error handling tap: $e");
    }
  }

  @override
  Widget build(BuildContext context) {
    print("Building PumpkinSmashScreen widget");
    return Scaffold(
      appBar: AppBar(
        title: Text('Pumpkin Smash'),
        actions: [
          Padding(
            padding: const EdgeInsets.all(8.0),
            child: Center(
              child: Text('HRVST: $harvestCoins', style: TextStyle(fontSize: 18)), // Display the current HRVST coin balance
            ),
          ),
        ],
      ),
      body: GestureDetector(
        onTapDown: _onTap, // Detect tap events to pop bubbles
        child: Container(
          color: Colors.white,
          child: Center(
            child: Text(
              'Tap to Smash Pumpkins!',
              style: TextStyle(fontSize: 24),
            ),
          ),
        ),
      ),
    );
  }
}
```
