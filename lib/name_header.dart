import 'package:flutter/material.dart';
import 'package:sudoku/game_state.dart';

class NameHeader extends StatefulWidget {
  final Set<String> rules;
  const NameHeader(this.rules, {super.key});

  @override
  State<NameHeader> createState() => _NameHeaderState();
}

class _NameHeaderState extends State<NameHeader> {
  @override
  Widget build(BuildContext context) {
    String name = "";

    print(widget.rules);

    // is it a domino sudoku
    widget.rules.contains("ParityRule") ||
            widget.rules.contains("ConsecutiveRule") ||
            widget.rules.contains("XRule")
        ? name = "Domino"
        : name = name;

    widget.rules.containsAll({"ParityRule", "XRule"}) &&
            !widget.rules.contains("ConsecutiveRule")
        ? name = "Tic Tac Toe"
        : name = name;

    widget.rules.containsAll({"ParityRule", "ConsecutiveRule"}) &&
            !widget.rules.contains("XRule")
        ? name = "Dots"
        : name = name;
    widget.rules.contains("KnightsMove")
        ? name.isEmpty
            ? name = "Knights Move"
            : name = "Knights Move $name"
        : name = name;
    widget.rules.contains("ZipperRule") ? name = "Zipper$name" : name = name;

    // Just a regular sudou
    widget.rules.length == 1 && widget.rules.contains("SquareRule")
        ? name = "Classic"
        : name = name;

    widget.rules.length == 8 ? name = "Chaos" : name = name;
    widget.rules.isEmpty
        ? name = "Bare Minimum"
        : name == "Tic Tac Toe"
            ? name = name
            : name = "$name Sudoku";
    return Text(name,
        style: const TextStyle(fontSize: 20, color: Colors.white));
  }
}
