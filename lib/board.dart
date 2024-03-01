import 'dart:ffi';

import 'package:flutter/material.dart';
import 'package:sudoku/cell.dart';

class Board extends StatefulWidget {
  String boardString;

  Board(this.boardString);

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
          height: 420,
          width: 420,
          child: Stack(
            children: [
              Container(color: Color.fromARGB(255, 19, 22, 54)),
              GridView.builder(
                padding: EdgeInsets.zero,
                gridDelegate: const SliverGridDelegateWithFixedCrossAxisCount(
                    crossAxisCount: 3, crossAxisSpacing: 2, mainAxisSpacing: 2),
                itemBuilder: (context, index) {
                  return Container(
                    color: Color.fromARGB(255, 127, 132, 177),
                  );
                },
              ),
              GridView.builder(
                padding: EdgeInsets.zero,
                itemCount: 81,
                gridDelegate: const SliverGridDelegateWithFixedCrossAxisCount(
                    crossAxisCount: 9, crossAxisSpacing: 2, mainAxisSpacing: 2),
                itemBuilder: (context, index) {
                  return Cell(boardArray.elementAt(index));
                },
              ),
            ],
          ),
        ),
      ),
    );
  }
}
