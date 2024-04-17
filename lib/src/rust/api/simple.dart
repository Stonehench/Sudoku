// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.24.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Stream<(int, int)> progress({dynamic hint}) =>
    RustLib.instance.api.progress(hint: hint);

Future<String?> generateWithSize(
        {required int size,
        required List<String> rulesSrc,
        required String difficulty,
        dynamic hint}) =>
    RustLib.instance.api.generateWithSize(
        size: size, rulesSrc: rulesSrc, difficulty: difficulty, hint: hint);

Future<List<(int, int)>> getXPositions({dynamic hint}) =>
    RustLib.instance.api.getXPositions(hint: hint);

Future<List<(int, int)>> getConsecutivePositions({dynamic hint}) =>
    RustLib.instance.api.getConsecutivePositions(hint: hint);

Future<List<(int, int)>> getParityPositions({dynamic hint}) =>
    RustLib.instance.api.getParityPositions(hint: hint);

Future<List<(int, List<(int, int)>)>> getZipperPositions({dynamic hint}) =>
    RustLib.instance.api.getZipperPositions(hint: hint);

Future<List<Uint16List>> getThermometerPositions({dynamic hint}) =>
    RustLib.instance.api.getThermometerPositions(hint: hint);

Future<bool> checkLegality(
        {required int position, required int value, dynamic hint}) =>
    RustLib.instance.api
        .checkLegality(position: position, value: value, hint: hint);

Future<void> closeThreads({dynamic hint}) =>
    RustLib.instance.api.closeThreads(hint: hint);

Future<int?> difficultyValues(
        {required int size, required String difficulty, dynamic hint}) =>
    RustLib.instance.api
        .difficultyValues(size: size, difficulty: difficulty, hint: hint);

Future<void> setFromStr({required String sudoku, dynamic hint}) =>
    RustLib.instance.api.setFromStr(sudoku: sudoku, hint: hint);
