import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_spinkit/flutter_spinkit.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:sudoku/api.dart';
import 'package:uuid/uuid.dart';
import 'package:http/http.dart' as http;

class AccountPage extends StatefulWidget {
  const AccountPage({super.key});

  @override
  State<StatefulWidget> createState() {
    return _AccountPageState();
  }
}

class Account {
  static Account? _account;
  final String username;
  final String userID;
  const Account(this.username, this.userID);
}

enum AccountLoadState { unrequesed, loading, error, success, noAccount }

class _AccountPageState extends State<AccountPage> {
  String? error;

  AccountLoadState state = AccountLoadState.unrequesed;

  TextEditingController usernameController = TextEditingController();

  void register() async {
    var username = usernameController.text;
    var userId = await _getId();
    await http.get(serverAddress.resolve("/register/$userId/$username"));

    var newAcc = await getAccount();
    setState(() {
      Account._account = newAcc;
      state = AccountLoadState.success;
    });
  }

  @override
  Widget build(BuildContext context) {
    Widget body;

    switch (state) {
      case AccountLoadState.unrequesed:
        getAccount().then(
          (newAcc) => setState(() {
            if (newAcc != null) {
              Account._account = newAcc;
              state = AccountLoadState.success;
            } else {
              state = AccountLoadState.noAccount;

              //state = AccountLoadState.error;
              // error = "Failed to log into server";
            }
          }),
        );

        body = SpinKitWave(
          color: Theme.of(context).highlightColor,
        );
      case AccountLoadState.loading:
        body = SpinKitWave(
          color: Theme.of(context).highlightColor,
        );
      case AccountLoadState.error:
        body = Text(error!);
      case AccountLoadState.success:
        body = Container(
          padding: const EdgeInsets.all(20),
          child: Column(
            children: [
              Row(
                children: [
                  Text("username: ${Account._account!.username}"),
                  OutlinedButton(
                      onPressed: () async {
                        await logout();
                        setState(() {
                          state = AccountLoadState.unrequesed;
                        });
                      },
                      child: const Text("Logout"))
                ],
              )
            ],
          ),
        );
      case AccountLoadState.noAccount:
        body = Container(
          padding: const EdgeInsets.all(20),
          child: Column(
            children: [
              const Text(
                "Create account",
                style: TextStyle(fontSize: 30),
              ),
              TextField(
                decoration: const InputDecoration(
                  border: OutlineInputBorder(),
                  labelText: 'Username',
                ),
                controller: usernameController,
              ),
              const SizedBox(
                height: 10,
              ),
              Row(
                children: [
                  OutlinedButton(
                      onPressed: register, child: const Text("Create"))
                ],
              ),
            ],
          ),
        );
    }

    return Scaffold(
      appBar: AppBar(
        title: const Text("Account"),
      ),
      body: Center(child: body),
    );
  }
}

Future<void> logout() async {
  _userId = null;
  Account._account = null;

  SharedPreferences preferences = await SharedPreferences.getInstance();
  await preferences.clear();
}

String? _userId;

Future<Account?> getAccount() async {
  if (Account._account != null) {
    return Account._account;
  }
  var userId = await _getId();
  var getAccountUri = serverAddress.resolve("/login/$userId");

  var res = await http.get(getAccountUri);

  Map<String, dynamic> body = jsonDecode(res.body);
  if (body.containsKey("username")) {
    String username = body["username"];
    Account._account = Account(username, userId);
    return Account._account;
  } else {
    return null;
  }
}

Future<String> _getId() async {
  if (_userId != null) {
    return _userId!;
  }

  // Obtain shared preferences.
  final SharedPreferences prefs = await SharedPreferences.getInstance();

  var id = prefs.getString("uuid");

  if (id == null) {
    var generator = const Uuid();
    _userId = generator.v4();
    prefs.setString("uuid", _userId!);
  } else {
    _userId = id;
  }

  return _userId!;
}
