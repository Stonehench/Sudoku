import 'package:flutter/material.dart';

// This is the widget in the info box that explains the rules.
List<Widget> getRules(Set<String> rules) {
  return [
    if (rules.contains("SquareRule")) ...[
      const Column(
        children: [
          Text(
            "Square Rule",
            style: TextStyle(fontSize: 20),
          ),
          Text("Cells in the same square must be unique\n")
        ],
      )
    ],
    if (rules.contains("KnightsMove")) ...[
      const Column(
        children: [
          Text(
            "Knight Rule",
            style: TextStyle(fontSize: 20),
          ),
          Text(
              "Cells, a chess knights move away from eachother, can not be the same number\n")
        ],
      )
    ],
    if (rules.contains("XRule")) ...[
      const Column(
        children: [
          Text(
            "X Rule",
            style: TextStyle(fontSize: 20),
          ),
          Text(
              "Cells with an X between them, must add to the sudoku's size plus one\n")
        ],
      )
    ],
    if (rules.contains("DiagonalRule")) ...[
      const Column(
        children: [
          Text(
            "Diagonal Rule",
            style: TextStyle(fontSize: 20),
          ),
          Text("Cells must be unique from other cells on the same diagonal\n")
        ],
      )
    ],
    if (rules.contains("ParityRule")) ...[
      const Column(
        children: [
          Text(
            "Parity Rule",
            style: TextStyle(fontSize: 20),
          ),
          Text("Cells with a circle between them must have different parity\n")
        ],
      )
    ],
    if (rules.contains("ConsecutiveRule")) ...[
      const Column(
        children: [
          Text(
            "Consecutive Rule",
            style: TextStyle(fontSize: 20),
          ),
          Text("Cells with a dot between them must be one apart\n")
        ],
      )
    ],
    if (rules.contains("ZipperRule")) ...[
      const Column(
        children: [
          Text(
            "Zipper Rule",
            style: TextStyle(fontSize: 20),
          ),
          Text("Zippers appears as pink lines with a circle in the center.\n"
              "The two cells on the line with equal distance from the center, must add to the center\n")
        ],
      )
    ],
    if (rules.contains("ThermometerRule")) ...[
      const Column(
        children: [
          Text(
            "Thermometer Rule",
            style: TextStyle(fontSize: 20),
          ),
          Text("Thermometers appears as blue lines with a circle in one end.\n"
              "Cells, starting at the dot, must be in rising order\n")
        ],
      )
    ]
  ];
}
