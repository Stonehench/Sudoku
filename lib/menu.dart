import 'dart:convert';

import 'package:flutter/material.dart';
import 'dart:math';
import 'package:http/http.dart' as http;
import 'package:flutter/services.dart';
import 'package:flutter_spinkit/flutter_spinkit.dart';
import 'package:sudoku/api.dart';
import 'package:sudoku/game_state.dart';
import 'package:sudoku/game_view.dart';
import 'package:sudoku/gameloader.dart';
import 'package:sudoku/scoreboard.dart';
import 'package:sudoku/src/rust/api/simple.dart';

class Menu extends StatefulWidget {
  const Menu({super.key});

  @override
  State<Menu> createState() => _MenuState();
}

class _MenuState extends State<Menu> {
  Set<String> gameModes = {};

  String sizeText = "9x9";
  int size = 9;
  final inputTextController = TextEditingController();

  // Function for updating text and size base textfield further down
  void onTextChange(String newText) {
    int? newSize = int.tryParse(newText);
    if (newSize != null) {
      if (newSize == 0) {
        setState(() {
          sizeText = "Cannot create size 0";
        });
      } else if (newSize < 4) {
        size = 4;
        setState(() {
          sizeText = "4x4 (Size too small rounding up)";
        });
      } else {
        int sqrtSize = sqrt(newSize).toInt();
        String rounding;
        size = sqrtSize * sqrtSize;
        if (sqrtSize * sqrtSize != newSize) {
          rounding = " (Rounding down to $size)";
        } else {
          rounding = "";
        }
        setState(() {
          sizeText = "${size}x$size$rounding";
        });
      }
    }
  }

  // Name and state of rules, for checkboxes further down
  final List<(String, String, bool)> rules = [
    ("Square rule", "SquareRule", true),
    ("Knights move", "KnightsMove", false),
    ("X rule", "XRule", false),
    ("Diaginal rule", "DiagonalRule", false),
    ("Parity Domino", "ParityRule", false),
    ("Consecutive", "ConsecutiveRule", false),
    ("Zippers", "ZipperRule", false),
    ("Thermometers", "ThermometerRule", false)
  ];

  bool initialized = false;

  // Function for adding rule checkboxes
  List<Widget> ruleWidgets() {
    List<Widget> list = [];

    // Extraction of real name for displaying
    for (var (name, realname, def) in rules) {
      if (!initialized) {
        initialized = true;
        if (def) {
          gameModes.add(realname);
        }
      }
      // Adds the rule names and checkboxes to
      list.add(
        SizedBox(
          width: 140,
          child: Row(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Text(name),
              const Spacer(
                flex: 1,
              ),
              Checkbox(
                value: gameModes.contains(realname),
                onChanged: (v) {
                  setState(() {
                    if (v == true) {
                      gameModes.add(realname);
                    } else {
                      gameModes.remove(realname);
                    }
                  });
                },
              ),
            ],
          ),
        ),
      );
    }

