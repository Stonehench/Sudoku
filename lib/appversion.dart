import 'dart:convert';

import 'package:http/http.dart' as http;
import 'package:package_info_plus/package_info_plus.dart';

class Version {
  Version(this.major, this.minor, this.patch);

  final int major;
  final int minor;
  final int patch;

  static int compare(Version a, Version b) {
    int major = a.major - b.major;
    int minor = a.minor - b.minor;
    int patch = a.patch - b.patch;

    print("DIFFS: $major . $minor . $patch");
    if (major != 0) {
      return major;
    }
    if (minor != 0) {
      return minor;
    }
    return patch;
  }
}

Future<bool> checkIsOutdated() async {
  try {
    var resp = await http.get(Uri.https("f-skipper.com", "/api/dir/apps/"));
    if (resp.statusCode != 200) {
      return false;
    }
    var entries =
        (JsonDecoder().convert(resp.body) as Map)["entries"] as List<dynamic>;

    var sudokuNames = entries
        .map((e) => ((e as Map)["relative_name"] as String))
        .where((name) => name.startsWith("sudoku"));

    var versions = sudokuNames.map((n) {
      var numbers = n.split("_")[1].split(".apk")[0].split(".");
      return Version(
          int.parse(numbers[0]), int.parse(numbers[1]), int.parse(numbers[2]));
    }).toList();

    versions.sort(Version.compare);
    var newest = versions.last;

    PackageInfo packageInfo = await PackageInfo.fromPlatform();

    String version = packageInfo.version;

    var versionNums = version.split(".");

    var packageVersion = Version(int.parse(versionNums[0]),
        int.parse(versionNums[1]), int.parse(versionNums[2]));

    int compareRes = Version.compare(newest, packageVersion);
    return compareRes > 0;
  } catch (e) {
    return false;
  }
}