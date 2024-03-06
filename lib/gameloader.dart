import 'package:flutter/material.dart';
import 'package:flutter_spinkit/flutter_spinkit.dart';
import 'package:sudoku/game_state.dart';
import 'package:sudoku/game_view.dart';

class GameLoader extends StatefulWidget {
  const GameLoader(this.sudokuSource, {super.key});

  final Future<String?> sudokuSource;

  @override
  State<GameLoader> createState() => _GameLoaderState();
}

class _GameLoaderState extends State<GameLoader> {
  bool awaiting = false;

  @override
  Widget build(BuildContext context) {
    if (!awaiting) {
      awaiting = true;

      () async {
        var source = await widget.sudokuSource;
        if (source == null) {
          //I Dunno
          return;
        }

        GameState.setInstance(GameState(source));
        setState(() {
          Navigator.of(context).pushReplacement(MaterialPageRoute(
            builder: (context) => const GameView(),
          ));
        });
      }();
    }

    return Scaffold(
        body: Center(
            child: SpinKitWave(
      color: Theme.of(context).highlightColor,
    )));
  }
}
