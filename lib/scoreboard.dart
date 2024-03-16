import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_spinkit/flutter_spinkit.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:uuid/uuid.dart';
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
        body = Column(
          children: scoreboard!.map(scoreItem).toList(),
        );
    }

    return Scaffold(
      appBar: AppBar(),
      body: Center(child: body),
    );
  }

  Widget scoreItem(Score score) {
    return Text("${score.username} : ${score.value}");
  }
}

class Score {
  final String username;
  final int value;
  const Score(this.username, this.value);

  @override
  String toString() {
    return "{username: $username, value: $value}";
  }
}

//Uri serverAddress = Uri.https("jensogkarsten.site/");
Uri serverAddress = Uri.http("localhost:5000");

Future<List<Score>?> getScoreBoard() async {
  try {
    var response = await http.get(serverAddress);
    var jsonRes = jsonDecode(response.body);

    List<Score> scoreBoard = [];
    for (var score in jsonRes as List<dynamic>) {
      var scoreMap = score as Map<String, dynamic>;
      scoreBoard.add(Score(scoreMap["username"], scoreMap["value"]));
    }
    return scoreBoard;
  } catch (e) {
    print("Scoreboard fetching failed with $e");
    return null;
  }
}

String? _id;

Future<String> _get_id() async {
  if (_id != null) {
    return _id!;
  }

  // Obtain shared preferences.
  final SharedPreferences prefs = await SharedPreferences.getInstance();

  var id = prefs.getString("uuid");

  if (id == null) {
    var generator = const Uuid();
    _id = generator.v4();
    prefs.setString("uuid", _id!);
  } else {
    _id = id;
  }

  return _id!;
}
