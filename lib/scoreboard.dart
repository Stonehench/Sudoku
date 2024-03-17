import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_spinkit/flutter_spinkit.dart';
import 'package:sudoku/account.dart';
import 'package:sudoku/api.dart';
import 'package:http/http.dart' as http;

class Scoreboard extends StatefulWidget {
  const Scoreboard({super.key});

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
          var res = await getScoreBoard();
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
        body = ListView(
          padding: const EdgeInsets.all(5),
          children: scoreboard!.indexed.map(scoreItem).toList(),
        );
    }

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
      body: Center(child: body),
    );
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

  Widget scoreItem((int, Score) data) {
    var (index, score) = data;
    var place = index + 1;

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

    var decoration = place == 1
        ? topDecorator
        : (place == scoreboard!.length ? botDecorator : normalDecorator);

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
            child: Text("#$place", style: styleOfPlace(place, context)),
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
  const Score(this.username, this.value, this.you);

  @override
  String toString() {
    return "{username: $username, value: $value}";
  }
}

Future<List<Score>?> getScoreBoard() async {
  Account account = (await getAccount())!;
  try {
    var response = await http
        .get(serverAddress.resolve("/scoreboard?user_id=${account.userID}"));
    var jsonRes = jsonDecode(response.body);

    List<Score> scoreBoard = [];
    for (var score in jsonRes as List<dynamic>) {
      var scoreMap = score as Map<String, dynamic>;
      scoreBoard
          .add(Score(scoreMap["username"], scoreMap["value"], scoreMap["you"]));
    }
    return scoreBoard;
  } catch (e) {
    print("Scoreboard fetching failed with $e");
    return null;
  }
}
