import 'package:flutter/material.dart';
import 'package:sudoku/game_state.dart';

class Parity extends StatefulWidget {
  const Parity({super.key});

  @override
  State<StatefulWidget> createState() => _ParityState();
}

class _ParityState extends State<Parity> {
  Color lineColor = const Color.fromARGB(255, 152, 118, 175);

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
          // Horizontal parity
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
                child: state.parityPositions.any((t) =>
                        t.$1 ==
                            ((index % (state.size - 1)) +
                                ((index ~/ (state.size - 1)) * state.size)) &&
                        t.$2 ==
                            ((index % (state.size - 1)) +
                                    (index ~/ (state.size - 1)) * state.size) +
                                1)
                    ? Text("O",
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
                child: state.parityPositions
                        .any((t) => t.$1 == index && t.$2 == t.$1 + state.size)
                    ? Text("O",
                        style: TextStyle(
                            fontSize: fontSize / 3 * 2,
                            color: const Color.fromARGB(255, 19, 22, 54)))
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
