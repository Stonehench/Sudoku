import 'dart:math';

import 'package:flutter/material.dart';
import 'package:sudoku/cell.dart';
import 'package:sudoku/src/rust/api/simple.dart';

class Board extends StatefulWidget {
  final Object? size;
  Board(this.size, {super.key});
  String boardString = getSudokuStr()!;

  @override
  State<StatefulWidget> createState() => _BoardState();
}

class _BoardState extends State<Board> {
  @override
  Widget build(BuildContext context) {
    var boardArray = widget.boardString.split(",");
    return Scaffold(
      body: Center(
        child: SizedBox(
          height: 340,
          width: 340,
          child: Stack(
            alignment: Alignment.center,
            children: [
              Container(color: const Color.fromARGB(255, 19, 22, 54)),
              GridView.builder(
                padding: EdgeInsets.zero,
                itemCount: widget.size as int,
                gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
                    crossAxisCount: sqrt(widget.size as int).toInt(),
                    crossAxisSpacing: 2,
                    mainAxisSpacing: 2),
                itemBuilder: (context, index) {
                  return Container(
                    color: const Color.fromARGB(255, 127, 132, 177),
                  );
                },
              ),
              GridView.builder(
                physics: const NeverScrollableScrollPhysics(),
                padding: EdgeInsets.zero,
                itemCount: (widget.size as int) * (widget.size as int),
                gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
                    crossAxisCount: widget.size as int,
                    crossAxisSpacing: 2,
                    mainAxisSpacing: 2),
                itemBuilder: (context, index) {
                  return Cell(
                      boardArray.elementAt(index), index, widget.size as int);
                },
              ),
            ],
          ),
        ),
      ),
    );
  }
}
