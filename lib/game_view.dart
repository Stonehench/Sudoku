import 'dart:math';

import 'package:flutter/material.dart';
import 'package:flutter_spinkit/flutter_spinkit.dart';
import 'package:sudoku/account.dart';
import 'package:sudoku/board.dart';
import 'package:sudoku/digit_selection.dart';
import 'package:sudoku/game_header.dart';
import 'package:sudoku/scoreboard.dart';
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
              Board((rules.contains("SquareRule"))),
              const SizedBox(height: 10),
              DigitSelect(size),
              const ToolBar(),
            ],
          ),
          ListenableBuilder(
            listenable: state,
            builder: (context, _) {
              if (state.scoreStatus() != ScoreSubmissionStatus.gameNotDone) {
                return victoryWidget(context, state);
              } else if (state.lives <= 0) {
                return defeatWidget(context, state);
              } else {
                return const SizedBox.shrink();
              }
            },
          ),
        ],
      ),
    );
  }

  Widget victoryWidget(BuildContext context, GameState state) {
    var buttonStyle = OutlinedButton.styleFrom(
      shape: const RoundedRectangleBorder(
          borderRadius: BorderRadius.all(Radius.circular(5))),
    );

    return Center(
      child: Container(
        decoration: BoxDecoration(
          backgroundBlendMode: BlendMode.darken,
          borderRadius: const BorderRadius.all(Radius.circular(20)),
          border: Border.all(color: Colors.white),
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
            ...switch (state.scoreStatus()) {
              ScoreSubmissionStatus.gameNotDone ||
              ScoreSubmissionStatus.unSubmitted =>
                [SpinKitCircle(color: Theme.of(context).highlightColor)],
              ScoreSubmissionStatus.inAir => [
                  SpinKitCircle(color: Theme.of(context).highlightColor)
                ],
              ScoreSubmissionStatus.submitted => [
                  Text("Gained ${state.tryGetScore()!} points"),
                  const ScoreboardEmbed(onlyYou: true),
                ],
              ScoreSubmissionStatus.noWifi => [
                  const Text("Failed to connect to server. Check your wifi")
                ],
              ScoreSubmissionStatus.serverError => [
                  Text("Internal server error: ${state.serverErrorStatus}")
                ],
              ScoreSubmissionStatus.noAccount => [
                  const Text("Not logged in"),
                  OutlinedButton(
                      style: buttonStyle,
                      onPressed: () async {
                        await Navigator.of(context).push(MaterialPageRoute(
                            builder: (context) => const AccountPage()));
                        state.retryScoreSubmit();
                      },
                      child: const Text("Login"))
                ],
            },
            const Spacer(),
            OutlinedButton(
                style: buttonStyle,
                onPressed: () {
                  Navigator.of(context).pop();
                },
                child: const Text("Home")),
          ],
        ),
      ),
    );
  }

  Widget defeatWidget(BuildContext context, GameState state) {
    var buttonStyle = OutlinedButton.styleFrom(
      shape: const RoundedRectangleBorder(
          borderRadius: BorderRadius.all(Radius.circular(5))),
    );

    return Center(
      child: Container(
        decoration: BoxDecoration(
          backgroundBlendMode: BlendMode.darken,
          borderRadius: const BorderRadius.all(Radius.circular(20)),
          border: Border.all(color: Colors.white),
          color: Theme.of(context).dialogBackgroundColor,
        ),
        padding: const EdgeInsets.all(20),
        width: MediaQuery.of(context).size.width * 0.8,
        height: MediaQuery.of(context).size.height * 0.5,
        child: Column(
          children: [
            Text(
              getLoseText(),
              style: const TextStyle(fontSize: 35),
              textAlign: TextAlign.center,
            ),
            const Spacer(),
            OutlinedButton(
                style: buttonStyle,
                onPressed: () {
                  Navigator.of(context).pop();
                },
                child: const Text("Home")),
          ],
        ),
      ),
    );
  }
}

String getLoseText() {
  List<String> loseText = [
    "You suck!",
    "Loser!",
    "Give up!",
    "Really?",
    "...?",
    "???",
    "Your mom!",
    "Did your dog play?",
    "IQ = 5",
    "Are you dumb?",
    "Congrats!\n Your the first to fail such a simple sudoku!",
    "Maybe try a 1x1?",
    "Delete the app!",
    "kys!",
    "Go commit die!"
  ];

  int index = Random().nextInt(loseText.length);

  return loseText[index];
  //return "You Lose!";
}
