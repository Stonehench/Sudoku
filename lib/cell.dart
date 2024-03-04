import 'dart:async';

import 'package:flutter/material.dart';
import 'package:sudoku/game_view.dart';
import 'package:sudoku/src/rust/api/simple.dart';

class Cell extends StatefulWidget {
  final String digit;
  final int index;
  final int size;

  Cell(
    this.digit,
    this.index,
    this.size, {
    super.key,
  });

  bool initialClue = true;

  @override
  State<StatefulWidget> createState() => _CellState();
}

class _CellState extends State<Cell> {
  String? digit;
  bool isCurrentlyError = false;

  @override
  Widget build(BuildContext context) {
    digit ??= widget.digit;
    double fontSize = widget.size <= 9
        ? 30.0
        : widget.size <= 16
            ? 15.0
            : 6.0;

    if (digit!.trim() == "0") {
      // if the number in the cell is not initially 0
      // then it must have been a clue provided in the generated sudoku
      widget.initialClue = false;
    }

    return InkWell(
      onTap: () {
        if (digit!.trim() == "0" || !widget.initialClue) {
          bool legal = checkLegality(
              position: widget.index, value: GameState.selectedDigit);
          if (legal) {
            setState(() {
              digit = GameState.selectedDigit.toString();
            });
          } else {
            setState(() {
              isCurrentlyError = true;
              Timer(const Duration(seconds: 1), () {
                setState(() {
                  isCurrentlyError = false;
                });
              });
            });
          }
        } else {
          setState(() {
            isCurrentlyError = true;
            Timer(const Duration(seconds: 1), () {
              setState(() {
                isCurrentlyError = false;
              });
            });
          });
        }
      },
      child: Container(
        color: isCurrentlyError ? Colors.red : Theme.of(context).highlightColor,
        alignment: Alignment.center,
        child: digit != null && digit!.trim() != "0"
            ? Text(digit!,
                style: TextStyle(
                    fontSize: fontSize,
                    color: widget.initialClue ? Colors.white : Colors.black))
            // 30 or 9x9, 15 for 16x16 , 6 for anything else (for now at least)
            : const Text(""),
      ),
    );
  }
}
