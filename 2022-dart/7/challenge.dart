import 'dart:io';
import 'dart:convert';

const path = './7/input';

void main() {
  part1();
  part2();
}

void part1() async {
  final sizes = await parseDirSizes();
  var sum = 0;
  for (final size in sizes.values) {
    if (size <= 100000) sum += size;
  }
  print("Part 1: $sum");
}

void part2() async {
  final sizes = await parseDirSizes();

  final availableSpace = 70000000 - sizes["/"]!;
  final neededSpace = 30000000 - availableSpace;

  // look for smallest value (start with max possible file size to compare against)
  // this loops saves needing to sort or iterate multiple times
  var smallest = sizes["/"]!;
  for (final x in sizes.values) {
    if (x >= neededSpace && x < smallest) {
      smallest = x;
    }
  }

  print("Part 2: $smallest");
}

Future<Map<String, int>> parseDirSizes() async {
  final lines =
      File(path).openRead().transform(utf8.decoder).transform(LineSplitter());

  List<String> breadcrumb = [];
  Map<String, int> sizes = {};
  await for (final line in lines) {
    final parts = line.split(" ");
    switch (parts[0]) {
      case "\$":
        if (parts[1] == "cd") {
          if (parts[2] == "..") {
            breadcrumb.removeLast();
          } else {
            breadcrumb.add(parts[2]);
            sizes.putIfAbsent(breadcrumb.join(), () => 0);
          }
        }
        break;

      case "dir":
        continue;

      default:
        if (parts.length == 2) {
          final size = int.parse(parts[0]);
          for (var i = 0; i < breadcrumb.length; i++) {
            final key = breadcrumb.take(i + 1).join();
            sizes[key] = sizes[key]! + size;
          }
        }
    }
  }
  return sizes;
}
