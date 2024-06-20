import 'package:flutter/material.dart';
import 'package:sudoku/game_state.dart';

// This widget call is responsible for all domino rules
class Domino extends StatefulWidget {
  final String symbol;
  final List<(int, int)> positions;
  final Color color;
  const Domino(this.symbol, this.positions, this.color, {super.key});

  @override
  State<StatefulWidget> createState() => _DominoState();
}

// an argument, that is the sign that has to be displayed, is passed in
// along with all the coordinates from the domino clue, where the displaying should happen.
class _DominoState extends State<Domino> {
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
          // Horizontal
          // create all the horizontal dominos
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
                child: widget.positions.any((t) =>
                        t.$1 ==
                            ((index % (state.size - 1)) +
                                ((index ~/ (state.size - 1)) * state.size)) &&
                        t.$2 ==
                            ((index % (state.size - 1)) +
                                    (index ~/ (state.size - 1)) * state.size) +
                                1)
                    ? Text(widget.symbol,
                        style: TextStyle(
                            fontSize: fontSize / 3 * 2, color: widget.color))
                    : const Text(""),
              );
            },
          ),
          // Vertical
          // create all the vertical dominos
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
                child: widget.positions
                        .any((t) => t.$1 == index && t.$2 == t.$1 + state.size)
                    ? Text(widget.symbol,
                        style: TextStyle(
                            fontSize: fontSize / 3 * 2, color: widget.color))
                    : const Text(""),
              );
            },
          ),
        ],
      ),
    );
  }
}
