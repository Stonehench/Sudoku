import 'package:flutter/material.dart';
import 'package:sudoku/game_state.dart';

class ToolBar extends StatefulWidget {
  const ToolBar({super.key});

  @override
  State<ToolBar> createState() => _ToolBarState();
}

class _ToolBarState extends State<ToolBar> {
  @override
  Widget build(BuildContext context) {
    //double fontSize = 30;

    return SizedBox(
      height: 50,
      width: 340,
      child: Container(
        alignment: Alignment.center,
        child: ListenableBuilder(
          listenable: GameState.getInstance(),
          builder: (ctx, _) {
            GameState state = GameState.getInstance();
            ScoreSubmissionStatus status = state.scoreStatus();
            bool inGame = status == ScoreSubmissionStatus.gameNotDone &&
                !(state.lives <= 0);
            return ButtonBar(
              alignment: MainAxisAlignment.center,
              children: [
                TextButton(
                  onPressed: inGame
                      ? () {
                          //print("please erase the digit thanks:D");
                          GameState.getInstance().setSelected(0);

                          // It kinda works might need a rework!
                        }
                      : null,
                  style: TextButton.styleFrom(
                    backgroundColor: GameState.getInstance().selectedDigit == 0
                        ? Theme.of(ctx).secondaryHeaderColor
                        : Theme.of(ctx).scaffoldBackgroundColor,
                  ),
                  child: const Text("Erase"),
                ),
                TextButton(
                  onPressed: inGame
                      ? () {
                          //print("please erase the digit thanks:D");
                          GameState.getInstance().switchDrafting();

                          // It kinda works might need a rework!
                        }
                      : null,
                  style: TextButton.styleFrom(
                    backgroundColor: GameState.getInstance().drafting
                        ? Theme.of(ctx).secondaryHeaderColor
                        : Theme.of(ctx).scaffoldBackgroundColor,
                  ),
                  child: const Text("Draft"),
                ),
                TextButton(
                  onPressed: GameState.getInstance().numberOfHint > 0
                      ? inGame
                          ? () {
                              GameState.getInstance().getHint();
                            }
                          : null
                      : null,
                  style: TextButton.styleFrom(
                    backgroundColor: Theme.of(ctx).scaffoldBackgroundColor,
                  ),
                  child: const Text("Hint"),
                ),
              ],
            );
          },
        ),
      ),
    );
  }
}
