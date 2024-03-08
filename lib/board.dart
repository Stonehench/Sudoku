import 'dart:math';

import 'package:flutter/material.dart';
import 'package:sudoku/cell.dart';
import 'package:sudoku/game_state.dart';

class Board extends StatefulWidget {
  const Board({super.key});

  @override
  State<StatefulWidget> createState() => _BoardState();
}

class _BoardState extends State<Board> {
  @override
  Widget build(BuildContext context) {
    GameState state = GameState.getInstance();
    double fontSize = state.size <= 9
        ? 30.0
        : state.size <= 16
            ? 15.0
            : 6.0;

    return Center(
      child: SizedBox(
        height: 340,
        width: 340,
        child: InteractiveViewer(
          minScale: 1,
          maxScale: sqrt(state.size),
          child: Stack(
            alignment: Alignment.center,
            children: [
              Container(color: const Color.fromARGB(255, 19, 22, 54)),
              GridView.builder(
                padding: EdgeInsets.zero,
                itemCount: state.board.length,
                gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
                    crossAxisCount: sqrt(state.size).toInt(),
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
                itemCount: state.board.length,
                gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
                  crossAxisCount: state.size,
                  crossAxisSpacing: 2,
                  mainAxisSpacing: 2,
                ),
                itemBuilder: (ctx, index) {
                  return Container(
                    color: Theme.of(context).highlightColor,
                  );
                },
              ),
              // Horizontal X-rule
              GridView.builder(
                padding: EdgeInsets.fromLTRB(
                    (340 / (state.size)) / 2, 0, (340 / (state.size)) / 2, 0),
                itemCount: state.board.length - state.size,
                gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
                    crossAxisCount: state.size - 1,
                    crossAxisSpacing: 2,
                    mainAxisSpacing: 2),
                itemBuilder: (context, index) {
                  return Container(
                    alignment: Alignment.center,
                    child: state.xPositions.any((t) =>
                            t.$1 ==
                                ((index % (state.size - 1)) +
                                    ((index ~/ (state.size - 1)) *
                                        state.size)) &&
                            t.$2 ==
                                ((index % (state.size - 1)) +
                                        (index ~/ (state.size - 1)) *
                                            state.size) +
                                    1)
                        ? Text("X",
                            style: TextStyle(
                                fontSize: fontSize / 3 * 2,
                                color: const Color.fromARGB(255, 19, 22, 54)))
                        : const Text(""),
                  );
                },
              ),
              // Vertical X-rule
              GridView.builder(
                padding: EdgeInsets.fromLTRB(
                    0, (340 / (state.size)) / 2, 0, (340 / (state.size)) / 2),
                itemCount: state.board.length - state.size,
                gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
                    crossAxisCount: state.size,
                    crossAxisSpacing: 2,
                    mainAxisSpacing: 2),
                itemBuilder: (context, index) {
                  return Container(
                    alignment: Alignment.center,
                    child: state.xPositions.any(
                            (t) => t.$1 == index && t.$2 == t.$1 + state.size)
                        ? Text("X",
                            style: TextStyle(
                                fontSize: fontSize / 3 * 2,
                                color: const Color.fromARGB(255, 19, 22, 54)))
                        : const Text(""),
                  );
                },
              ),
              ListenableBuilder(
                listenable: state,
                builder: (ctx, _) => GridView.builder(
                  physics: const NeverScrollableScrollPhysics(),
                  padding: EdgeInsets.zero,
                  itemCount: state.board.length,
                  gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
                    crossAxisCount: state.size,
                    crossAxisSpacing: 2,
                    mainAxisSpacing: 2,
                  ),
                  itemBuilder: (ctx, index) {
                    return Cell(state.board[index], index, state.board.length,
                        state.initialClues.contains(index));
                  },
                ),
              )
            ],
          ),
        ),
      ),
    );
  }
}
