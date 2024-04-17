import 'package:flutter/material.dart';

class GameHeader extends StatelessWidget {
  final Set<String> rules;
  const GameHeader(this.rules, {super.key});

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: 100,
      width: 340,
      child: Column(
        children: [
          Row(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              const BackButton(),
              const SizedBox(
                width: 240,
              ),
              SizedBox(
                // Use width to change distance from the edge
                width: 40,
                child: InkWell(
                  onTap: () {
                    showDialog(
                      context: context,
                      builder: (context) => AlertDialog(
                        title: const Align(
                          alignment: Alignment.center,
                          child: Text("Rules"),
                        ),
                        content: IntrinsicHeight(
                          child: SizedBox(
                            height: 300,
                            child: RawScrollbar(
                              child: SingleChildScrollView(
                                child: Column(
                                  ,
                                  children: rules
                                      .map(
                                        (e) => e == "SquareRule"
                                            ? const Column(
                                                children: [
                                                  Text(
                                                    "Square Rule",
                                                    style:
                                                        TextStyle(fontSize: 20),
                                                  ),
                                                  Text(
                                                      "Cells in the same square must be unique\n")
                                                ],
                                              )
                                            : e == "KnightsMove"
                                                ? const Column(
                                                    children: [
                                                      Text(
                                                        "Knight Rule",
                                                        style: TextStyle(
                                                            fontSize: 20),
                                                      ),
                                                      Text(
                                                          "Cells, a chess knights move away from eachother, can not be the same number\n")
                                                    ],
                                                  )
                                                : e == "XRule"
                                                    ? const Column(
                                                        children: [
                                                          Text(
                                                            "X Rule",
                                                            style: TextStyle(
                                                                fontSize: 20),
                                                          ),
                                                          Text(
                                                              "Cells with an X between them, must add to the sudoku's size plus one\n")
                                                        ],
                                                      )
                                                    : e == "DiagonalRule"
                                                        ? const Column(
                                                            children: [
                                                              Text(
                                                                "Diagonal Rule",
                                                                style: TextStyle(
                                                                    fontSize:
                                                                        20),
                                                              ),
                                                              Text(
                                                                  "Cells must be unique from other cells on the same diagonal\n")
                                                            ],
                                                          )
                                                        : e == "ParityRule"
                                                            ? const Column(
                                                                children: [
                                                                  Text(
                                                                    "Parity Rule",
                                                                    style: TextStyle(
                                                                        fontSize:
                                                                            20),
                                                                  ),
                                                                  Text(
                                                                      "Cells with a circle between them must have different parity\n")
                                                                ],
                                                              )
                                                            : e == "ConsecutiveRule"
                                                                ? const Column(
                                                                    children: [
                                                                      Text(
                                                                        "Consecutive Rule",
                                                                        style: TextStyle(
                                                                            fontSize:
                                                                                20),
                                                                      ),
                                                                      Text(
                                                                          "Cells with a dot between them must be one apart\n")
                                                                    ],
                                                                  )
                                                                : e == "ZipperRule"
                                                                    ? const Column(
                                                                        children: [
                                                                          Text(
                                                                            "Zipper Rule",
                                                                            style:
                                                                                TextStyle(fontSize: 20),
                                                                          ),
                                                                          Text(
                                                                              "Zippers appears as pink lines with a circle in the center."
                                                                              "The two cells on the line with equal distance from the center, must add to the center\n")
                                                                        ],
                                                                      )
                                                                    : e == "ThermometerRule"
                                                                        ? const Column(
                                                                            children: [
                                                                              Text(
                                                                                "Thermometer Rule",
                                                                                style: TextStyle(fontSize: 20),
                                                                              ),
                                                                              Text("Thermometers appears as blue lines with a circle in one end."
                                                                                  "Cells starting at the dot must be in rising order\n")
                                                                            ],
                                                                          )
                                                                        : Text(e),
                                      )
                                      .toList(),
                                ),
                              ),
                            ),
                          ),
                        ),
                      ),
                    );
                  },
                  child: const Align(
                    alignment: Alignment.centerLeft,
                    child: Icon(Icons.info),
                  ),
                ),
              ),
            ],
          ),
          const Text("Standard Sudoku", style: TextStyle(fontSize: 30)),
        ],
      ),
    );
  }
}
