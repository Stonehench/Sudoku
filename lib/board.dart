import 'dart:math';

import 'package:flutter/material.dart';
import 'package:sudoku/cell.dart';
import 'package:sudoku/game_state.dart';
import 'package:sudoku/domino_gui.dart';
import 'package:sudoku/zipper_rule_gui.dart';

class Board extends StatefulWidget {
  const Board({super.key});

  @override
  State<StatefulWidget> createState() => _BoardState();
}

class _BoardState extends State<Board> {
  @override
  Widget build(BuildContext context) {
    GameState state = GameState.getInstance();
    List<(int, int)> dominoPositions = List.empty(growable: true);
    dominoPositions.addAll(state.consecutivePositions);
    dominoPositions.addAll(state.xPositions);
    dominoPositions.addAll(state.parityPositions);

    Color cellColor = const Color.fromARGB(255, 143, 147, 179);
    Color symbolColor = const Color.fromARGB(255, 44, 52, 134);

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
                  return Container(color: cellColor);
                },
              ),
              Domino("■", dominoPositions, cellColor),
              const Zipper(),
              Domino("⨯", state.xPositions, symbolColor),
              Domino("◦", state.parityPositions, symbolColor),
              Domino("•", state.consecutivePositions, symbolColor),
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
