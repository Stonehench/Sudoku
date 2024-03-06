import 'package:flutter/material.dart';
import 'package:sudoku/game_state.dart';

class DigitSelect extends StatefulWidget {
  final Object? size;
  const DigitSelect(this.size, {super.key});

  @override
  State<DigitSelect> createState() => _DigitSelectState();
}

class _DigitSelectState extends State<DigitSelect> {
  GameState state = GameState.getInstance();
  @override
  Widget build(BuildContext context) {
    double fontSize = widget.size! as int <= 9
        ? 30.0
        : widget.size as int <= 16
            ? 15.0
            : 6.0;
    return SizedBox(
      height: 50,
      width: 340,
      child: Container(
        alignment: Alignment.center,
        //color: const Color.fromARGB(255, 178, 195, 233),

        child: ListenableBuilder(
          listenable: GameState.getInstance(),
          builder: (cxt, _) => ListView.builder(
            //physics: const NeverScrollableScrollPhysics(),
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
                      : Theme.of(context).secondaryHeaderColor,
                  alignment: Alignment.center,
                  height: 50,
                  width: 40,
                  child: Text(
                    (index + 1).toString(),
                    style: TextStyle(
                      fontSize: fontSize,
                      color: Colors.black,
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
