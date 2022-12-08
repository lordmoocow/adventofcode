import 'dart:async';
import 'dart:collection';
import 'dart:io';
import 'dart:convert';

const path = './8/input';

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

  // just for debugging - or maybe not actually
  final visualise = List.generate(maxY, (_) => List.generate(maxY, (_) => 0));

  for (var row = 0; row < maxY; row++) {
    for (var col = 0; col < maxX; col++) {
      // edges always visible
      if (row == 0 || row == maxY - 1 || col == 0 || col == maxY - 1) {
        count++;
        visualise[row][col] = trees[row][col] - 48;
        continue;
      }

      var r = false;
      var l = false;
      var u = false;
      var d = false;

      // check if the right is blocked
      for (var x = col + 1; x < maxX; x++) {
        if (trees[row][x] >= trees[row][col]) {
          r = true;
          break;
        }
      }
      // check if the left is blocked
      for (var x = col - 1; x >= 0; x--) {
        if (trees[row][x] >= trees[row][col]) {
          l = true;
          break;
        }
      }
      // check if the below is blocked
      for (var y = row + 1; y < maxY; y++) {
        if (trees[y][col] >= trees[row][col]) {
          d = true;
          break;
        }
      }
      // check if the above is blocked
      for (var y = row - 1; y >= 0; y--) {
        if (trees[y][col] >= trees[row][col]) {
          u = true;
          break;
        }
      }

      if (!(r && l && u && d)) {
        count++;
        visualise[row][col] = trees[row][col] - 48;
      }
    }
  }

  // for (final row in visualise) {
  //   print("$row");
  // }
  print("Part 1: $count");
}

void part2() async {
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

  var maxScore = 0;

  // bounds
  final maxX = trees[0].length;
  final maxY = trees.length;

  // just for debugging - or maybe not actually
  final visualise = List.generate(maxY, (_) => List.generate(maxY, (_) => 0));

  for (var row = 0; row < maxY; row++) {
    for (var col = 0; col < maxX; col++) {
      if (row == 0 || row == maxY - 1 || col == 0 || col == maxY - 1) {
        visualise[row][col] = 0;
        continue;
      }

      var score = 0;

      // right
      var count = 0;
      for (var x = col + 1; x < maxX; x++) {
        count++;
        if (trees[row][x] < trees[row][col]) {
          continue;
        }
        break;
      }
      score = count;

      // left
      count = 0;
      for (var x = col - 1; x >= 0; x--) {
        count++;
        if (trees[row][x] < trees[row][col]) {
          continue;
        }
        break;
      }
      score *= count;

      // down
      count = 0;
      for (var y = row + 1; y < maxY; y++) {
        count++;
        if (trees[y][col] < trees[row][col]) {
          continue;
        }
        break;
      }
      score *= count;

      // up
      count = 0;
      for (var y = row - 1; y >= 0; y--) {
        count++;
        if (trees[y][col] < trees[row][col]) {
          continue;
        }
        break;
      }
      score *= count;


      visualise[row][col] = score;
      if (score > maxScore) {
        maxScore = score;
      }
    }
  }

  // for (final row in visualise) {
  //   print("$row");
  // }
  print("Part 1: $maxScore");
}
