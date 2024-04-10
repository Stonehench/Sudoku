import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_spinkit/flutter_spinkit.dart';
import 'package:sudoku/game_state.dart';
import 'package:sudoku/game_view.dart';
import 'package:sudoku/src/rust/api/simple.dart';

class GameLoader extends StatefulWidget {
  final Set<String> rules;
  final String difficulty;
  final int size;
  const GameLoader(this.sudokuSource, this.rules, this.difficulty, this.size,
      {super.key});

  final Future<String?> sudokuSource;

  @override
  State<GameLoader> createState() => _GameLoaderState();
}

class _GameLoaderState extends State<GameLoader> {
  bool awaiting = false;
  int removed = 0;
  int? targetRemoved;
  @override
  Widget build(BuildContext context) {
    if (!awaiting) {
      awaiting = true;

      () async {
        var source = await widget.sudokuSource;
        if (source == null) {
          //I Dunno
          setState(() {
            Navigator.of(context)
                .pop("Failed to generate sudoku with these rules");
          });

          return;
        }
        var xPositions = await getXPositions();
        var parityPositions = await getParityPositions();
        var zipperPositions = await getZipperPositions();

        GameState.setInstance(GameState(source, xPositions, parityPositions, zipperPositions));
        setState(() {
          Navigator.of(context).pushReplacement(MaterialPageRoute(
            builder: (context) => GameView(widget.rules),
          ));
        });
      }();

      () async {
        var newTargetRemoved = await difficultyValues(
            size: widget.size, difficulty: widget.difficulty);
        if (newTargetRemoved != null) {
          if (mounted) {
            setState(() {
              targetRemoved = newTargetRemoved;
            });
          }
        } else {
          throw "Backend failed to parse GUI difficulty (${widget.difficulty}). This is really fucking weird";
        }
      }();
    }

    () async {
      var progress = await waitForProgess();
      if (progress != null) {
        if (mounted) {
          setState(() {
            removed = progress;
          });
        }
      } else {
        Timer(const Duration(milliseconds: 500), () {
          if (mounted) {
            setState(() {});
          }
        });
      }
    }();

    return Scaffold(
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            SpinKitWave(
              color: Theme.of(context).highlightColor,
            ),
            targetRemoved == null
                ? Text("$removed")
                : Text("$removed / $targetRemoved")
          ],
        ),
      ),
    );
  }
}
