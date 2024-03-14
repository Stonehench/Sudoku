import 'package:flutter/material.dart';
import 'package:sudoku/board.dart';
import 'package:sudoku/digit_selection.dart';
import 'package:sudoku/game_header.dart';
import 'package:sudoku/tool_bar.dart';
import 'package:sudoku/game_state.dart';

class GameView extends StatelessWidget {
  final Set<String> rules;
  const GameView(this.rules, {super.key});

  @override
  Widget build(BuildContext context) {
    var size = GameState.getInstance().size;

    return Scaffold(
      body: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        crossAxisAlignment: CrossAxisAlignment.center,
        children: [
          GameHeader(rules),
          const Board(),
          const SizedBox(height: 10),
          DigitSelect(size),
          const ToolBar(),
        ],
      ),
    );
  }
}
