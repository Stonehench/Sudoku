import 'package:flutter/material.dart';

class GameHeader extends StatefulWidget {
  final Set<String> rules;
  const GameHeader(this.rules, {super.key});

  @override
  State<GameHeader> createState() => _DigitSelectState();
}

class _DigitSelectState extends State<GameHeader> {
  //var rulesAsString =
//                      gameModes.fold("", (prev, e) => prev + e + "\n");
  @override
  Widget build(BuildContext context) {
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
  }
}