    return list;
  }

  // hide size selection box by default
  bool showSizeBox = false;

  // initial selection of size
  List<bool> sizeValueStates = <bool>[
    false,
    true,
    false,
    false,
  ];

  // Values for size selection
  List<int> sizeValues = [
    4,
    9,
    16,
    0,
  ];

  // Get size from text
  getSizeFromText() {
    if (inputTextController.text == "") {
      return 4;
    } else {
      return int.parse(inputTextController.text);
    }
  }

  // Selection buttons for size selection
  Widget sudokuSizeWidget() {
    return ToggleButtons(
      direction: Axis.horizontal,
      onPressed: (int index) {
        setState(() {
          // Set size and control visibility of size selection box
          if (index == 3) {
            showSizeBox = true;
            for (int i = 0; i < sizeValueStates.length; i++) {
              sizeValueStates[i] = i == index;
            }
            size = 4;
            onTextChange(getSizeFromText().toString());
          } else {
            showSizeBox = false;
            for (int i = 0; i < sizeValueStates.length; i++) {
              sizeValueStates[i] = i == index;
              if (i == index) {
                size = sizeValues[index];
                onTextChange(pow((index + 2), 2).toString());
              }
            }
          }
        });
      },
      // Visual configurations
      borderRadius: const BorderRadius.all(Radius.circular(8)),
      borderColor: Colors.transparent,
      constraints: const BoxConstraints(
        minHeight: 40.0,
        minWidth: 80.0,
      ),
      isSelected: sizeValueStates,
      children: sizeValues
          .map((e) => e == 0 ? const Icon(Icons.add) : Text(e.toString()))
          .toList(),
    );
  }

  // Initial difficulty
  String gameDifficulty = "Medium";

  // Initial selection of difficulty
  List<bool> difficulitiesValues = <bool>[
    false,
    true,
    false,
    false,
  ];

  // Difficulty options
  List<String> difficulitiesNames = [
    "Easy",
    "Medium",
    "Hard",
    "Expert",
  ];

  // Selection buttons for difficulty selection
  Widget difficulitiesWidgets() {
    return ToggleButtons(
      direction: Axis.horizontal,
      onPressed: (int index) {
        setState(() {
          for (int i = 0; i < difficulitiesValues.length; i++) {
            difficulitiesValues[i] = i == index;
            if (i == index) {
              gameDifficulty = difficulitiesNames[index];
            }
          }
        });
      },
      borderRadius: const BorderRadius.all(Radius.circular(8)),
      borderColor: Colors.transparent,
      constraints: const BoxConstraints(
        minHeight: 40.0,
        minWidth: 80.0,
      ),
      isSelected: difficulitiesValues,
      children: difficulitiesNames.map((e) => Text(e)).toList(),
    );
  }

  bool failedToFetchDaily = false;
  bool? dailySolved;
  String? dailyPuzzle;
  String? dailyDate;
  bool notLoggedIn = false;

  // Flutter build function
  @override
  Widget build(BuildContext context) {
    AccountState accState = AccountState.instance();
    accState.updateStreak();

    // Handels daily sudoku and steak
    if (dailySolved == null &&
        notLoggedIn == false &&
        failedToFetchDaily == false) {
      getDaily().then((value) {
        if (value == null) {
          notLoggedIn = true;
          return;
        }
        var (newPuzzle, newStatus, date) = value;
        setState(() {
          dailyPuzzle = newPuzzle;
          dailySolved = newStatus;
          dailyDate = date;
          failedToFetchDaily = false;
        });
      });
    }

    // Main menu view
    return Scaffold(
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            // Titel
            Text(
              "Sudoku",
              style: TextStyle(
                  fontSize: 50,
                  color: Theme.of(context).buttonTheme.colorScheme!.primary),
            ),
            const SizedBox(height: 10),
            // Show steak and multiplier
            ListenableBuilder(
              listenable: accState,
              builder: (context, _) {
                Account? acc = accState.get();
                const double h = 60;
                if (acc == null) {
                  return const SizedBox(
                    height: h,
                  );
                }
                if (acc.multiplier == null || acc.streak == null) {
                  return const SizedBox(
                    height: h,
                  );
                }

                return SizedBox(
                  height: h,
                  child: Column(
                    children: [
                      Text("Streak: ${acc.streak} days"),
                      Text("Multiplier: ${acc.multiplier!.toStringAsFixed(2)}"),
                    ],
                  ),
                );
              },
            ),
            // Display size and size options
            Text(sizeText),
            const SizedBox(height: 10),
            sudokuSizeWidget(),
            const SizedBox(height: 10),
            Visibility(
              // Size selection box
              visible: showSizeBox,
              child: Column(
                children: [
                  SizedBox(
                    width: 250,
                    child: TextField(
                      onChanged: onTextChange,
                      controller: inputTextController,
                      keyboardType: TextInputType.number,
                      onTapOutside: (event) {
                        FocusManager.instance.primaryFocus?.unfocus();
                      },
                      inputFormatters: <TextInputFormatter>[
                        FilteringTextInputFormatter.digitsOnly
                      ],
                      decoration: const InputDecoration(
                        border: OutlineInputBorder(),
                        labelText: 'Sudoku size',
                      ),
                    ),
                  ),
                  const SizedBox(height: 10),
                ],
              ),
            ),
            // Display difficulty options
            difficulitiesWidgets(),
            const SizedBox(height: 10),
            // Display rule options
            SizedBox(
              width: 300,
              child: Wrap(
                spacing: 5,
                crossAxisAlignment: WrapCrossAlignment.center,
                children: ruleWidgets(),
              ),
            ),
            const SizedBox(height: 20),
            // Next window buttons
            Row(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                // Create Sudoku button
                OutlinedButton(
                  onPressed: () {
                    setState(() {
                      sizeText = "${size}x$size";
                    });
                    Future<String?> sudokuSource = generateWithSize(
                        size: size,
                        rulesSrc: gameModes.toList(),
                        difficulty: gameDifficulty);
                    () async {
                      var res = await Navigator.of(context).push(
                        MaterialPageRoute(
                          builder: (context) => GameLoader(
                              sudokuSource, gameModes, gameDifficulty, size),
                        ),
                      );
                      setState(() {
                        if (res != null) {
                          sizeText = res.toString();
                        }
                      });
                    }();
                  },
                  child: const Text('Create Sudoku'),
                ),
                const SizedBox(
                  width: 10,
                ),
                // To scoreboard button
                OutlinedButton(
                  onPressed: () async {
                    await Navigator.of(context).push(MaterialPageRoute(
                        builder: (context) => const ScoreboardPage()));
                    setState(() {
                      //Rebuild
                    });
                  },
                  child: const Text("Scoreboard"),
                )
              ],
            ),
            const SizedBox(
              height: 10,
            ),
            // Daily Sudoku button and text
            Row(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                if (AccountState.instance().get() == null) ...[
                  const Text("Login to solve daily puzzles"),
                ] else if (failedToFetchDaily) ...[
                  const Text("Failed to fetch daily puzzle"),
                ] else if (dailySolved == null) ...[
                  SpinKitCircle(
                    color: Theme.of(context).highlightColor,
                  )
                ] else if (dailySolved == true) ...[
                  const ElevatedButton(
                      onPressed: null, child: Text("Already solved")),
                ] else ...[
                  ElevatedButton(
                      onPressed: () async {
                        await setFromStr(sudoku: dailyPuzzle!);
                        var xPositions = await getXPositions();
                        var parityPositions = await getParityPositions();
                        var zipperPositions = await getZipperPositions();
                        var consecutivePositions =
                            await getConsecutivePositions();
                        var thermometerPositions =
                            await getThermometerPositions();
                        GameState.setInstance(GameState(
                            dailyPuzzle!.split("\n\n")[1],
                            xPositions,
                            parityPositions,
                            consecutivePositions,
                            zipperPositions,
                            thermometerPositions,
                            daily: dailyDate!));

                        setState(() {
                          Navigator.of(context)
                              .push(MaterialPageRoute(
                            builder: (context) =>
                                const GameView({"SquareRule"}),
                          ))
                              .then((_) {
                            getDaily().then((value) {
                              setState(() {
                                dailySolved = null;
                                dailyPuzzle = null;
                                dailyDate = null;
                              });
                              if (value == null) {
                                notLoggedIn = true;
                                return;
                              }
                              var (newPuzzle, newStatus, date) = value;
                              setState(() {
                                dailyPuzzle = newPuzzle;
                                dailySolved = newStatus;
                                dailyDate = date;
                                failedToFetchDaily = false;
                              });
                            });
                          });
                        });
                      },
                      child: const Text("Daily puzzle")),
                ]
              ],
            )
          ],
        ),
      ),
    );
  }
}

// Get daily function
Future<(String, bool?, String)?> getDaily() async {
  Account? acc = AccountState.instance().get();
  Map<String, String>? body;
  if (acc != null) {
    body = {"user_id": acc.userID};
  }

  try {
    var response = await http.post(serverAddress.resolve("/daily"), body: body);

    Map<String, dynamic> jsonBody = jsonDecode(response.body);

    bool? dailySolved = jsonBody["solved"];
    String puzzle = jsonBody["puzzle"];
    String dato = jsonBody["dato"];

    return (puzzle, dailySolved, dato);
  } catch (e) {
    return null;
  }
}
