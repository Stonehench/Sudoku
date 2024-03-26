import 'dart:math';

import 'package:flutter/material.dart';
import 'package:sudoku/game_state.dart';

class Zipper extends StatefulWidget {
  const Zipper({super.key});

  @override
  State<StatefulWidget> createState() => _ZipperState();
}

class _ZipperState extends State<Zipper> {
  @override
  Widget build(BuildContext context) {
    GameState state = GameState.getInstance();

    double fontSize = state.size <= 9
        ? 30.0
        : state.size <= 16
            ? 15.0
            : 6.0;

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
                    ? SizedBox(
                        height: 10,
                        width: 10,
                        child: Container(
                          decoration: const BoxDecoration(
                            shape: BoxShape.circle,
                            color: Color.fromARGB(255, 129, 78, 163),
                          ),
                        ))
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
                            left == index &&
                                left + state.size == zipper.$2[j + 1].$2 ||
                            right == index &&
                                right + state.size == zipper.$2[j + 1].$1 ||
                            right == index &&
                                right + state.size == zipper.$2[j + 1].$2)) {
                      return true;
                    }

                    if (j > 0 &&
                        (left == index &&
                                left + state.size == zipper.$2[j - 1].$1 ||
                            left == index &&
                                left + state.size == zipper.$2[j - 1].$2 ||
                            right == index &&
                                right + state.size == zipper.$2[j - 1].$1 ||
                            right == index &&
                                right + state.size == zipper.$2[j - 1].$2)) {
                      return true;
                    }

                    // next to the center
                    if (zipper.$1 == index &&
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
        ],
      ),
    );
  }

  Widget getHorizontal(GameState state) {
    return SizedBox(
        width: 340 / state.size,
        height: 10,
        child: Container(
          color: const Color.fromARGB(255, 129, 78, 163),
        ));
  }

  Widget getVertical(GameState state) {
    return SizedBox(
        height: 340 / state.size,
        width: 10,
        child: Container(
          color: const Color.fromARGB(255, 129, 78, 163),
        ));
  }
}
