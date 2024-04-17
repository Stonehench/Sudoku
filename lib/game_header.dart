import 'package:flutter/material.dart';
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
                        title: const Text("Rules"),
                        content: IntrinsicHeight(
                          child: Column(
                            children: rules.map((e) => Text(e)).toList(),
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
          ListenableBuilder(listenable: GameState.getInstance(), builder: (context, child) {
            return Row(children: generateLives());
          })
        ],
      ),
    );
  }
}

List<Widget> generateLives() {
    List<Widget> hearts = [];
    for (int i = 3; i > GameState.getInstance().lives; i--) {
      hearts.add(const Icon(Icons.favorite_border, color: Colors.black));
    }
    for (int i = 0; i < GameState.getInstance().lives; i++) {
      hearts.add(const Icon(Icons.favorite, color: Colors.red));
    }
    return hearts;
  }
