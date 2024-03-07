import 'package:flutter/material.dart';
import 'package:flutter_spinkit/flutter_spinkit.dart';
import 'package:sudoku/game_state.dart';
import 'package:sudoku/game_view.dart';
import 'package:sudoku/src/rust/api/simple.dart';

class GameLoader extends StatefulWidget {
  const GameLoader(this.sudokuSource, {super.key});

  final Future<String?> sudokuSource;

  @override
  State<GameLoader> createState() => _GameLoaderState();
}

class _GameLoaderState extends State<GameLoader> {
  bool awaiting = false;
  int removed = 0;

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

        GameState.setInstance(GameState(source));
        setState(() {
          Navigator.of(context).pushReplacement(MaterialPageRoute(
            builder: (context) => const GameView(),
          ));
        });
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
            Text("$removed / 55")
          ],
        ),
      ),
    );
  }
}
