import 'dart:math';

import 'package:flutter/material.dart';
import 'package:sudoku/game_state.dart';

class Zipper extends StatefulWidget {
  const Zipper({super.key});

  @override
  State<StatefulWidget> createState() => _ZipperState();
}

class _ZipperState extends State<Zipper> {
  Color lineColor = const Color.fromARGB(255, 152, 118, 175);

  @override
  Widget build(BuildContext context) {
    GameState state = GameState.getInstance();

    return Center(
      child: Stack(
        children: [
          //Text( style: const TextStyle(fontSize: 20),state.zipperPositions.toString()),
          // circles in all cells
          GridView.builder(
            itemCount: state.board.length,
            gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
                crossAxisCount: state.size),
            itemBuilder: (context, index) {
              return Container(
                alignment: Alignment.center,
                child: state.zipperPositions.any((zipper) =>
                        // if the center has a neighbour below
                        zipper.$1 == index ||
                        zipper.$2.any((element) =>
                            element.$1 == index || element.$2 == index))
                    ? Container(
                        width: 10,
                        height: 10,
                        decoration: BoxDecoration(
                          shape: BoxShape.circle,
                          color: lineColor,
                        ),
                      )
                    : const Text(""),
              );
            },
          ),
          // Horizontal
          GridView.builder(
            padding: EdgeInsets.fromLTRB(
                (340 / (state.size)) / 2, 0, (340 / (state.size)) / 2, 0),
            itemCount: state.board.length - state.size,
            gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
                crossAxisCount: state.size - 1),
            itemBuilder: (context, index) {
              return Container(
                alignment: Alignment.center,
                child: state.zipperPositions.any((zipper) {
                  int i = ((index % (state.size - 1)) +
                      ((index ~/ (state.size - 1)) * state.size));
                  // if the center has a neighbour to the right
                  for (int j = 0; j < zipper.$2.length; j++) {
                    if (j < zipper.$2.length - 1 &&
                        (zipper.$2[j].$1 == i && zipper.$2[j + 1].$1 == i + 1 ||
                            zipper.$2[j].$1 == i &&
                                zipper.$2[j + 1].$2 == i + 1 ||
                            zipper.$2[j].$2 == i &&
                                zipper.$2[j + 1].$1 == i + 1 ||
                            zipper.$2[j].$2 == i &&
                                zipper.$2[j + 1].$2 == i + 1)) {
                      return true;
                    }
                    if (j > 0 &&
                        (zipper.$2[j].$1 == i && zipper.$2[j - 1].$1 == i + 1 ||
                            zipper.$2[j].$1 == i &&
                                zipper.$2[j - 1].$2 == i + 1 ||
                            zipper.$2[j].$2 == i &&
                                zipper.$2[j - 1].$1 == i + 1 ||
                            zipper.$2[j].$2 == i &&
                                zipper.$2[j - 1].$2 == i + 1)) {
                      return true;
                    }
                    if (zipper.$2[j].$1 == i && zipper.$1 == i + 1 ||
                        zipper.$2[j].$2 == i && zipper.$1 == i + 1) {
                      return true;
                    }
                  }
                  return zipper.$1 == i &&
                      (zipper.$2.first.$1 == zipper.$1 + 1 ||
                          zipper.$2.first.$2 == zipper.$1 + 1);
                })
                    ? getHorizontal(state)
                    : const Text(""),
              );
            },
          ),

          // Vertical
          GridView.builder(
            padding: EdgeInsets.fromLTRB(
                0, (340 / (state.size)) / 2, 0, (340 / (state.size)) / 2),
            itemCount: state.board.length - state.size,
            gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
                crossAxisCount: state.size),
            itemBuilder: (context, index) {
              return Container(
                alignment: Alignment.center,
                child: state.zipperPositions.any((zipper) {
                  // if the center has a neighbour to the right
                  for (int j = 0; j < zipper.$2.length; j++) {
                    int left = zipper.$2[j].$1;
                    int right = zipper.$2[j].$2;
                    if (j < zipper.$2.length - 1 &&
                            (left == index &&
                                    left + state.size == zipper.$2[j + 1].$1 ||
                                right == index &&
                                    right + state.size ==
                                        zipper.$2[j + 1].$2) ||
                        j > 0 &&
                            (left == index &&
                                    left + state.size == zipper.$2[j - 1].$1 ||
                                right == index &&
                                    right + state.size ==
                                        zipper.$2[j - 1].$2) ||
                        // next to cente
                        zipper.$1 == index &&
                            zipper.$1 + state.size == zipper.$2.first.$1 ||
                        zipper.$1 == index &&
                            zipper.$1 + state.size == zipper.$2.first.$2 ||
                        zipper.$2[j].$1 == index &&
                            zipper.$1 == index + state.size ||
                        zipper.$2[j].$2 == index &&
                            zipper.$1 == index + state.size) {
                      return true;
                    }
                  }
                  return false;
                })
                    ? getVertical(state)
                    : const Text(""),
              );
            },
          ),

          // diagonals
          GridView.builder(
            padding: EdgeInsets.fromLTRB(
                (340 / (state.size)) / 2,
                (340 / (state.size)) / 2,
                (340 / (state.size)) / 2,
                (340 / (state.size)) / 2),
            itemCount: state.board.length - state.size,
            gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
                crossAxisCount: state.size - 1),
            itemBuilder: (context, index) {
              int i = ((index % (state.size - 1)) +
                  ((index ~/ (state.size - 1)) * state.size));
              return Container(
                alignment: Alignment.center,
                child: state.zipperPositions.any((zipper) {
                  // if the center has a neighbour to the right
                  for (int j = 0; j < zipper.$2.length; j++) {
                    int left = zipper.$2[j].$1;
                    int right = zipper.$2[j].$2;
                    if (j < zipper.$2.length - 1 &&
                            (left == i &&
                                    left + state.size + 1 ==
                                        zipper.$2[j + 1].$1 ||
                                right == i &&
                                    right + state.size + 1 ==
                                        zipper.$2[j + 1].$2) ||
                        j > 0 &&
                            (left == i &&
                                    left + state.size + 1 ==
                                        zipper.$2[j - 1].$1 ||
                                right == i &&
                                    right + state.size + 1 ==
                                        zipper.$2[j - 1].$2) ||
                        // next to cente
                        zipper.$1 == i &&
                            zipper.$1 + state.size + 1 == zipper.$2.first.$1 ||
                        zipper.$1 == i &&
                            zipper.$1 + state.size + 1 == zipper.$2.first.$2 ||
                        zipper.$2[j].$1 == zipper.$2.first.$1 &&
                            zipper.$2[j].$1 == i &&
                            zipper.$1 == i + state.size + 1 ||
                        zipper.$2[j].$2 == zipper.$2.first.$2 &&
                            zipper.$2[j].$2 == i &&
                            zipper.$1 == i + state.size + 1) {
                      return true;
                    }
                  }
                  return false;
                })
                    ? getDiagonal(state, 0.785398163)
                    : const Text(""),
              );
            },
          ),
        ],
      ),
    );
  }

  Widget getHorizontal(GameState state) {
    return SizedBox(
        width: 340 / state.size,
        height: 10,
        child: Container(
          color: lineColor,
        ));
  }

  Widget getVertical(GameState state) {
    return SizedBox(
        height: 340 / state.size,
        width: 10,
        child: Container(
          color: lineColor,
        ));
  }

  Widget getDiagonal(GameState state, double angle) {
    return Transform.rotate(
        angle: angle,
        child: Container(
          width: 100,
          height: 10,
          color: lineColor,
        ));
  }
}
