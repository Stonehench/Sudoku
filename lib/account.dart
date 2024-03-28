import 'package:flutter/material.dart';
import 'package:sudoku/api.dart';
import 'package:sudoku/login_widget.dart';
import 'package:sudoku/scoreboard.dart';

class AccountPage extends StatelessWidget {
  const AccountPage({super.key});

  @override
  Widget build(BuildContext context) {
    AccountState aState = AccountState.instance();

    var body = ListenableBuilder(
      listenable: aState,
      builder: (context, child) =>
          aState.get() == null ? const LoginWidget() : const LoggedInWidget(),
    );

    return Scaffold(
      appBar: AppBar(title: const Text("Account")),
      body: body,
    );
  }
}

class LoggedInWidget extends StatefulWidget {
  const LoggedInWidget({super.key});

  @override
  State<LoggedInWidget> createState() => _LoggedInWidgetState();
}

class _LoggedInWidgetState extends State<LoggedInWidget> {
  TextEditingController passwordController = TextEditingController();

  String? changePasswdStatus;

  @override
  Widget build(BuildContext context) {
    AccountState aState = AccountState.instance();
    Account? account = aState.get();
    if (account == null) {
      return const Text("Somehow, account is null!");
    }

    var buttonStyle = ElevatedButton.styleFrom(
      shape: const RoundedRectangleBorder(
          borderRadius: BorderRadius.all(Radius.circular(5))),
    );

    return Container(
      padding: const EdgeInsets.all(10),
      child: ListView(
        children: [
          Center(
            child: Column(
              children: [
                Text(
                  "Hello, ${account.username}",
                  style: const TextStyle(fontSize: 30),
                ),
                const ScoreboardEmbed(onlyYou: true)
              ],
            ),
          ),
          Container(
            padding: const EdgeInsets.all(5),
            color: Theme.of(context).dialogBackgroundColor,
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Text("Change password", style: TextStyle(fontSize: 20)),
                Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    SizedBox(
                      width: 220,
                      child: TextField(
                        obscureText: true,
                        decoration:
                            const InputDecoration(hintText: "New Password"),
                        controller: passwordController,
                      ),
                    ),
                    ElevatedButton(
                      style: buttonStyle,
                      onPressed: () async {
                        var res = await aState.changePasswd(
                            account, passwordController.text);
                        setState(() {
                          if (res) {
                            changePasswdStatus = "Password changed";
                          } else {
                            changePasswdStatus = "Failed to change password";
                          }
                        });
                      },
                      child: const Text("change"),
                    ),
                  ],
                ),
                if (changePasswdStatus != null) ...[
                  Text(changePasswdStatus!),
                ]
              ],
            ),
          ),
          Container(
            padding: const EdgeInsets.all(5),
            child: ElevatedButton(
                style: buttonStyle,
                onPressed: () => aState.logout(),
                child: const Text("Logout")),
          )
        ],
      ),
    );
  }
}
