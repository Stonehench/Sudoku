import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_spinkit/flutter_spinkit.dart';
import 'package:sudoku/account.dart';
import 'package:sudoku/api.dart';
import 'package:http/http.dart' as http;

class Scoreboard extends StatefulWidget {
  final bool onlyYou;
  final bool asPage;
  const Scoreboard({super.key, this.onlyYou = false, this.asPage = true});

  @override
  State<Scoreboard> createState() => _ScoreboardState();
}

enum LoadingState { unstarted, loading, failed, success }

class _ScoreboardState extends State<Scoreboard> {
  LoadingState loadingState = LoadingState.unstarted;
  List<Score>? scoreboard;

  @override
  Widget build(BuildContext context) {
    Widget body;
    switch (loadingState) {
      case LoadingState.unstarted:
        () async {
          setState(() {
            loadingState = LoadingState.loading;
          });
          var res =
              await (widget.onlyYou ? getCurrentPlace() : getScoreBoard());
          setState(() {
            if (res == null) {
              loadingState = LoadingState.failed;
            } else {
              scoreboard = res;
              loadingState = LoadingState.success;
            }
          });
        }();
        body = const SpinKitCircle(
          color: Colors.white,
        );

      case LoadingState.loading:
        body = const SpinKitCircle(
          color: Colors.white,
        );

      case LoadingState.failed:
        body = const Center(
            child: Text(
                "Failed to fetch scoreboard. Check your internet connection"));
      case LoadingState.success:
        if (widget.onlyYou) {
          body = Column(
              //padding: const EdgeInsets.all(5),
              children: scoreboard!.map(scoreItem).toList());
        } else {
          body = ListView(
            padding: const EdgeInsets.all(5),
            children: scoreboard!.map(scoreItem).toList(),
          );
        }
    }
    if (widget.asPage) {
      return Scaffold(
        appBar: AppBar(
          title: const Text("Scoreboard"),
          actions: [
            TextButton(
                onPressed: () => Navigator.of(context).push(MaterialPageRoute(
                    builder: (context) => const AccountPage())),
                child: const Text("Account"))
          ],
        ),
        body: RefreshIndicator(
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
            child: Center(child: body)),
      );
    } else {
      print("BODY!!$body");
      return Center(child: body);
    }
  }

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

    var decoration = score.place == 1
        ? topDecorator
        : (score.place == scoreboard!.length ? botDecorator : normalDecorator);

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

  return allScores
      .take(index + 3)
      .toList()
      .reversed
      .take(4)
      .toList()
      .reversed
      .toList();
}
