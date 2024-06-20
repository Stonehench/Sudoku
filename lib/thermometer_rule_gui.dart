import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:sudoku/game_state.dart';

class Thermometer extends StatefulWidget {
  const Thermometer({super.key});
  @override
  State<StatefulWidget> createState() => _ThermometerState();
}

// This file is responsible for drawing thermometers
class _ThermometerState extends State<Thermometer> {
  Color lineColor = const Color.fromARGB(255, 70, 112, 167);

  @override
  Widget build(BuildContext context) {
    GameState state = GameState.getInstance();
    return Stack(
      children: [
        // Circles in all cells
        GridView.builder(
          padding: EdgeInsets.zero,
          itemCount: state.board.length,
          gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
              crossAxisCount: state.size),
          itemBuilder: (context, index) {
            int dotWidth = 50;
            return Container(
              alignment: Alignment.center,
              child: state.thermometerPositions.any((thermometer) {
                // Make the first circle bigger
                if (thermometer.first == index) {
                  dotWidth = 100;
                }
                return thermometer
                    .any((element) => element == index || element == index);
              })
                  ? Container(
                      width: dotWidth / state.size,
                      height: dotWidth / state.size,
                      decoration: BoxDecoration(
                        shape: BoxShape.circle,
                        color: lineColor,
                      ),
                    )
                  : const Text(""),
            );
          },
        ),

        // Draw horizontal lines
        GridView.builder(
          padding: EdgeInsets.fromLTRB(
              (340 / (state.size)) / 2, 0, (340 / (state.size)) / 2, 0),
          itemCount: state.board.length - state.size,
          gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
              crossAxisCount: state.size - 1),
          itemBuilder: (context, index) {
            return Container(
              alignment: Alignment.center,
              child: state.thermometerPositions.any((thermometer) {
                int i = ((index % (state.size - 1)) +
                    ((index ~/ (state.size - 1)) * state.size));
                // if the center has a neighbour to the right
                for (int j = 0; j < thermometer.length; j++) {
                  int prev = j > 0 ? thermometer[j - 1] : -1;
                  int next =
                      j < thermometer.length - 1 ? thermometer[j + 1] : -1;
                  int current = thermometer[j];

                  if ((prev != -1 && current + 1 == prev && i == current) ||
                      (next != -1 && current + 1 == next && i == current)) {
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

        // Draw vertical lines
        GridView.builder(
          padding: EdgeInsets.fromLTRB(
              0, (340 / (state.size)) / 2, 0, (340 / (state.size)) / 2),
          itemCount: state.board.length - state.size,
          gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
              crossAxisCount: state.size),
          itemBuilder: (context, index) {
            return Container(
              alignment: Alignment.center,
              child: state.thermometerPositions.any((thermometer) {
                int s = state.size;
                // if the center has a neighbour to the right
                for (int j = 0; j < thermometer.length; j++) {
                  int prev = j > 0 ? thermometer[j - 1] : -1;
                  int next =
                      j < thermometer.length - 1 ? thermometer[j + 1] : -1;
                  int current = thermometer[j];

                  if ((prev != -1 && current + s == prev && index == current) ||
                      (next != -1 && current + s == next && index == current)) {
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

        // The reasoning for why there are 3 functions per diagonal are explained in depth in the report.
        // In short the first is the main diagonal drawing, the 2 right after are smaller drawings
        // seamlessly connecting the main diagonal to the circles.

        // The next three GridView.builder draws diagonals from top left to bottom right
        // Center diagonal drawing
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
              child: state.thermometerPositions.any((thermometer) {
                int s = state.size;
                // if the center has a neighbour to the right
                for (int j = 0; j < thermometer.length; j++) {
                  int prev = j > 0 ? thermometer[j - 1] : -1;
                  int next =
                      j < thermometer.length - 1 ? thermometer[j + 1] : -1;
                  int current = thermometer[j];

                  if ((prev != -1 && current + s + 1 == prev && i == current) ||
                      (next != -1 && current + s + 1 == next && i == current)) {
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

        // First small diagonal drawing
        GridView.builder(
          padding: EdgeInsets.zero,
          itemCount: state.board.length,
          gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
              crossAxisCount: state.size),
          itemBuilder: (context, index) {
            int i = index;
            return Container(
              alignment: Alignment.center,
              child: state.thermometerPositions.any((thermometer) {
                int s = state.size;
                // if the center has a neighbour to the right
                for (int j = 0; j < thermometer.length; j++) {
                  int prev = j > 0 ? thermometer[j - 1] : -1;
                  int next =
                      j < thermometer.length - 1 ? thermometer[j + 1] : -1;
                  int current = thermometer[j];

                  if ((prev != -1 && current + s + 1 == prev && i == current) ||
                      (next != -1 && current + s + 1 == next && i == current)) {
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

        // Second small diagonal drawing
        GridView.builder(
          padding: EdgeInsets.zero,
          itemCount: state.board.length,
          gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
              crossAxisCount: state.size),
          itemBuilder: (context, index) {
            int i = index;
            return Container(
              alignment: Alignment.center,
              child: state.thermometerPositions.any((thermometer) {
                int s = state.size;
                // if the center has a neighbour to the right
                for (int j = 0; j < thermometer.length; j++) {
                  int prev = j > 0 ? thermometer[j - 1] : -1;
                  int next =
                      j < thermometer.length - 1 ? thermometer[j + 1] : -1;
                  int current = thermometer[j];

                  if ((prev != -1 && current - s - 1 == prev && i == current) ||
                      (next != -1 && current - s - 1 == next && i == current)) {
                    return true;
                  }
                }
                return false;
              })
                  ? getSmallDiagonal(state, 0.785398163, Alignment.topLeft)
                  : const Text(""),
            );
          },
        ),

        // The next three GridView.builder draws diagonals from top right to bottom left
        // Center diagonal drawing
        GridView.builder(
          padding: EdgeInsets.zero,
          itemCount: state.board.length,
          gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
              crossAxisCount: state.size),
          itemBuilder: (context, index) {
            int i = index;
            return Container(
              alignment: Alignment.center,
              child: state.thermometerPositions.any((thermometer) {
                int s = state.size;
                // if the center has a neighbour to the right
                for (int j = 0; j < thermometer.length; j++) {
                  int prev = j > 0 ? thermometer[j - 1] : -1;
                  int next =
                      j < thermometer.length - 1 ? thermometer[j + 1] : -1;
                  int current = thermometer[j];

                  if ((prev != -1 && current + s - 1 == prev && i == current) ||
                      (next != -1 && current + s - 1 == next && i == current)) {
                    return true;
                  }
                }
                return false;
              })
                  ? getSmallDiagonal(state, -0.785398163, Alignment.bottomLeft)
                  : const Text(""),
            );
          },
        ),

        // First small diagonal drawing
        GridView.builder(
          padding: EdgeInsets.zero,
          itemCount: state.board.length,
          gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
              crossAxisCount: state.size),
          itemBuilder: (context, index) {
            int i = index;
            return Container(
              alignment: Alignment.center,
              child: state.thermometerPositions.any((thermometer) {
                int s = state.size;
                // if the center has a neighbour to the right
                for (int j = 0; j < thermometer.length; j++) {
                  int prev = j > 0 ? thermometer[j - 1] : -1;
                  int next =
                      j < thermometer.length - 1 ? thermometer[j + 1] : -1;
                  int current = thermometer[j];

                  if ((prev != -1 && current - s + 1 == prev && i == current) ||
                      (next != -1 && current - s + 1 == next && i == current)) {
                    return true;
                  }
                }
                return false;
              })
                  ? getSmallDiagonal(state, -0.785398163, Alignment.topRight)
                  : const Text(""),
            );
          },
        ),

        // Second small diagonal drawing
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
              child: state.thermometerPositions.any((thermometer) {
                int s = state.size;
                // if the center has a neighbour to the right
                for (int j = 0; j < thermometer.length; j++) {
                  int prev = j > 0 ? thermometer[j - 1] : -1;
                  int next =
                      j < thermometer.length - 1 ? thermometer[j + 1] : -1;
                  int current = thermometer[j];

                  if ((prev != -1 && current + s - 1 == prev && i == current) ||
                      (next != -1 && current + s - 1 == next && i == current)) {
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

  // Returns horizontal rectagles
  Widget getHorizontal(GameState state) {
    return SizedBox(
        width: 340 / state.size,
        height: 50 / state.size,
        child: Container(
          color: lineColor,
        ));
  }

  // Returns vertical rectagles
  Widget getVertical(GameState state) {
    return SizedBox(
        height: 340 / state.size,
        width: 50 / state.size,
        child: Container(
          color: lineColor,
        ));
  }

  // Returns small connecting diagonals
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
                  height: 50 / state.size,
                  width: 200 / state.size,
                  color: lineColor),
            ),
          )),
    );
  }

  // Returns Center diagonals
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
                height: 50 / state.size,
                color: lineColor,
              )),
        ));
  }
}
