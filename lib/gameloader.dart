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
  @override
  void initState() {
    super.initState();

    //Await finished generation
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
      var consecutivePositions = await getConsecutivePositions();

      GameState.setInstance(GameState(source, xPositions, parityPositions,
          consecutivePositions, zipperPositions));
      setState(() {
        Navigator.of(context).pushReplacement(MaterialPageRoute(
          builder: (context) => GameView(widget.rules),
        ));
      });
    }();

    progressSink = progress();
    progressSink.forEach((args) {
      setState(() {
        removed = args;
      });
    });
  }

  late Stream<(int, int)> progressSink;

  (int, int)? removed;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            SpinKitWave(
              color: Theme.of(context).highlightColor,
            ),
            () {
              if (removed != null) {
                var (status, target) = removed!;
                return Text("$status / $target");
              } else {
                return const Text("Me Thinkey");
              }
            }()
          ],
        ),
      ),
    );
  }
}
