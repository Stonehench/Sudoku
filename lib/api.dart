import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'package:shared_preferences/shared_preferences.dart';

Uri serverAddress = Uri.http("jensogkarsten.site");
//Uri serverAddress = Uri.http("localhost:5000");

class Account {
  final String username;
  final String userID;
  const Account(this.username, this.userID);
}

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
      _account = account;
      notifyListeners();
      return true;
    } catch (e) {
      _errorMsg = "Failed to connect to server";
      notifyListeners();
      return false;
    }
  }

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

  Future<void> logout() async {
    _account = null;
    await _prefs.clear();
    notifyListeners();
  }
}
