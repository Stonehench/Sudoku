import 'dart:math';

import 'package:flutter/material.dart';
import 'package:sudoku/name_header.dart';
import 'rule_display.dart';
import 'package:sudoku/game_state.dart';

class GameHeader extends StatelessWidget {
  final Set<String> rules;
  const GameHeader(this.rules, {super.key});

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: 130,
      width: 340,
      child: Column(
        mainAxisAlignment: MainAxisAlignment.end,
        children: [
          Row(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              const BackButton(),
              const SizedBox(
                width: 240,
              ),
              SizedBox(
                // Use width to change distance from the edge
                width: 40,
                child: InkWell(
                  onTap: () {
                    showDialog(
                      context: context,
                      builder: (context) => AlertDialog(
                        title: const Align(
                          alignment: Alignment.center,
                          child: Text("Rules"),
                        ),
                        content: IntrinsicHeight(
                          child: SizedBox(
                            height: 300,
                            child: RawScrollbar(
                              thumbVisibility: true,
                              thumbColor: Color.fromARGB(99, 152, 152, 227),
                              child: SingleChildScrollView(
                                child: Column(
                                    children: List.from(
                                  [
                                    const Column(
                                      children: [
                                        Text(
                                          "Row Rule",
                                          style: TextStyle(fontSize: 20),
                                        ),
                                        Text(
                                            "For a regular shaped sudoku, any digit must appear exactly once in any row\n")
                                      ],
                                    ),
                                    const Column(
                                      children: [
                                        Text(
                                          "Column Rule",
                                          style: TextStyle(fontSize: 20),
                                        ),
                                        Text(
                                            "For a regular shaped sudoku, any digit must appear exactly once in any column\n")
                                      ],
                                    ),
                                  ],
                                )..addAll(getRules(rules))),
                              ),
                            ),
                          ),
                        ),
                      ),
                    );
                  },
                  child: const Align(
                    alignment: Alignment.centerLeft,
                    child: Icon(Icons.info),
                  ),
                ),
              ),
            ],
          ),
          const Spacer(),
          NameHeader(rules),
          const Spacer(),
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            crossAxisAlignment: CrossAxisAlignment.center,
            children: [
              ListenableBuilder(
                  listenable: GameState.getInstance(),
                  builder: (context, child) {
                    return Row(
                        mainAxisAlignment: MainAxisAlignment.start,
                        children: generateIcons(
                            Icons.favorite,
                            Icons.favorite_border,
                            Colors.red,
                            3,
                            GameState.getInstance().lives));
                  }),
              ListenableBuilder(
                  listenable: GameState.getInstance(),
                  builder: (context, child) {
                    return Row(
                        mainAxisAlignment: MainAxisAlignment.end,
                        children: generateIcons(
                            Icons.search,
                            Icons.search_outlined,
                            Colors.lightBlue,
                            sqrt(GameState.getInstance().size).toInt(),
                            GameState.getInstance().numberOfHint));
                  }),
            ],
          ),
          const SizedBox(
            height: 5,
            width: 10,
          )
        ],
      ),
    );
  }
}

List<Widget> generateIcons(IconData primaryIcon, IconData secondaryIcon,
    Color color, int numberOfIcons, int numberLeft) {
  List<Widget> hearts = [];
  for (int i = numberOfIcons; i > numberLeft; i--) {
    hearts.add(Icon(secondaryIcon, color: Colors.black));
  }
  for (int i = 0; i < numberLeft; i++) {
    hearts.add(Icon(primaryIcon, color: color));
  }
  return hearts;
}
