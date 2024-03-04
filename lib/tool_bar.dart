import 'package:flutter/material.dart';
import 'package:sudoku/game_view.dart';

class ToolBar extends StatefulWidget {
  const ToolBar({super.key});

  @override
  State<ToolBar> createState() => _ToolBarState();
}

class _ToolBarState extends State<ToolBar> {
  @override
  Widget build(BuildContext context) {
    double fontSize = 30;

    return SizedBox(
      height: 50,
      width: 340,
      child: Container(
        alignment: Alignment.center,
        child: ButtonBar(
          alignment: MainAxisAlignment.center,
          children: [
            const BackButton(),
            TextButton(
                onPressed: () {
                  print("please erase the digit thanks:D");
                  GameState.selectedDigit = 0;
                  // and some logic to allow "placement" in already placed digits,
                  // just only the once the player placed themselves

                  // because it no work
                },
                child: Text("ERASE"))
          ],
        ),
      ),
    );
  }
}
