import 'package:flutter/material.dart';
import 'package:flutter_spinkit/flutter_spinkit.dart';
import 'package:sudoku/board.dart';
import 'package:sudoku/digit_selection.dart';
import 'package:sudoku/game_header.dart';
import 'package:sudoku/tool_bar.dart';
import 'package:sudoku/game_state.dart';

class GameView extends StatelessWidget {
  final Set<String> rules;
  const GameView(this.rules, {super.key});

  @override
  Widget build(BuildContext context) {
    var size = GameState.getInstance().size;

    GameState state = GameState.getInstance();

    return Scaffold(
      body: Stack(
        children: [
          Column(
            mainAxisAlignment: MainAxisAlignment.center,
            crossAxisAlignment: CrossAxisAlignment.center,
            children: [
              GameHeader(rules),
              const Board(),
              const SizedBox(height: 10),
              DigitSelect(size),
              const ToolBar(),
            ],
          ),
          ListenableBuilder(
            listenable: state,
            builder: (context, _) {
              return Center(
                child: !state.gameDone()
                    ? null
                    : Container(
                        decoration: BoxDecoration(
                          backgroundBlendMode: BlendMode.darken,
                          borderRadius:
                              const BorderRadius.all(Radius.circular(20)),
                          color: Theme.of(context).dialogBackgroundColor,
                        ),
                        padding: const EdgeInsets.all(20),
                        width: MediaQuery.of(context).size.width * 0.8,
                        height: MediaQuery.of(context).size.height * 0.5,
                        child: Column(
                          children: [
                            const Text(
                              "You win!",
                              style: TextStyle(fontSize: 35),
                            ),
                            if (state.scoreInAir()) ...[
                              const SpinKitChasingDots(
                                color: Colors.red,
                              )
                            ] else if (state.submitted()) ...[
                              Text("You gained X points"),
                            ] else ...[
                              const Text(
                                  "Failed to submit score. Check yo wifi or login"),
                              OutlinedButton(
                                  onPressed: () {
                                    state.trySubmitScore();
                                  },
                                  child: const Text("Retry"))
                            ]
                          ],
                        ),
                      ),
              );
            },
          ),
        ],
      ),
    );
  }
}
