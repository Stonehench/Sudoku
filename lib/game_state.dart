import 'dart:async';
import 'dart:math';

import 'package:flutter/foundation.dart';

class GameState extends ChangeNotifier {
  static GameState? _instance;

  static void setInstance(GameState newstate) {
    _instance = newstate;
  }

  static GameState getInstance() {
    return _instance!;
  }

  GameState(String sudokuSource) {
    board = sudokuSource.split(",").takeWhile((str) => str.isNotEmpty).map((n) => int.parse(n)).map((n) => n == 0? null : n).toList();
    size = sqrt(board.length).toInt();
  }

  @override
  void addListener(VoidCallback listener) {
    super.addListener(listener);

    Timer(const Duration(seconds: 1), () {notifyListeners();});
    //notifyListeners();
    
  }

  late final int size;

  int selectedDigit = 1;
  late List<int?> board;
}
