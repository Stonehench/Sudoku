import 'package:flutter/material.dart';
import 'package:sudoku/board.dart';
import 'package:sudoku/digit_selection.dart';
import 'package:sudoku/tool_bar.dart';

class GameView extends StatelessWidget {
  final Object? size;
  const GameView(this.size, {super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        crossAxisAlignment: CrossAxisAlignment.center,
        children: [
          Board(size),
          const SizedBox(height: 10),
          DigitSelect(size),
          const ToolBar(),
        ],
      ),
    );
  }
}

class GameState {
  static int selectedDigit = 1;
}
