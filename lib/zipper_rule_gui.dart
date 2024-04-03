import 'dart:math';

import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
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

    return Stack(
      children: [
        // circles in all cells
        GridView.builder(
          padding: EdgeInsets.zero,
          itemCount: state.board.length,
          gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
              crossAxisCount: state.size),
          itemBuilder: (context, index) {
            int dotWidth = 100;
            return Container(
              alignment: Alignment.center,
              child: state.zipperPositions.any((zipper) {
                if (zipper.$1 == index) {
                  dotWidth = 200;
                }
                return zipper.$1 == index ||
                    zipper.$2.any((element) =>
                        element.$1 == index || element.$2 == index);
              })
                  ? Container(
                      width: dotWidth / state.size,
                      height: dotWidth / state.size,
                      //color: lineColor,
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
                  int prevL = -1;
                  int prevR = -1;
                  int nextL = -1;
                  int nextR = -1;
                  if (j > 0) {
                    prevL = zipper.$2[j - 1].$1;
                    prevR = zipper.$2[j - 1].$2;
                  }
                  int left = zipper.$2[j].$1;
                  int right = zipper.$2[j].$2;
                  if (j < zipper.$2.length - 1) {
                    nextL = zipper.$2[j + 1].$1;
                    nextR = zipper.$2[j + 1].$2;
                  }

                  if ((prevL != -1 && left + 1 == prevL && i == left) ||
                      (prevR != -1 && right + 1 == prevR && i == right) ||
                      (nextL != -1 && left + 1 == nextL && i == left) ||
                      (nextR != -1 && right + 1 == nextR && i == right) ||
                      zipper.$1 == i &&
                          (zipper.$2.first.$1 == zipper.$1 + 1 ||
                              zipper.$2.first.$2 == zipper.$1 + 1) ||
                      i == left &&
                          left + 1 == zipper.$1 &&
                          zipper.$2.first.$1 == left ||
                      i == right &&
                          right + 1 == zipper.$1 &&
                          zipper.$2.first.$2 == right) {
                    return true;
                  }
                }
                return false;
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
                int s = state.size;
                // if the center has a neighbour to the right
                for (int j = 0; j < zipper.$2.length; j++) {
                  int prevL = -1;
                  int prevR = -1;
                  int nextL = -1;
                  int nextR = -1;
                  if (j > 0) {
                    prevL = zipper.$2[j - 1].$1;
                    prevR = zipper.$2[j - 1].$2;
                  }
                  int left = zipper.$2[j].$1;
                  int right = zipper.$2[j].$2;
                  if (j < zipper.$2.length - 1) {
                    nextL = zipper.$2[j + 1].$1;
                    nextR = zipper.$2[j + 1].$2;
                  }

                  if ((prevL != -1 && left + s == prevL && index == left) ||
                      (prevR != -1 && right + s == prevR && index == right) ||
                      (nextL != -1 && left + s == nextL && index == left) ||
                      (nextR != -1 && right + s == nextR && index == right) ||
                      zipper.$1 == index &&
                          (zipper.$2.first.$1 == zipper.$1 + s ||
                              zipper.$2.first.$2 == zipper.$1 + s) ||
                      index == left &&
                          left + s == zipper.$1 &&
                          zipper.$2.first.$1 == left ||
                      index == right &&
                          right + s == zipper.$1 &&
                          zipper.$2.first.$2 == right) {
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

        // diagonals from top left to bottom right
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
                int s = state.size;
                // if the center has a neighbour to the right
                for (int j = 0; j < zipper.$2.length; j++) {
                  int prevL = -1;
                  int prevR = -1;
                  int nextL = -1;
                  int nextR = -1;
                  if (j > 0) {
                    prevL = zipper.$2[j - 1].$1;
                    prevR = zipper.$2[j - 1].$2;
                  }
                  int left = zipper.$2[j].$1;
                  int right = zipper.$2[j].$2;
                  if (j < zipper.$2.length - 1) {
                    nextL = zipper.$2[j + 1].$1;
                    nextR = zipper.$2[j + 1].$2;
                  }

                  if ((prevL != -1 && left + s + 1 == prevL && i == left) ||
                      (prevR != -1 && right + s + 1 == prevR && i == right) ||
                      (nextL != -1 && left + s + 1 == nextL && i == left) ||
                      (nextR != -1 && right + s + 1 == nextR && i == right) ||
                      zipper.$1 == i &&
                          (zipper.$2.first.$1 == zipper.$1 + s + 1 ||
                              zipper.$2.first.$2 == zipper.$1 + s + 1) ||
                      i == left &&
                          left + s + 1 == zipper.$1 &&
                          zipper.$2.first.$1 == left ||
                      i == right &&
                          right + s + 1 == zipper.$1 &&
                          zipper.$2.first.$2 == right) {
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
        GridView.builder(
          padding: EdgeInsets.zero,
          itemCount: state.board.length,
          gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
              crossAxisCount: state.size),
          itemBuilder: (context, index) {
            int i = index;
            return Container(
              alignment: Alignment.center,
              child: state.zipperPositions.any((zipper) {
                int s = state.size;
                // if the center has a neighbour to the right
                for (int j = 0; j < zipper.$2.length; j++) {
                  int prevL = -1;
                  int prevR = -1;
                  int nextL = -1;
                  int nextR = -1;
                  if (j > 0) {
                    prevL = zipper.$2[j - 1].$1;
                    prevR = zipper.$2[j - 1].$2;
                  }
                  int left = zipper.$2[j].$1;
                  int right = zipper.$2[j].$2;
                  if (j < zipper.$2.length - 1) {
                    nextL = zipper.$2[j + 1].$1;
                    nextR = zipper.$2[j + 1].$2;
                  }

                  if ((prevL != -1 && left + s + 1 == prevL && i == left) ||
                      (prevR != -1 && right + s + 1 == prevR && i == right) ||
                      (nextL != -1 && left + s + 1 == nextL && i == left) ||
                      (nextR != -1 && right + s + 1 == nextR && i == right) ||
                      zipper.$1 == i &&
                          (zipper.$2.first.$1 == zipper.$1 + s + 1 ||
                              zipper.$2.first.$2 == zipper.$1 + s + 1) ||
                      i == left &&
                          left + s + 1 == zipper.$1 &&
                          zipper.$2.first.$1 == left ||
                      i == right &&
                          right + s + 1 == zipper.$1 &&
                          zipper.$2.first.$2 == right) {
                    return true;
                  }
                }
                return false;
              })
                  ? getSmallDiagonal(state, 0.785398163, Alignment.bottomRight)
                  : const Text(""),
            );
          },
        ),
        /*
        GridView.builder(
          padding: EdgeInsets.zero,
          itemCount: state.board.length,
          gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
              crossAxisCount: state.size),
          itemBuilder: (context, index) {
            int i = index;
            return Container(
              alignment: Alignment.center,
              child: state.zipperPositions.any((zipper) {
                int s = state.size;
                // if the center has a neighbour to the right
                for (int j = 0; j < zipper.$2.length; j++) {
                  int prevL = -1;
                  int prevR = -1;
                  int nextL = -1;
                  int nextR = -1;
                  if (j > 0) {
                    prevL = zipper.$2[j - 1].$1;
                    prevR = zipper.$2[j - 1].$2;
                  }
                  int left = zipper.$2[j].$1;
                  int right = zipper.$2[j].$2;
                  if (j < zipper.$2.length - 1) {
                    nextL = zipper.$2[j + 1].$1;
                    nextR = zipper.$2[j + 1].$2;
                  }

                  if ((prevL != -1 && left - s - 1 == prevL && i == left) ||
                      (prevR != -1 && right - s - 1 == prevR && i == right) ||
                      (nextL != -1 && left - s - 1 == nextL && i == left) ||
                      (nextR != -1 && right - s - 1 == nextR && i == right) ||
                      zipper.$1 == i &&
                          (zipper.$2.first.$1 == zipper.$1 - s - 1 ||
                              zipper.$2.first.$2 == zipper.$1 - s - 1) ||
                      i == left &&
                          left - s - 1 == zipper.$1 &&
                          zipper.$2.first.$1 == left ||
                      i == right &&
                          right - s - 1 == zipper.$1 &&
                          zipper.$2.first.$2 == right) {
                    return true;
                  }
                }
                return false;
              })
                  ? getSmallDiagonal(state, 0.785398163, Alignment.topLeft)
                  : const Text(""),
            );
          },
        ), */
        // diagonals from top right to bottom left
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
                    ((index ~/ (state.size - 1)) * state.size)) +
                1;
            return Container(
              alignment: Alignment.center,
              child: state.zipperPositions.any((zipper) {
                int s = state.size;
                // if the center has a neighbour to the right
                for (int j = 0; j < zipper.$2.length; j++) {
                  int prevL = -1;
                  int prevR = -1;
                  int nextL = -1;
                  int nextR = -1;
                  if (j > 0) {
                    prevL = zipper.$2[j - 1].$1;
                    prevR = zipper.$2[j - 1].$2;
                  }
                  int left = zipper.$2[j].$1;
                  int right = zipper.$2[j].$2;
                  if (j < zipper.$2.length - 1) {
                    nextL = zipper.$2[j + 1].$1;
                    nextR = zipper.$2[j + 1].$2;
                  }

                  if ((prevL != -1 && left + s - 1 == prevL && i == left) ||
                      (prevR != -1 && right + s - 1 == prevR && i == right) ||
                      (nextL != -1 && left + s - 1 == nextL && i == left) ||
                      (nextR != -1 && right + s - 1 == nextR && i == right) ||
                      zipper.$1 == i &&
                          (zipper.$2.first.$1 == zipper.$1 + s - 1 ||
                              zipper.$2.first.$2 == zipper.$1 + s - 1) ||
                      i == left &&
                          left + s - 1 == zipper.$1 &&
                          zipper.$2.first.$1 == left ||
                      i == right &&
                          right + s - 1 == zipper.$1 &&
                          zipper.$2.first.$2 == right) {
                    return true;
                  }
                }
                return false;
              })
                  ? getDiagonal(state, -0.785398163)
                  : const Text(""),
            );
          },
        ),
      ],
    );
  }

  Widget getHorizontal(GameState state) {
    return SizedBox(
        width: 340 / state.size,
        height: 100 / state.size,
        child: Container(
          color: lineColor,
        ));
  }

  Widget getVertical(GameState state) {
    return SizedBox(
        height: 340 / state.size,
        width: 100 / state.size,
        child: Container(
          color: lineColor,
        ));
  }

  Widget getSmallDiagonal(GameState state, double angle, Alignment alignment) {
    return Align(
      alignment: alignment,
      child: SizedBox(
          height: 200 / state.size,
          width: 200 / state.size,
          child: Align(
            alignment: Alignment.center,
            child: Transform.rotate(
              angle: angle,
              child: Container(
                  height: 100 / state.size,
                  width: 200 / state.size,
                  color: lineColor),
            ),
          )),
    );
  }

  Widget getDiagonal(GameState state, double angle) {
    return SizedBox(
        height: 340 / state.size,
        width: 340 / state.size,
        child: Align(
          alignment: Alignment.center,
          child: Transform.rotate(
              angle: angle,
              child: Container(
                width: 340 / state.size,
                height: 100 / state.size,
                color: lineColor,
              )),
        ));
  }
}
