import 'package:flutter/material.dart';

class Cell extends StatefulWidget {
  String number;

  Cell(this.number, {super.key});

  @override
  State<StatefulWidget> createState() => _CellState();
}

class _CellState extends State<Cell> {
  @override
  Widget build(BuildContext context) {
    return InkWell(
      onTap: () {
        print(widget.number);
      },
      child: Container(
        color: Color.fromARGB(255, 178, 195, 233),
        alignment: Alignment.center,
        child: !widget.number.startsWith("0")
            ? Text(widget.number, style: TextStyle(fontSize: 30))
            : const Text(""),
      ),
    );
  }
}
