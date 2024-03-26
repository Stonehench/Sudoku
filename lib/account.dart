import 'package:flutter/material.dart';
import 'package:sudoku/api.dart';
import 'package:sudoku/login_widget.dart';

class AccountPage extends StatelessWidget {
  const AccountPage({super.key});

  @override
  Widget build(BuildContext context) {
    AccountState? aState = AccountState.instance();

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
  @override
  Widget build(BuildContext context) {
    AccountState? aState = AccountState.instance();

    return ElevatedButton(
        onPressed: () => aState.logout(), child: const Text("Logout"));
  }
}
