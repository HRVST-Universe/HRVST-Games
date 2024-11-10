```dart
import 'package:flutter/material.dart';
import 'pumpkin_smash_screen.dart';

void main() {
  runApp(PumpkinSmashApp());
}

class PumpkinSmashApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Pumpkin Smash',
      theme: ThemeData(
        primarySwatch: Colors.orange,
      ),
      home: PumpkinSmashScreen(),
    );
  }
}
```
