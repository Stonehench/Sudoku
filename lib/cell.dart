import 'dart:async';

import 'package:flutter/material.dart';
import 'package:sudoku/game_state.dart';

class Cell extends StatefulWidget {
  final int? digit;
  final int index;
  final int size;
  final bool initialClue;

  const Cell(
    this.digit,
    this.index,
    this.size,
    this.initialClue, {
    super.key,
  });

  @override
  State<StatefulWidget> createState() => _CellState();
}

class _CellState extends State<Cell> {
  GameState state = GameState.getInstance();
  bool isCurrentlyError = false;

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
    if (widget.digit != null) {
      setErr();
    }
    
    if (GameState.getInstance().selectedDigit == 0 || widget.digit == null) {
      if (!GameState.getInstance().updateDigit(widget.index)) {
        setErr();
      }
    }

    
  }

  @override
  Widget build(BuildContext context) {
    var state = GameState.getInstance();
    double fontSize = state.size <= 9
        ? 30.0
        : state.size <= 16
            ? 15.0
            : 6.0;

    return InkWell(
      onTap: onClick,
      child: Container(
        color: isCurrentlyError ? Colors.red : Theme.of(context).highlightColor,
        alignment: Alignment.center,
        child: widget.digit != null
            ? Text(widget.digit!.toString(),
                style: TextStyle(
                    fontSize: fontSize,
                    color: widget.initialClue ? Colors.black : Colors.white))
            // 30 or 9x9, 15 for 16x16 , 6 for anything else (for now at least)
            : const Text(""),
      ),
    );
  }
}
