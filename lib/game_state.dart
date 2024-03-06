import 'dart:math';

import 'package:flutter/foundation.dart';
import 'package:sudoku/src/rust/api/simple.dart';

class GameState extends ChangeNotifier {
  static GameState? _instance;

  static void setInstance(GameState newstate) {
    _instance = newstate;
  }

  static GameState getInstance() {
    return _instance!;
  }

  GameState(String sudokuSource) {
    board = sudokuSource
        .split(",")
        .takeWhile((str) => str.isNotEmpty)
        .map((n) => int.parse(n))
        .map((n) => n == 0 ? null : n)
        .toList();
    for (int i = 0; i < board.length; i++) {
      if (board[i] != null) {
        initialClues.add(i);
      }
    }

    size = sqrt(board.length).toInt();
  }

  late final int size;

  int selectedDigit = 1;
  late List<int?> board;
  List<int> initialClues = [];

  Future<bool> updateDigit(int position) async {
    if (selectedDigit == 0) {
      board[position] = null;
      notifyListeners();
      return true;
    }

    if (await checkLegality(position: position, value: selectedDigit)) {
      board[position] = selectedDigit;
      notifyListeners();
      return true;
    }
    return false;
  }

  void setSelected(int newSelected) {
    selectedDigit = newSelected;
    notifyListeners();
  }
}
