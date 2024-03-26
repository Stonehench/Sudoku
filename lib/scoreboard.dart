import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_spinkit/flutter_spinkit.dart';
import 'package:sudoku/account.dart';
import 'package:sudoku/api.dart';
import 'package:http/http.dart' as http;

class ScoreboardPage extends StatefulWidget {
  const ScoreboardPage({super.key});

  @override
  State<ScoreboardPage> createState() => _ScoreboardPageState();
}

enum LoadingState { unstarted, loading, failed, success }

class _ScoreboardPageState extends State<ScoreboardPage> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("Scoreboard"),
        actions: [
          TextButton(
              onPressed: () => Navigator.of(context).push(
                  MaterialPageRoute(builder: (context) => const AccountPage())),
              child: const Text("Account"))
        ],
      ),
      body: const ScoreboardEmbed(
        onlyYou: false,
      ),
    );
  }
}

class ScoreboardEmbed extends StatefulWidget {
  final bool onlyYou;

  const ScoreboardEmbed({super.key, required this.onlyYou});
  @override
  State<ScoreboardEmbed> createState() => _ScoreboardEmbedState();
}

class _ScoreboardEmbedState extends State<ScoreboardEmbed> {
  LoadingState loadingState = LoadingState.unstarted;
  List<Score>? scoreboard;

  TextStyle styleOfPlace(int place, BuildContext context) {
    if (place == 1) {
      return const TextStyle(
          color: Color.fromARGB(255, 255, 165, 0), fontSize: 30);
    } else if (place == 2) {
      return const TextStyle(
          color: Color.fromARGB(255, 192, 192, 192), fontSize: 25);
    } else if (place == 3) {
      return const TextStyle(
          color: Color.fromARGB(255, 205, 127, 50), fontSize: 20);
    } else {
      return const TextStyle(fontSize: 18);
    }
  }

  Widget scoreItem(Score score) {
    var topDecorator = BoxDecoration(
      borderRadius: const BorderRadius.only(
          topLeft: Radius.circular(10), topRight: Radius.circular(10)),
      color: Theme.of(context).focusColor,
    );
    var botDecorator = BoxDecoration(
      borderRadius: const BorderRadius.only(
          bottomLeft: Radius.circular(10), bottomRight: Radius.circular(10)),
      color: Theme.of(context).focusColor,
    );

    var normalDecorator = BoxDecoration(
      color: Theme.of(context).focusColor,
    );

    var decoration = scoreboard!.first == score
        ? topDecorator
        : (scoreboard!.last == score ? botDecorator : normalDecorator);

    var you = score.you ? [const Text("You!")] : [];

    return Container(
      decoration: decoration,
      margin: const EdgeInsets.all(5),
      height: 50,
      child: Row(
        children: [
          Container(
            width: 50,
            margin: const EdgeInsets.fromLTRB(10, 0, 0, 0),
            child: Text("#${score.place}",
                style: styleOfPlace(score.place, context)),
          ),
          Text(
            score.username,
            style: const TextStyle(fontSize: 18),
          ),
          const Spacer(),
          ...you,
          Container(
            padding: const EdgeInsets.fromLTRB(10, 0, 10, 0),
            child: Text(
              "${score.value}",
              style: const TextStyle(fontSize: 20),
            ),
          )
        ],
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return RefreshIndicator(
      onRefresh: () async {
        var res = await getScoreBoard();
        setState(() {
          if (res == null) {
            loadingState = LoadingState.failed;
          } else {
            scoreboard = res;
            loadingState = LoadingState.success;
          }
        });
      },
      child: Center(
        child: switch (loadingState) {
          LoadingState.unstarted => () {
              //Cringe, usejt, osv ..
              () async {
                setState(() {
                  loadingState = LoadingState.loading;
                });
                var res = await (widget.onlyYou
                    ? getCurrentPlace()
                    : getScoreBoard());
                setState(() {
                  if (res == null) {
                    loadingState = LoadingState.failed;
                  } else {
                    scoreboard = res;
                    loadingState = LoadingState.success;
                  }
                });
              }();
              return const SpinKitCircle(
                color: Colors.white,
              );
            }(),
          LoadingState.loading => const SpinKitCircle(
              color: Colors.white,
            ),
          LoadingState.failed => const Center(
              child: Text(
                  "Failed to fetch scoreboard. Check your internet connection")),
          LoadingState.success => widget.onlyYou
              ? Column(
                  //padding: const EdgeInsets.all(5),
                  children: scoreboard!.map(scoreItem).toList())
              : ListView(
                  padding: const EdgeInsets.all(5),
                  children: scoreboard!.map(scoreItem).toList(),
                ),
        },
      ),
    );
  }
}

class Score {
  final String username;
  final int value;
  final bool you;
  final int place;
  const Score(this.username, this.value, this.you, this.place);

  @override
  String toString() {
    return "{username: $username, value: $value}";
  }
}

Future<List<Score>?> getScoreBoard() async {
  Account? account = AccountState.instance().get();
  try {
    String userID;
    if (account != null) {
      userID = account.userID;
    } else {
      userID = "";
    }
    var response =
        await http.get(serverAddress.resolve("/scoreboard?user_id=$userID"));
    var jsonRes = jsonDecode(response.body);

    List<Score> scoreBoard = [];
    for (var score in jsonRes as List<dynamic>) {
      var scoreMap = score as Map<String, dynamic>;
      scoreBoard.add(Score(scoreMap["username"], scoreMap["value"],
          scoreMap["you"], scoreBoard.length + 1));
    }
    return scoreBoard;
  } catch (e) {
    print("Scoreboard fetching failed with $e");
    return null;
  }
}

Future<List<Score>?> getCurrentPlace() async {
  var allScores = await getScoreBoard();
  if (allScores == null) {
    return null;
  }

  int index = allScores.indexWhere((score) => score.you);

  // WTFFFF. Virker tho
  return allScores
      .take(index + 3)
      .toList()
      .reversed
      .take(4)
      .toList()
      .reversed
      .toList();
}
