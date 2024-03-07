import 'package:flutter/material.dart';
import 'dart:math';

import 'package:flutter/services.dart';
import 'package:sudoku/gameloader.dart';
import 'package:sudoku/src/rust/api/simple.dart';

class Menu extends StatefulWidget {
  const Menu({super.key});

  @override
  State<Menu> createState() => _MenuState();
}

class _MenuState extends State<Menu> {
  String sizeText = "9x9";
  int size = 9;
  final inputTextController = TextEditingController();

  void onTextChange(String newText) {
    int? newSize = int.tryParse(newText);
    if (newSize != null) {
      if (newSize == 0) {
        setState(() {
          sizeText = "Cannot create size 0";
        });
      } else {
        int sqrtSize = sqrt(newSize).toInt();
        String rounding;
        size = sqrtSize * sqrtSize;
        if (sqrtSize * sqrtSize != newSize) {
          rounding = " (Rounding down to $size)";
        } else {
          rounding = "";
        }
        setState(() {
          sizeText = "${size}x$size$rounding";
        });
      }
    }
  }

  Set<String> gameModes = {};

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
                onChanged: onTextChange,
                controller: inputTextController,
                keyboardType: TextInputType.number,
                inputFormatters: <TextInputFormatter>[
                  FilteringTextInputFormatter.digitsOnly
                ],
                decoration: const InputDecoration(
                  border: OutlineInputBorder(),
                  labelText: 'Sudoku size',
                ),
              ),
            ),
            Text(sizeText),
            TextButton(
              onPressed: () {
                Future<String?> sudokuSource =
                    generateWithSize(size: size, rulesSrc: []);
                inputTextController.clear();
                Navigator.of(context).push(MaterialPageRoute(
                  builder: (context) => GameLoader(sudokuSource),
                ));
              },
              child: const Text('Create Sudoku'),
            ),
            Row(
              children: [
                Text("Knights move"),
                Checkbox(
                    value: gameModes.contains("KnightsMove"),
                    onChanged: (v) {
                      setState(() {
                        if (v == true) {
                          gameModes.add("KnightsMove");
                        } else {
                          gameModes.remove("KnightsMove");
                        }
                      });
                    })
              ],
            )
          ],
        ),
      ),
    );
  }
}
