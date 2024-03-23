import 'dart:math';

import 'package:flutter/foundation.dart';
import 'package:sudoku/api.dart';
import 'package:sudoku/src/rust/api/simple.dart';
import 'package:http/http.dart' as http;

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
      var preGameDone = gameDone();

      board[position] = selectedDigit;

      if (!preGameDone && gameDone()) {
        trySubmitScore();
      }

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
    return board.where((b) => b == n).length == size;
  }

  bool gameDone() {
    return board.every((n) => n != null);
  }

  bool _scoreSubmitted = false;
  bool _scoreInAir = false;

  bool submitted() {
    return _scoreSubmitted;
  }

  bool scoreInAir() {
    return _scoreInAir;
  }
  //WTFF. Det her er en kæmpe mess.
  Future<int?> trySubmitScore() async {
    while (_scoreInAir) {
      //Block  while another request is flying
      await Future.delayed(const Duration(seconds: 1));
      print("Blocking request");
    }

    //TODO: Point fået af at vinde er hardcodet. Det skal være baseret på sværhedsgraden.
    int value = 3;
    if (_scoreSubmitted) {
      notifyListeners();
      return value;
    }

    _scoreInAir = true;
    Account? account = AccountState.instance().get();
    if (account != null) {
      //_scoreSubmitted = true;

      try {
        await http.post(serverAddress.resolve("/add_score"), body: {
          "user_id": account.userID,
          "value": value.toString(),
        });
        _scoreSubmitted = true;
        _scoreInAir = false;
        notifyListeners();
        return value;
      } catch (e) {
        print("Sumbission failed with $e");
        _scoreInAir = false;
        notifyListeners();
        return null;
      }
    }
    _scoreInAir = false;
  notifyListeners();
    return null;
  }

  bool drafting = false;
}
