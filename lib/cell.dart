import 'package:flutter/material.dart';
import 'package:sudoku/src/rust/api/simple.dart';

class Cell extends StatefulWidget {
  final String digit;
  final int index;
  final int size;

  const Cell(this.digit, this.index, this.size, {super.key});

  @override
  State<StatefulWidget> createState() => _CellState();
}

class _CellState extends State<Cell> {
  @override
  Widget build(BuildContext context) {
    return InkWell(
      onTap: () {
        bool legal = checkLegality(position: widget.index, value: int.parse(widget.digit));
        if (legal) {
          print("LEGAL MOVE!");
        } else {
          print("ILLEGAL MOVE");
        }
        print("${widget.digit} ${widget.index}");
      },
      child: Container(
        color: const Color.fromARGB(255, 178, 195, 233),
        alignment: Alignment.center,
        child: !widget.digit.startsWith("0")
            ? Text(widget.digit,
                style: widget.size <= 9
                    ? const TextStyle(fontSize: 30)
                    : widget.size <= 16
                        ? const TextStyle(fontSize: 15)
                        : const TextStyle(fontSize: 6))
            // 30 or 9x9, 15 for 16x16 , 6 for anything else (for now at least)
            : const Text(""),
      ),
    );
  }
}
