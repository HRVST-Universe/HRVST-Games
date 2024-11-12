import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';
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
      // Initialize the game from Rust/WASM
      if (js.context.hasProperty('initGame')) {
        js.context.callMethod('initGame');
        print("Game initialized");
      } else {
        print("initGame is not defined in the JavaScript context");
      }
    } catch (e) {
      print("Error initializing game: $e");
    }
  }

  // Handles tap events on the screen
  void _onTap(TapDownDetails details) {
    print("_onTap called");
    // Verify if the game instance is correctly initialized
    if (!js.context.hasProperty('onTap')) {
      print("Game is not initialized");
      return;
    }
    
    // Get the x and y coordinates of the tap
    final double x = details.localPosition.dx;
    final double y = details.localPosition.dy;
    print("Screen tapped at: x=$x, y=$y");
    
    try {
      // Call the WASM function to process the tap
      print("Calling onTap in WASM");
      if (js.context.hasProperty('onTap')) {
        String updatedBubbles = js.context.callMethod('onTap', [x, y]);
        print("Updated bubbles: $updatedBubbles");
        
        // Update the harvest coin balance
        setState(() {
          print("Updating harvest coins");
          harvestCoins = js.context.callMethod('getHarvestCoins');
          print("Harvest coins updated: $harvestCoins");
        });
      } else {
        print("onTap is not defined in the JavaScript context");
      }
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
              // Display the current HRVST coin balance
              child: Text('HRVST: $harvestCoins', style: TextStyle(fontSize: 18)),
            ),
          ),
        ],
      ),
      body: GestureDetector(
        onTapDown: _onTap, // Detect tap events to pop bubbles
        child: Stack(
          children: [
            // Background SVG graphic
            SvgPicture.asset(
              'assets/background.svg',
              width: MediaQuery.of(context).size.width,
              height: MediaQuery.of(context).size.height,
              fit: BoxFit.cover,
            ),
            // Center text for tapping instructions
            Center(
              child: Column(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  Text(
                    'Tap to Smash Pumpkins!',
                    style: TextStyle(fontSize: 24),
                  ),
                  // Pumpkin SVG as game element or skin
                  SvgPicture.asset(
                    'assets/pumpkin.svg',
                    width: 100,
                    height: 100,
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}
