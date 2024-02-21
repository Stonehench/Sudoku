import 'package:flutter/material.dart';
import 'package:sudoku/src/rust/api/simple.dart';
import 'package:sudoku/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('Sudoku!')),
        body: Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              SizedBox(
                width: 250,
                child: TextField(
                  
                  decoration: InputDecoration(
                    border: OutlineInputBorder(),
                    labelText: 'Sudoku size',
                  ),
                ),
              ),
              TextButton(
                onPressed: () => print('du har klikket på knappen'),
                child: Text(
                  'Create Sudoku'
                ),
              ),
            ],
          ),
          
        ),
      ),
    );
  }
}
