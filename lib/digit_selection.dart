import 'package:flutter/material.dart';
import 'package:sudoku/game_state.dart';
import 'dart:math';

class DigitSelect extends StatefulWidget {
  final Object? size;
  const DigitSelect(this.size, {super.key});

  @override
  State<DigitSelect> createState() => _DigitSelectState();
}

spaceBetween(int number) {
  return 38.0 + 12 * (log(number) / ln10).floor();
}

class _DigitSelectState extends State<DigitSelect> {
  GameState state = GameState.getInstance();
  @override
  Widget build(BuildContext context) {
    double fontSize = 30;
    return SizedBox(
      height: 50,
      width: 340,
      child: Container(
        alignment: Alignment.center,
        child: ListenableBuilder(
          listenable: GameState.getInstance(),
          builder: (cxt, _) => ListView.builder(
            scrollDirection: Axis.horizontal,
            itemCount: widget.size as int,
            padding: const EdgeInsets.all(2),
            itemBuilder: (BuildContext context, int index) {
              return InkWell(
                onTap: () {
                  setState(() {
                    state.setSelected(index + 1);
                  });
                },
                child: Container(
                  color: GameState.getInstance().selectedDigit == index + 1
                      ? Theme.of(context).primaryColorLight
                      : Colors.transparent,
                  alignment: Alignment.center,
                  height: 38,
                  width: spaceBetween(index + 1),
                  child: Text(
                    (index + 1).toString(),
                    style: TextStyle(
                      fontSize: fontSize,
                      color: state.digitDone(index + 1)
                          ? Colors.green
                          : Colors.white,
                    ),
                  ),
                ),
              );
            },
          ),
        ),
      ),
    );
  }
}
