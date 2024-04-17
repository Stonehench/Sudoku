import 'dart:math';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:sudoku/api.dart';
import 'package:sudoku/src/rust/api/hint.dart';
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

  GameState(
      String sudokuSource,
      this.xPositions,
      this.parityPositions,
      this.consecutivePositions,
      this.zipperPositions,
      this.thermometerPositions,
      {this.daily}) {
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
    addListener(_trySubmitScore);
  }
  String? daily;
  late final int size;

  int selectedDigit = 1;
  int lives = 3;
  late List<int?> board;
  List<int> initialClues = [];
  List<List<int>> drafts = [];
  List<(int, int)> xPositions;
  List<(int, int)> parityPositions;
  List<(int, int)> consecutivePositions;
  List<(int, List<(int, int)>)> zipperPositions;
  List<List<int>> thermometerPositions;

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
    loseLife();
    return false;
  }


  void loseLife() {
    lives--;
    if (lives == 0) {
      print("Game over"); // handle loss of lives
    }
    notifyListeners();
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

  bool _gameDone() {
    return board.every((n) => n != null);
  }

  int? tryGetScore() {
    return _submittedScore;
  }

  ScoreSubmissionStatus _scoreSubmitStatus = ScoreSubmissionStatus.gameNotDone;
  int? _submittedScore;

  ScoreSubmissionStatus scoreStatus() {
    return _scoreSubmitStatus;
  }

  void retryScoreSubmit() {
    //Retry not allowed if status is already submitted.
    switch (_scoreSubmitStatus) {
      //Only allow retry if status is noAccount,noWifi or serverErrir.
      case ScoreSubmissionStatus.noAccount:
      case ScoreSubmissionStatus.noWifi:
      case ScoreSubmissionStatus.serverError:
        _scoreSubmitStatus = ScoreSubmissionStatus.unSubmitted;
        notifyListeners();

      default:
        return;
    }
  }

  int? serverErrorStatus;

  // Vi laver det til en state machine type beat tænker jeg
  //Denne funktion bliver kaldt ved hver notifyListeners()
  void _trySubmitScore() async {
    if (!_gameDone()) {
      return;
    }
    if (_scoreSubmitStatus == ScoreSubmissionStatus.gameNotDone) {
      _scoreSubmitStatus = ScoreSubmissionStatus.unSubmitted;
    }
    switch (_scoreSubmitStatus) {
      case ScoreSubmissionStatus.inAir:
        return;
      case ScoreSubmissionStatus.gameNotDone:
        return;
      case ScoreSubmissionStatus.noAccount:
        return;
      case ScoreSubmissionStatus.noWifi:
        return;
      case ScoreSubmissionStatus.submitted:
        return;
      case ScoreSubmissionStatus.serverError:
        return;
      case ScoreSubmissionStatus.unSubmitted:
        int value = (size * board.length);
        if (initialClues.isNotEmpty) {
          value = (size * board.length) ~/ initialClues.length;
        }

        Account? account = AccountState.instance().get();
        if (account == null) {
          _scoreSubmitStatus = ScoreSubmissionStatus.noAccount;
          notifyListeners();
          return;
        }
        if (account.multiplier != null) {
          value = (value * account.multiplier!).toInt();
        }

        try {
          _scoreSubmitStatus = ScoreSubmissionStatus.inAir;
          notifyListeners();

          var response =
              await http.post(serverAddress.resolve("/add_score"), body: {
            "user_id": account.userID,
            "value": value.toString(),
            if (daily != null) ...{"daily_dato": daily!}
          });

          if (response.statusCode != 200) {
            _scoreSubmitStatus = ScoreSubmissionStatus.serverError;
            serverErrorStatus = response.statusCode;
            notifyListeners();
            return;
          }
          _submittedScore = value;
          _scoreSubmitStatus = ScoreSubmissionStatus.submitted;
          notifyListeners();
          return;
        } catch (e) {
          _scoreSubmitStatus = ScoreSubmissionStatus.noWifi;
          notifyListeners();
          return;
        }
    }
  }

  void getHint() async {
    List<int> hints = [];
    for (int i = 0; i < board.length; i++) {
      if (board[i] == null) {
        hints.add(i);
      }
    }
    Future<(int, int)?> clues = hint(freeIndexes: hints);
    var clue = await clues;
    print(clue);
    if (clue != null) {
      board[clue.$2] = clue.$1;
    }
    notifyListeners();
  }

  bool drafting = false;
}

enum ScoreSubmissionStatus {
  gameNotDone,
  unSubmitted,
  noAccount,
  noWifi,
  inAir,
  submitted,
  serverError,
}
