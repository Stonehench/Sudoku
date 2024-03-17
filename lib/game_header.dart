import 'dart:async';

import 'package:flutter/material.dart';
import 'package:sudoku/account.dart';
import 'package:sudoku/game_state.dart';
import 'package:sudoku/scoreboard.dart';

class GameHeader extends StatefulWidget {
  final Set<String> rules;
  const GameHeader(this.rules, {super.key});

  @override
  State<GameHeader> createState() => _DigitSelectState();
}

class _DigitSelectState extends State<GameHeader> {
  @override
  Widget build(BuildContext context) {
    return ListenableBuilder(
      listenable: GameState.getInstance(),
      builder: (context, _) {
        if (GameState.getInstance().gameDone()) {
          Timer(const Duration(milliseconds: 100), () async {
            Account? account = await getAccount();

            if (context.mounted) {
              showDialog(
                context: context,
                builder: (context) => AlertDialog(
                  title: const Text("You Win!"),
                  actions: [
                    OutlinedButton(
                      onPressed: () => Navigator.of(context)
                          .popUntil((route) => route.isFirst),
                      child: const Text("Home"),
                    ),
                    account == null
                        ? OutlinedButton(
                            onPressed: () async {
                              await Navigator.of(context).push(
                                  MaterialPageRoute(
                                      builder: (context) =>
                                          const AccountPage()));

                              account = await getAccount();

                              if (account != null) {
                                if (context.mounted) {
                                  Navigator.of(context).pop();
                                  setState(() {
                                    //Reload
                                  });
                                }
                              }
                            },
                            child: const Text("Login to submit score"))
                        : OutlinedButton(
                            onPressed: () async {
                              await Navigator.of(context).push(
                                  MaterialPageRoute(
                                      builder: (context) =>
                                          const Scoreboard()));
                              account = await getAccount();

                              //Hvis der siden er blevet logged ud
                              if (account == null) {
                                if (context.mounted) {
                                  Navigator.of(context).pop();
                                  setState(() {
                                    //Reload
                                  });
                                }
                              }
                            },
                            child: const Text("Check scoreboard"))
                  ],
                ),
              );
            }
          });
        }

        return Column(
          children: [
            TextButton(
                onPressed: () {
                  showDialog(
                    context: context,
                    builder: (context) => AlertDialog(
                      title: const Text("Rules"),
                      content: IntrinsicHeight(
                        child: Column(
                          children: widget.rules.map((e) => Text(e)).toList(),
                        ),
                      ),
                    ),
                  );
                },
                child: const Text("Rules")),
            const Text("Standard Sudoku", style: TextStyle(fontSize: 24)),
          ],
        );
      },
    );
  }
}
