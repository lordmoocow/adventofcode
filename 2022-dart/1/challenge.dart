import 'dart:io';
import 'dart:convert';
import 'dart:async';

const path = '/workspaces/advent/2022-dart/1/input';

void main() {
  part1();
  part2();
}

void part1() async {
  final elves = File(path)
      .openRead()
      .transform(utf8.decoder)
      .transform(LineSplitter())
      .transform(CalorieCounter());

  var largest = 0;
  await for (final elf in elves) {
    if (elf > largest) {
      largest = elf;
    }
  }
  print('Part 1: $largest');
}

void part2() async {
  final elves = File(path)
      .openRead()
      .transform(utf8.decoder)
      .transform(LineSplitter())
      .transform(CalorieCounter());

  final largest = [0, 0, 0];
  await for (final elf in elves) {
    for (var i in [2, 1, 0]) {
      if (elf > largest[i]) {
        largest.removeAt(0);
        largest.insert(i, elf);
        break;
      }
    }
  }

  var sum = 0;
  for (final elf in largest) {
    sum += elf;
  }
  print('Part 2: $sum');
}

class CalorieCounter extends StreamTransformerBase<String, int> {
  @override
  Stream<int> bind(Stream<String> stream) async* {
    var count = 0;
    await for (final value in stream) {
      if (value == '') {
        yield count;
        count = 0;
      } else {
        count += int.parse(value);
      }
    }
    yield count;
  }
}
