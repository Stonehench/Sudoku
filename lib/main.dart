import 'package:flutter/material.dart';
import 'package:sudoku/game_view.dart';
import 'package:sudoku/menu.dart';
import 'package:sudoku/src/rust/api/simple.dart';
import 'package:sudoku/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const SudokuApp());
}

class SudokuApp extends StatelessWidget {
  const SudokuApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      theme: ThemeData.dark(),
      initialRoute: '/menu',
      routes: {
        '/menu': (context) => const Menu(),
        '/board': (context) => const GameView(),
      },
    );
  }

  @override
  void dispose() {
    closeThreads();
  }
}
