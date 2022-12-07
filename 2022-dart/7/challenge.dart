import 'dart:io';
import 'dart:convert';

const path = '/workspaces/adventofcode/2022-dart/7/input';

void main() {
  part1();
  part2();
}

void part1() async {
  final lines =
      File(path).openRead().transform(utf8.decoder).transform(LineSplitter());

  var current = "";
  List<String> breadcrumb = [];
  Map<String, int> sizes = {};
  await for (final line in lines) {
    final parts = line.split(" ");
    switch (parts[0]) {
      case "\$":
        if (parts.length > 2) {
          if (parts[2] == "..") {
            breadcrumb.removeLast();
            current = breadcrumb.last;
          } else {
            current = parts[2];
            breadcrumb.add(current);
            sizes.putIfAbsent(breadcrumb.join("/"), () => 0);
          }
        }
        break;

      case "dir":
        //sizes.putIfAbsent("${breadcrumb.join("/")}/${parts[1]}", () => 0);
        continue;

      default:
        if (parts.length == 2) {
          //print("${breadcrumb.join("/")}: ${parts}");
          final size = int.parse(parts[0]);
          for (var i = 0; i < breadcrumb.length; i++) {
            final key = breadcrumb.take(i + 1).join("/");
            sizes[key] = sizes[key]! + size;
          }
        }
    }
  }

  print("${sizes}");
  var sum = 0;
  for (final size in sizes.values) {
    if (size <= 100000) sum += size;
  }
  print("Part 1: $sum");
}

void part2() async {}
