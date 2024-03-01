import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'dart:math';

import 'package:flutter/services.dart';
import 'package:sudoku/src/rust/frb_generated.dart';

class Menu extends StatelessWidget {
  final myController = TextEditingController();

  @override
  Widget build(BuildContext context) {
    return Scaffold(
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
                  FilteringTextInputFormatter.digitsOnly
                ],
                decoration: InputDecoration(
                  border: OutlineInputBorder(),
                  labelText: 'Sudoku size',
                ),
              ),
            ),
            myController.text.isNotEmpty
                ? Text('This results in ' +
                    pow(int.parse(myController.text), 3).toString())
                : Text(''),
            // Skal laves til en stateful widget så vidt jeg husker før man kan opdatere UI'en
            TextButton(
              onPressed: () => {
                Navigator.of(context).pushNamed('/board'),
                print("hej"),
                myController.clear()
              },
              child: Text('Create Sudoku'),
            ),
          ],
        ),
      ),
    );
  }
}
