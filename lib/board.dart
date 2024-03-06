import 'dart:math';

import 'package:flutter/material.dart';
import 'package:sudoku/cell.dart';
import 'package:sudoku/game_state.dart';

//TODO sudokuen bliver gemt som en string og der sker kun ændrig i lokal state
// når der klikkes på en celle.
// Det skal nok ændres til at der er en ordentlig ræpresentation af sudokuen
// i guien. Eventuelt skal alt informationen gemmes i den statiske GameState.
class Board extends StatefulWidget {
  const Board({super.key});

  @override
  State<StatefulWidget> createState() => _BoardState();
}

class _BoardState extends State<Board> {
  @override
  Widget build(BuildContext context) {
    GameState state = GameState.getInstance();

    return Center(
      child: SizedBox(
        height: 340,
        width: 340,
        child: Stack(
          alignment: Alignment.center,
          children: [
            Container(color: const Color.fromARGB(255, 19, 22, 54)),
            GridView.builder(
              padding: EdgeInsets.zero,
              itemCount: state.board.length,
              gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
                  crossAxisCount: state.size,
                  crossAxisSpacing: 2,
                  mainAxisSpacing: 2),
              itemBuilder: (context, index) {
                return Container(
                  color: const Color.fromARGB(255, 127, 132, 177),
                );
              },
            ),
            ListenableBuilder(
              listenable: state,
              builder: (ctx, _) => GridView.builder(
                physics: const NeverScrollableScrollPhysics(),
                padding: EdgeInsets.zero,
                itemCount: state.board.length,
                gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
                  crossAxisCount: state.size,
                  crossAxisSpacing: 2,
                  mainAxisSpacing: 2,
                ),
                itemBuilder: (ctx, index) {
                  return Cell(state.board[index], index, state.board.length);
                },
              ),
            )
          ],
        ),
      ),
    );
  }
}
