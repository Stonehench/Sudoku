import 'package:flutter/material.dart';
import 'dart:math';

import 'package:flutter/services.dart';
import 'package:sudoku/game_state.dart';
import 'package:sudoku/gameloader.dart';
import 'package:sudoku/src/rust/api/simple.dart';

class Menu extends StatefulWidget {
  Menu({super.key});

  @override
  State<Menu> createState() => _MenuState();

  Set<String> gameModes = {};

  getGameRules() {
    return gameModes;
  }
}

class _MenuState extends State<Menu> {
  String sizeText = "9x9";
  int size = 9;
  final inputTextController = TextEditingController();

  void onTextChange(String newText) {
    int? newSize = int.tryParse(newText);
    if (newSize != null) {
      if (newSize == 0) {
        setState(() {
          sizeText = "Cannot create size 0";
        });
      } else {
        int sqrtSize = sqrt(newSize).toInt();
        String rounding;
        size = sqrtSize * sqrtSize;
        if (sqrtSize * sqrtSize != newSize) {
          rounding = " (Rounding down to $size)";
        } else {
          rounding = "";
        }
        setState(() {
          sizeText = "${size}x$size$rounding";
        });
      }
    }
  }

  //Set<String> gameModes = {};

  final List<(String, String, bool)> rules = [
    ("Square rule", "SquareRule", true),
    ("Knights move", "KnightsMove", false),
    ("X rule", "XRule", false),
    ("Diaginal rule", "DiagonalRule", false),
  ];

  bool initialized = false;

  List<Widget> ruleWidgets() {
    List<Widget> list = [];

    for (var (name, realname, def) in rules) {
      if (!initialized) {
        initialized = true;
        if (def) {
          widget.gameModes.add(realname);
        }
      }

      list.add(
        Row(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Text(name),
            Checkbox(
              value: widget.gameModes.contains(realname),
              onChanged: (v) {
                setState(() {
                  if (v == true) {
                    widget.gameModes.add(realname);
                  } else {
                    widget.gameModes.remove(realname);
                  }
                });
              },
            ),
          ],
        ),
      );
    }

    return list;
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Sudoku!')),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            SizedBox(
              width: 250,
              child: TextField(
                onChanged: onTextChange,
                controller: inputTextController,
                keyboardType: TextInputType.number,
                inputFormatters: <TextInputFormatter>[
                  FilteringTextInputFormatter.digitsOnly
                ],
                decoration: const InputDecoration(
                  border: OutlineInputBorder(),
                  labelText: 'Sudoku size',
                ),
              ),
            ),
            Text(sizeText),
            TextButton(
              onPressed: () {
                setState(() {
                  sizeText = "${size}x$size";
                });

                Future<String?> sudokuSource = generateWithSize(
                    size: size, rulesSrc: widget.gameModes.toList());
                //inputTextController.clear();
                () async {
                  var rulesAsString =
                      widget.gameModes.fold("", (prev, e) => prev + e + "\n");
                  var res = await Navigator.of(context).push(
                    MaterialPageRoute(
                      builder: (context) =>
                          GameLoader(sudokuSource, rulesAsString),
                    ),
                  );
                  if (res != null) {
                    setState(() {
                      sizeText = res.toString();
                    });
                  }
                }();
              },
              child: const Text('Create Sudoku'),
            ),
            Wrap(
              spacing: 20,
              crossAxisAlignment: WrapCrossAlignment.center,
              children: ruleWidgets(),
            )
          ],
        ),
      ),
    );
  }
}
