import 'package:flutter/material.dart';
import 'rule_display.dart';

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
          const Text("Standard Sudoku", style: TextStyle(fontSize: 30)),
        ],
      ),
    );
  }
}
