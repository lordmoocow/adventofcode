import 'dart:async';
import 'dart:collection';
import 'dart:io';
import 'dart:convert';

const path = './7/input';

void main() {
  part1();
  part2();
}

void part1() async {
  final trees = await File(path)
      .openRead()
      .transform(utf8.decoder)
      .transform(LineSplitter())
      .transform(StreamTransformer<String, Runes>.fromBind(((stream) async* {
    await for (final line in stream) {
      yield line.runes;
    }
  }))).toList();

  var count = 0;

  // Initialise visibles list with outer edge coordinates
  Queue<List<int>> visible = Queue();
  for (var i = 0; i < trees.first.length; i++) {
    visible.add([i, 0]);
    visible.add([i, trees.length]);
  }
  for (final row in trees) {
    visible.add([0, row.length]);
  }

  while (visible.isNotEmpty) {
    final tree = visible.removeFirst();
    // see if it blocks anything?
  }

  print("Part 1: $count");
}

void part2() async {}
