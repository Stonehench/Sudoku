import 'package:flutter/material.dart';
import 'package:sudoku/menu.dart';
import 'package:sudoku/src/rust/frb_generated.dart';
import 'board.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const SudokuApp());
}

class SudokuApp extends StatelessWidget {
  const SudokuApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      initialRoute: '/menu',
      routes: {
        '/menu': (context) => const Menu(),
        '/board': (context) =>
            Board(ModalRoute.of(context)!.settings.arguments),
      },
    );
  }
}
