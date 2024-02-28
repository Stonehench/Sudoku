import 'dart:math';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:sudoku/menu.dart';
import 'package:sudoku/src/rust/frb_generated.dart';
import 'board.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(SudokuApp());
}

class SudokuApp extends StatelessWidget {
  SudokuApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      initialRoute: '/menu',
      routes: {
        '/menu': (context) => Menu(),
        '/board': (context) => Board(),
      },
    );
  }
}
