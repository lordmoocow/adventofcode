import 'dart:async';
import 'dart:collection';
import 'dart:io';
import 'dart:convert';

const path = './8/testinput';

void main() {
  part1();
  part2();
}

void part1() async {
  final trees = await File(path)
      .openRead()
      .transform(utf8.decoder)
      .transform(LineSplitter())
      .transform(
          StreamTransformer<String, List<int>>.fromBind(((stream) async* {
    await for (final line in stream) {
      yield line.runes.toList();
    }
  }))).toList();

  var count = 0;

  // bounds
  final maxX = trees[0].length;
  final maxY = trees.length;

  List<List<bool>> checked = List.filled(maxY, List.filled(maxX, false));

  Queue<List<int>> tocheck = Queue();

  // Start from middle tree in all directions
  tocheck.add([(maxX / 2).round() - 1, (maxY / 2).round() - 1]);

  // Look out in each direction to find a tree that blocks the current tree
  // add it to the list of trees to check
  // keep doing this and tot up the ones which don't have anything blocking it
  // pray that it works - it didn't
  while (tocheck.isNotEmpty) {
    final tree = tocheck.removeFirst();
    final x = tree[1];
    final y = tree[0];

    if (checked[y][x]) continue;

    checked[y][x] = true;
    print("$tree: ${utf8.decode([trees[y][x]])}");

    var visible = false;

    if (x == 0 || x == maxX - 1 || y == 0 || y == maxY - 1) {
      visible = true;
    }

    // look right?
    for (var i = 1; x + i < maxX; i++) {
      if (trees[y][x + i] >= trees[y][x + i]) {
        if (!checked[y][x + i]) {
          tocheck.add([y, x + i]);
        }
        break;
      }
      visible = true;
    }

    // look down?
    for (var i = 1; y + i < maxY; i++) {
      if (trees[y + i][x] >= trees[y + i][x]) {
        if (!checked[y + i][x]) {
          tocheck.add([y + i, x]);
        }
        break;
      }
      visible = true;
    }

    // look left?
    for (var i = 1; x - i > 0; i++) {
      if (trees[y][x - i] >= trees[y][x - i]) {
        if (!checked[y][x - i]) {
          tocheck.add([y, x - i]);
        }
        break;
      }
      visible = true;
    }

    // look up?
    for (var i = 1; y - i > 0; i++) {
      if (trees[y - i][x] >= trees[y - i][x]) {
        if (!checked[y - i][x]) {
          tocheck.add([y - i, x]);
        }
        break;
      }
      visible = true;
    }

    if (visible) {
      count++;
    }

    for (final line in checked) {
      print("${line}");
    }
    for (final line in tocheck) {
      print("${line}");
    }
    break;
  }

  print("Part 1: $count");
}

void part2() async {}
