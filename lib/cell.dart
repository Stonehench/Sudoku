import 'dart:async';
import 'dart:ffi';
import 'dart:math';

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
      if (mounted) {
        setState(() {
          isCurrentlyError = false;
        });
      }
    });
  }

  void onClick() async {
    if (GameState.getInstance().drafting) {
      GameState.getInstance().changeDraft(widget.index);
      return;
    }

    //Check
    if (widget.digit != null) {
      setErr();
    }

    if ((GameState.getInstance().selectedDigit == 0 && !widget.initialClue) ||
        widget.digit == null) {
      if (!await GameState.getInstance().updateDigit(widget.index)) {
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

    var txtColor = widget.initialClue ? Colors.black : Colors.white;

    if (state.selectedDigit == widget.digit) {
      txtColor = Colors.amberAccent;
    }

    return Stack(children: [
      Container(
        color: isCurrentlyError ? Colors.red : Colors.transparent,
      ),
      InkWell(
        onTap: onClick,
        child: Container(
          color: Colors.transparent,
          alignment: Alignment.center,
          child: widget.digit != null
              ? Text(widget.digit!.toString(),
                  style: TextStyle(fontSize: fontSize, color: txtColor))
              // 30 or 9x9, 15 for 16x16 , 6 for anything else (for now at least)
              : Wrap(
                  spacing: 2.0,
                  children: state.drafts[widget.index]
                      .map((n) => Text(n.toString()))
                      .toList(),
                ),
        ),
      ),
    ]);
  }
}
