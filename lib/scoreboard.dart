// Author Thor s224817

import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_spinkit/flutter_spinkit.dart';
import 'package:sudoku/account.dart';
import 'package:sudoku/api.dart';
import 'package:http/http.dart' as http;

//The scorboard page. 
//Uses the scorboard embed which can be embedded other places if needed.
class ScoreboardPage extends StatefulWidget {
  const ScoreboardPage({super.key});

  @override
  State<ScoreboardPage> createState() => _ScoreboardPageState();
}

class _ScoreboardPageState extends State<ScoreboardPage> {
  @override
  Widget build(BuildContext context) {
    Key userKey;
    Account? account = AccountState.instance().get();
    if (account == null) {
      userKey = const Key("null");
    } else {
      userKey = Key(account.userID);
    }

    return Scaffold(
      appBar: AppBar(
        title: const Text("Scoreboard"),
        actions: [
          TextButton(
              onPressed: () async {
                await Navigator.of(context).push(MaterialPageRoute(
                    builder: (context) => const AccountPage()));
                setState(() {
                  //Force rebuild
                });
              },
              child: const Text("Account"))
        ],
      ),
      body: ScoreboardEmbed(
        key: userKey,
        onlyYou: false,
      ),
    );
  }
}

enum LoadingState { unstarted, loading, failed, success }

//The scoreboard embed. OnlyYou specifies whether to display all the users or just
// the current user and the one above and below.
class ScoreboardEmbed extends StatefulWidget {
  final bool onlyYou;

  const ScoreboardEmbed({super.key, required this.onlyYou});
  @override
  State<ScoreboardEmbed> createState() => _ScoreboardEmbedState();
}

class _ScoreboardEmbedState extends State<ScoreboardEmbed> {
  LoadingState loadingState = LoadingState.unstarted;
  List<Score>? scoreboard;
  //First, second and third places have different styles.
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
  //Builds a single persons score widget.
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

    var lasthtxt = "${score.lasth} / hour";

    var lasth = score.lasth != 0 ? [const Spacer(), Text(lasthtxt, style: const TextStyle(color: Colors.green),)] : [];

    var addons = score.you && !widget.onlyYou ? [const Text("You!"), ...lasth] : lasth;

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
          ...addons,
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
    return RefreshIndicator( // Reload on refresh
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

      // Uses a state machine to fetch the scoreboard.
      // This correctly handels internet failure.
      child: Center(
        child: switch (loadingState) {
          LoadingState.unstarted => () {
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

// Data class
class Score {
  final String username;
  final int value;
  final bool you;
  final int place;
  final int lasth;
  const Score(this.username, this.value, this.you, this.place, this.lasth);

  @override
  String toString() {
    return "{username: $username, value: $value}";
  }
}
//Tries to fetch the scoreboard from the server
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
      print(scoreMap);
      scoreBoard.add(Score(scoreMap["username"], scoreMap["value"],
          scoreMap["you"], scoreBoard.length + 1, scoreMap["lasth"]));
    }
    return scoreBoard;
  } catch (e) {
    print("Scoreboard fetching failed with $e");
    return null;
  }
}
//Given the scoreboard, get the current users location on it.
Future<List<Score>?> getCurrentPlace() async {
  var allScores = await getScoreBoard();
  if (allScores == null) {
    return null;
  }

  int index = allScores.indexWhere((score) => score.you);

  if (index == 0) {
    return allScores.take(3).toList();
  } else if (index + 1 == allScores.length) {
    return allScores.indexed
        .skipWhile((value) => fst(value) + 2 < index)
        .map(scd)
        .take(3)
        .toList();
  } else {
    return allScores.indexed
        .skipWhile((value) => fst(value) + 1 < index)
        .map(scd)
        .take(3)
        .toList();
  }
}
// Utility functions to extract value from touples.
T1 fst<T1, T2>((T1, T2) t) {
  var (t1, _) = t;
  return t1;
}

T2 scd<T1, T2>((T1, T2) t) {
  var (_, t2) = t;
  return t2;
}
