import 'package:flutter/material.dart';

class Board extends StatefulWidget {
  @override
  State<StatefulWidget> createState() => _BoardState();
}

class _BoardState extends State<Board> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(
        child: SizedBox(
          height: 384,
          width: 420,
          child: Stack(
            children: [
              Container(color: Color.fromARGB(255, 19, 22, 54)),
              GridView.builder(
                padding: EdgeInsets.zero,
                gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
                    crossAxisCount: 3, crossAxisSpacing: 2, mainAxisSpacing: 2),
                itemBuilder: (context, index) {
                  return Container(
                    color: Color.fromARGB(255, 127, 132, 177),
                  );
                },
              ),
              GridView.builder(
                padding: EdgeInsets.zero,
                itemCount: 81,
                gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
                    crossAxisCount: 9, crossAxisSpacing: 2, mainAxisSpacing: 2),
                itemBuilder: (context, index) {
                  return Container(
                    color: Color.fromARGB(255, 178, 195, 233),
                    padding: EdgeInsets.all(10),
                    child: Text(index.toString()),
                  );
                },
              ),
            ],
          ),
        ),
      ),
    );
  }
}
