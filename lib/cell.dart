import 'package:flutter/material.dart';

class Cell extends StatefulWidget {
  final String digit;
  final int index;

  const Cell(this.digit, this.index, {super.key});

  @override
  State<StatefulWidget> createState() => _CellState();
}

class _CellState extends State<Cell> {
  @override
  Widget build(BuildContext context) {
    return InkWell(
      onTap: () {
        print("${widget.digit} ${widget.index}");
      },
      child: Container(
        color: const Color.fromARGB(255, 178, 195, 233),
        alignment: Alignment.center,
        child: !widget.digit.startsWith("0")
            ? Text(widget.digit, style: const TextStyle(fontSize: 30))
            : const Text(""),
      ),
    );
  }
}
