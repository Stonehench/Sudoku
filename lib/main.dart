// Author Thor s224817
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:sudoku/api.dart';
import 'package:sudoku/menu.dart';
import 'package:sudoku/src/rust/api/simple.dart';
import 'package:sudoku/src/rust/frb_generated.dart';

Future<void> main() async {
  //Initialize rust 
  await RustLib.init();

  //Initialize localstorage
  WidgetsFlutterBinding.ensureInitialized();
  await AccountState.initialize();

  //Prevents landscape mode on phones.
  SystemChrome.setPreferredOrientations([
    DeviceOrientation.portraitUp,
  ]);
  runApp(const SudokuApp());
}

class SudokuApp extends StatefulWidget {
  const SudokuApp({super.key});

  @override
  State<SudokuApp> createState() => _SudokuAppState();
}

class _SudokuAppState extends State<SudokuApp> {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      theme: ThemeData.dark(),  //Uses dark theme!!
      initialRoute: '/menu',
      routes: {
        '/menu': (context) => const Menu(),
      },
    );
  }

  @override
  void dispose() {
    super.dispose();
    closeThreads();
  }
}
