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

  GameState(String sudokuSource, this.xPositions) {
    board = sudokuSource
        .split(",")
        .takeWhile((str) => str.isNotEmpty)
        .map((n) => int.parse(n))
        .map((n) => n == 0 ? null : n)
        .toList();

    for (int i = 0; i < board.length; i++) {
      drafts.add([]);
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
  List<List<int>> drafts = [];
  List<(int, int)> xPositions;

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

  void changeDraft(int position) {
    if (drafts[position].contains(selectedDigit)) {
      drafts[position].remove(selectedDigit);
    } else {
      drafts[position].add(selectedDigit);
    }
    notifyListeners();
  }

  void setSelected(int newSelected) {
    selectedDigit = newSelected;
    if (selectedDigit == 0) {
      drafting = false;
    }
    notifyListeners();
  }

  void switchDrafting() {
    drafting = !drafting;
    if (selectedDigit == 0) {
      selectedDigit = 1;
    }
    notifyListeners();
  }

  bool digitDone(int n) {
    //TODO: Det her skal måske memoizes. Ingen grund til at gøre det ved HVER update
    return board.where((b) => b == n).length == size;
  }

  bool gameDone() {
    return board.every((n) => n != null);
  }

  bool drafting = false;
}
