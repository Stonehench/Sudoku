import 'dart:math';

import 'package:flutter/material.dart';
import 'rule_display.dart';
import 'package:sudoku/game_state.dart';

class GameHeader extends StatelessWidget {
  final Set<String> rules;
  const GameHeader(this.rules, {super.key});

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: 100,
      width: 340,
      child: Column(
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
                                  children: getRules(rules),
                                ),
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
          const Text("Standard Sudoku", style: TextStyle(fontSize: 24)),
          Row(
            mainAxisAlignment: MainAxisAlignment.center,
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
              const SizedBox(
                width: 100,
                height: 10,
              ),
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
