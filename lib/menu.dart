import 'package:flutter/material.dart';
import 'dart:math';

import 'package:flutter/services.dart';
import 'package:sudoku/game_state.dart';
import 'package:sudoku/game_view.dart';
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
              onPressed: (){
                var sudokuSource = generateWithSize(size: size, rulesSrc: []);
                if (sudokuSource == null) {
                  setState(() {
                    sizeText = "Failed to generate for some reason";
                  });
                } else {
                  inputTextController.clear();
                  GameState.setInstance(GameState(sudokuSource));
                  Navigator.of(context).push(MaterialPageRoute(builder: ((context) => const GameView())));
                }
              },
              child: const Text('Create Sudoku'),
            ),
          ],
        ),
      ),
    );
  }
}
