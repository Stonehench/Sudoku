// Author Thor s224817
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:home_widget/home_widget.dart';
import 'package:sudoku/api.dart';
import 'package:sudoku/appversion.dart';
import 'package:sudoku/menu.dart';
import 'package:sudoku/src/rust/api/simple.dart';
import 'package:sudoku/src/rust/frb_generated.dart';

Future<void> main() async {
  isOutdatedFuture = checkIsOutdated();
  //Initialize rust
  await RustLib.init();

  //Initialize localstorage
  WidgetsFlutterBinding.ensureInitialized();
  await AccountState.initialize();

  //Prevents landscape mode on phones.
  SystemChrome.setPreferredOrientations([
    DeviceOrientation.portraitUp,
  ]);

  AccountState.instance().addListener(() async {
    Account? account = AccountState.instance().get();
    if (account != null) {
      await HomeWidget.saveWidgetData<int>("acc_streak", account.streak ?? 0);
      await HomeWidget.saveWidgetData<String>(
          "acc_multiplier_str", (account.multiplier ?? 0.0).toStringAsFixed(2));
      await HomeWidget.updateWidget(name: "StreakWidget");
    }
  });

  runApp(SudokuApp());
}

Future<bool>? isOutdatedFuture;

class SudokuApp extends StatefulWidget {
  const SudokuApp({super.key});

  @override
  State<SudokuApp> createState() => _SudokuAppState();
}

class _SudokuAppState extends State<SudokuApp> {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      theme: ThemeData.dark(), //Uses dark theme!!
      home: const Menu(),
    );
  }

  @override
  void dispose() {
    super.dispose();
    closeThreads();
  }
}
