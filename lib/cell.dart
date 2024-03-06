import 'dart:async';

import 'package:flutter/material.dart';
import 'package:sudoku/game_state.dart';
import 'package:sudoku/src/rust/api/simple.dart';

class Cell extends StatefulWidget {
  final int? digit;
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
  GameState state = GameState.getInstance();
  bool isCurrentlyError = false;
  int? digit;

  void setErr() {
    setState(() {
      isCurrentlyError = true;
    });
    Timer(const Duration(seconds: 1), () {
      setState(() {
        isCurrentlyError = false;
      });
    });
  }

  void onClick() {
    //Check
    if (digit != null) {
      setErr();
    }

    if (checkLegality(position: widget.index, value: GameState.getInstance().selectedDigit)) {
      setState(() {
        digit = GameState.getInstance().selectedDigit;
      });
    } else {
      setErr();
    }
  }

  @override
  Widget build(BuildContext context) {
    GameState state = GameState.getInstance();
    digit ??= widget.digit;
    return InkWell(
      onTap: onClick,
      child: Container(
        color: isCurrentlyError ? Colors.red : Theme.of(context).highlightColor,
        alignment: Alignment.center,
        child: digit != null
            ? Text(digit.toString(),
                style: state.size <= 9
                    ? const TextStyle(fontSize: 30)
                    : state.size <= 16
                        ? const TextStyle(fontSize: 15)
                        : const TextStyle(fontSize: 6))
            // 30 or 9x9, 15 for 16x16 , 6 for anything else (for now at least)
            : const Text(""),
      ),
    );
  }
}
