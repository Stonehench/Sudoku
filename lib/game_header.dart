import 'package:flutter/material.dart';

class GameHeader extends StatelessWidget {
  final Set<String> rules;
  const GameHeader(this.rules, {super.key});

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
                      children: rules.map((e) => Text(e)).toList(),
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
