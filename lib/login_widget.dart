// Author Thor s224817

import 'package:flutter/material.dart';
import 'package:sudoku/api.dart';

// Shows when user is not logged in, but wants to
class LoginWidget extends StatefulWidget {
  const LoginWidget({super.key});

  @override
  State<LoginWidget> createState() => _LoginWidgetState();
}

class _LoginWidgetState extends State<LoginWidget> {
  String? error;

  TextEditingController username = TextEditingController();
  TextEditingController password = TextEditingController();

  @override
  Widget build(BuildContext context) {
    AccountState aState = AccountState.instance();

    var buttonStyle = OutlinedButton.styleFrom(
      shape: const RoundedRectangleBorder(
          borderRadius: BorderRadius.all(Radius.circular(5))),
    );

    return Center(
      child: Column(children: [
        Container(
          margin: const EdgeInsets.all(20),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Container(
                margin: const EdgeInsets.all(5),
                child: const Text(
                  "Login / Register",
                  style: TextStyle(fontSize: 30),
                ),
              ),
              Container(
                margin: const EdgeInsets.all(5),
                child: TextField(
                  controller: username,
                  decoration: const InputDecoration(
                    border: OutlineInputBorder(),
                    labelText: 'Username',
                  ),
                ),
              ),
              Container(
                margin: const EdgeInsets.all(5),
                child: TextField(
                  controller: password,
                  obscureText: true,
                  decoration: const InputDecoration(
                    border: OutlineInputBorder(),
                    labelText: 'Password',
                  ),
                ),
              ),
              Container(
                margin: const EdgeInsets.all(5),
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    OutlinedButton(
                      style: buttonStyle,
                      onPressed: () async {
                        if (await aState.login(username.text, password.text)) {
                          setState(() {
                            Navigator.of(context).pop();
                          });
                        } else {
                          setState(() {
                            error = aState.getError();
                          });
                        }
                      },
                      child: Container(
                          margin: const EdgeInsets.fromLTRB(20, 5, 20, 5),
                          child: const Text("Login")),
                    ),
                    OutlinedButton(
                      style: buttonStyle,
                      onPressed: () async {
                        if (await aState.register(
                            username.text, password.text)) {
                          setState(() {
                            Navigator.of(context).pop();
                          });
                        } else {
                          setState(() {
                            error = aState.getError();
                          });
                        }
                      },
                      child: Container(
                        margin: const EdgeInsets.fromLTRB(20, 5, 20, 5),
                        child: const Text("Register"),
                      ),
                    ),
                  ],
                ),
              ),
              if (error != null) ...[
                Container(
                  margin: const EdgeInsets.fromLTRB(5, 0, 0, 0),
                  child: Text(error!),
                )
              ]
            ],
          ),
        ),
      ]),
    );
  }
}
