import 'dart:math';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:sudoku/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(MyApp());
}

class MyApp extends StatelessWidget {
  MyApp({super.key});

  final myController = TextEditingController();

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
                  controller: myController,
                  keyboardType: TextInputType.number,
                  inputFormatters: <TextInputFormatter>[
                    FilteringTextInputFormatter.digitsOnly],
                  decoration: InputDecoration(
                    border: OutlineInputBorder(),
                    labelText: 'Sudoku size',
                  ),
                ),
              ),
              myController.text.isNotEmpty ? Text('This results in ' + pow(int.parse(myController.text),3).toString()) : Text(''),
              // Skal laves til en stateful widget så vidt jeg husker før man kan opdatere UI'en
              TextButton(
                onPressed: () => {
                  print('du har klikket på knappen med værdien ' + myController.text),
                  myController.clear()
                  },
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
