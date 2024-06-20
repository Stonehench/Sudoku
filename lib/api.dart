// Author Thor s224817

import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'package:shared_preferences/shared_preferences.dart';

Uri serverAddress = Uri.http("jensogkarsten.site");

class Account {
  final String username;
  final String userID;
  final int? streak;
  final double? multiplier;
  const Account(this.username, this.userID, {this.streak, this.multiplier});
}

//This class contains the currect state of being logged in.
class AccountState extends ChangeNotifier {
  AccountState(this._prefs);

  static AccountState? _instance;

  Account? _account;
  String? _errorMsg;
  final SharedPreferences _prefs;

  static Future<void> initialize() async {
    var prefs = await SharedPreferences.getInstance();
    _instance = AccountState(prefs);
  }

  static AccountState instance() {
    return _instance!;
  }

  String? getError() {
    return _errorMsg;
  }

  Account? get() {
    if (_account != null) {
      return _account;
    }

    var maybeUsername = _prefs.getString("username");
    var maybeUserId = _prefs.getString("userID");

    if (maybeUsername != null && maybeUserId != null) {
      _account = Account(maybeUsername, maybeUserId);
      notifyListeners();
      return _account;
    } else {
      return null;
    }
  }

  // This function sends the users login information to the server and checks whether it exists
  Future<bool> login(String username, String password) async {
    try {
      var response = await http.post(serverAddress.resolve("/login"), body: {
        "username": username,
        "password": password,
      });
      if (response.statusCode != 200) {
        _errorMsg = "Invalid username or password";
        notifyListeners();
        return false;
      }

      var body = jsonDecode(response.body);

      Account account = Account(username, body["user_id"]);
      await _prefs.setString("username", account.username);
      await _prefs.setString("userID", account.userID);

      await updateStreak();

      _account = account;
      notifyListeners();
      return true;
    } catch (e) {
      _errorMsg = "Failed to connect to server";
      notifyListeners();
      return false;
    }
  }

  // This function creates a new user in the database if the username is unique
  Future<bool> register(String username, String password) async {
    try {
      var response = await http.post(serverAddress.resolve("/register"), body: {
        "username": username,
        "password": password,
      });
      if (response.statusCode != 200) {
        _errorMsg = "Username taken"; //Probably
        notifyListeners();
        return false;
      }
      var body = jsonDecode(response.body);
      Account account = Account(username, body["user_id"]);
      await _prefs.setString("username", account.username);
      await _prefs.setString("userID", account.userID);
      _account = account;
      notifyListeners();
      return true;
    } catch (e) {
      _errorMsg = "Failed to connect to server";
      notifyListeners();
      return false;
    }
  }

  // This function adds points to the users existing score
  Future<void> addScore(Account account, int value) async {
    try {
      var response =
          await http.post(serverAddress.resolve("/add_score"), body: {
        "user_id": account.userID,
        "value": value,
      });

      if (response.statusCode != 200) {
        throw "Failed with err ${response.statusCode}";
      }
    } catch (e) {
      throw "Failed to connect to server";
    }
  }

  // This function changes the users existing password
  Future<bool> changePasswd(Account account, String newPasswd) async {
    try {
      var response =
          await http.post(serverAddress.resolve("/change_passwd"), body: {
        "user_id": account.userID,
        "password": newPasswd,
      });

      return response.statusCode == 200;
    } catch (e) {
      return false;
    }
  }

  // This function updates the streak of the user.
  Future<bool> updateStreak() async {
    Account? acc = get();
    if (acc == null) {
      return false;
    }

    try {
      var response = await http.post(serverAddress.resolve("/streak"), body: {
        "user_id": _account!.userID,
      });

      var jsonMap = jsonDecode(response.body) as Map<String, dynamic>;

      Account newAccount = Account(
        acc.username,
        acc.userID,
        streak: jsonMap["streak"] as int,
        multiplier: jsonMap["multiplier"] as double,
      );

      if (newAccount != _account) {
        _account = newAccount;
        notifyListeners();
      }

      return true;
    } catch (e) {
      return false;
    }
  }

  // logs the user out
  Future<void> logout() async {
    _account = null;
    await _prefs.clear();
    notifyListeners();
  }
}
