import 'package:flutter/material.dart';
import 'package:sudoku/game_state.dart';
import 'package:sudoku/menu.dart';

class GameHeader extends StatefulWidget {
  String rules;
  GameHeader(this.rules);
  @override
  State<GameHeader> createState() => _DigitSelectState();
}

class _DigitSelectState extends State<GameHeader> {
  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        TextButton(
            onPressed: () {
              showDialog(
                context: context,
                builder: (context) => AlertDialog(
                  title: Text("Rules"),
                  content: Text(widget.rules),
                ),
              );
            },
            child: Text("Rules")),
        Text("Standard Sudoku", style: TextStyle(fontSize: 24)),
      ],
    );
  }
}
