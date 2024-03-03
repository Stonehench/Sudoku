import 'dart:async';

import 'package:flutter/material.dart';
import 'package:sudoku/game_view.dart';
import 'package:sudoku/src/rust/api/simple.dart';

class Cell extends StatefulWidget {
  final String digit;
  final int index;
  final int size;

  const Cell(
    this.digit,
    this.index,
    this.size, {
    super.key,
  });

  @override
  State<StatefulWidget> createState() => _CellState();
}

class _CellState extends State<Cell> {
  String? digit;
  bool isCurrentlyError = false;

  @override
  Widget build(BuildContext context) {
    digit ??= widget.digit;

    return InkWell(
      onTap: () {
        if (digit!.trim() == "0") {
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
                style: widget.size <= 9
                    ? const TextStyle(fontSize: 30)
                    : widget.size <= 16
                        ? const TextStyle(fontSize: 15)
                        : const TextStyle(fontSize: 6))
            // 30 or 9x9, 15 for 16x16 , 6 for anything else (for now at least)
            : const Text(""),
      ),
    );
  }
}
