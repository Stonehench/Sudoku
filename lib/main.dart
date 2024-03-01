
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

  final String tempSudoku =
      "0,2,0,6,0,8,0,0,0,5,8,0,0,0,9,7,0,0,0,0,0,0,4,0,0,0,0,3,7,0,0,0,0,5,0,0,6,0,0,0,0,0,0,0,4,0,0,8,0,0,0,0,1,3,0,0,0,0,2,0,0,0,0,0,0,9,8,0,0,0,3,6,0,0,0,3,0,6,0,9,0";

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      initialRoute: '/menu',
      routes: {
        '/menu': (context) => const Menu(),
        '/board': (context) => Board(tempSudoku),
      },
    );
  }
}
