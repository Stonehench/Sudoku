import 'package:flutter/material.dart';
import 'package:sudoku/board.dart';
import 'package:sudoku/digit_selection.dart';

class GameView extends StatefulWidget {
  final Object? size;
  const GameView(this.size, {super.key});

  @override
  State<GameView> createState() => _GameViewState();
}

class _GameViewState extends State<GameView> {
  int selectedDigit = 1;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        crossAxisAlignment: CrossAxisAlignment.center,
        children: [
          Board(
            widget.size,
            getDigit: () => selectedDigit,
          ),
          const SizedBox(height: 10),
          DigitSelect(
            widget.size,
            selectDigit: (newDigit) {
              setState(() {
                selectedDigit = newDigit;
              });
            },
          )
        ],
      ),
    );
  }
}
