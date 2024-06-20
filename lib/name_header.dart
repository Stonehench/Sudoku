import 'package:flutter/material.dart';

class NameHeader extends StatefulWidget {
  final Set<String> rules;
  const NameHeader(this.rules, {super.key});

  @override
  State<NameHeader> createState() => _NameHeaderState();
}

class _NameHeaderState extends State<NameHeader> {
  @override
  Widget build(BuildContext context) {
    List<String> name = List.empty(growable: true);
    // Find the name of the current Sudoku. Complex math for a complex problem.
    // Might be able to do better. It is what it is.
    widget.rules.containsAll({"ParityRule", "XRule"}) &&
            !widget.rules.contains("ConsecutiveRule")
        ? name.add("Tic Tac Toe")
        : widget.rules.containsAll({"ParityRule", "ConsecutiveRule"}) &&
                !widget.rules.contains("XRule")
            ? name.add("Dots")
            : widget.rules.contains("ParityRule") ||
                    widget.rules.contains("ConsecutiveRule") ||
                    widget.rules.contains("XRule")
                ? name.add("Domino")
                : name = name;

    widget.rules.contains("KnightsMove") ? name.add("Knightly") : name = name;

    widget.rules.contains("ZipperRule") &&
            widget.rules.contains("ThermometerRule")
        ? name.add("String Theory")
        : widget.rules.contains("ZipperRule")
            ? name.add("Zipper")
            : widget.rules.contains("ThermometerRule")
                ? name.add("Thermometer")
                : name = name;

    // Just a regular sudou
    widget.rules.length == 1 && widget.rules.contains("SquareRule")
        ? name.add("Classic")
        : name = name;

    widget.rules.length == 8
        ? {name.clear(), name.add("Chaos")}
        : widget.rules.isEmpty
            ? name.add("Bare Minimum")
            : name == name;

    List<Text> text = List.empty(growable: true);
    for (var i = 0; i < name.length; i++) {
      text.add(Text(
        "${name[i]} ",
        style: const TextStyle(fontSize: 20),
      ));
    }

    widget.rules.contains("SquareRule")
        ? text.add(const Text(
            "Sudoku",
            style: TextStyle(fontSize: 20),
          ))
        : text.add(const Text(
            "Puzzle",
            style: TextStyle(fontSize: 20),
          ));

    return Wrap(alignment: WrapAlignment.center, children: text);
  }
}
