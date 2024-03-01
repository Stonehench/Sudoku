import 'package:flutter/material.dart';
import 'dart:math';

import 'package:flutter/services.dart';

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
        size = newSize;
        int sqrtSize = sqrt(size).toInt();
        String rounding;
        if (sqrtSize * sqrtSize != size) {
          rounding = " (Rounding down to ${sqrtSize * sqrtSize})";
        } else {
          rounding = "";
        }
        setState(() {
          sizeText = "${sqrtSize}x$sqrtSize$rounding";
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
              onPressed: () => {
                Navigator.of(context).pushNamed('/board'),
                inputTextController.clear()
              },
              child: const Text('Create Sudoku'),
            ),
          ],
        ),
      ),
    );
  }
}
