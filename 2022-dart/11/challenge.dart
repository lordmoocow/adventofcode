import 'dart:async';
import 'dart:io';
import 'dart:convert';

const path = './11/input';

void main() {
  part1();
  part2();
}

void part1() async {
  final monkies = await File(path)
      .openRead()
      .transform(utf8.decoder)
      .transform(LineSplitter())
      .transform(MonkeyParser())
      .toList();

  for (var i = 0; i < 20; i++) {
    for (final monkey in monkies) {
      for (var j = 0; j < monkey.items.length; j++) {
        var item = monkey.inspect(j);
        item = (item / 3).floor();
        monkies[monkey.passTo(item)].items.add(item);
      }
      monkey.items.clear();
    }
  }

  // probably not the most efficient but meh
  monkies.sort((a, b) => b.inspections.compareTo(a.inspections));
  final monkeyBusiness = monkies[0].inspections * monkies[1].inspections;
  print("Part 1: $monkeyBusiness");
}

void part2() async {
  final monkies = await File(path)
      .openRead()
      .transform(utf8.decoder)
      .transform(LineSplitter())
      .transform(MonkeyParser())
      .toList();

  // this took way too long to figure out...
  // multply all the _testVals and we get a number which is a multiple of
  // every possible test value which can be used to reduce worry level
  var multiple = monkies.fold(1, (acc, monkey) => acc * monkey.testVal);

  for (var i = 0; i < 10000; i++) {
    for (final monkey in monkies) {
      for (var j = 0; j < monkey.items.length; j++) {
        var item = monkey.inspect(j);
        // reduce item value to keep it from growing exponentionally.
        // should still be divisible by testVal if it was originally
        //print("$item $multiple ${item % multiple}");
        item = item % multiple;
        monkies[monkey.passTo(item)].items.add(item);
      }
      monkey.items.clear();
    }
  }

  // probably not the most efficient but meh
  monkies.sort((a, b) => b.inspections.compareTo(a.inspections));
  final monkeyBusiness = monkies[0].inspections * monkies[1].inspections;
  print("Part 2: $monkeyBusiness");
}

class MonkeyParser extends StreamTransformerBase<String, Monkey> {
  @override
  Stream<Monkey> bind(Stream<String> stream) async* {
    final lines = <String>[];
    await for (final value in stream) {
      if (value.isNotEmpty) {
        lines.add(value.trim());
      } else {
        yield Monkey.fromNotes(lines);
        lines.clear();
      }
    }
    // just in case there is no empty line at the end
    if (lines.isNotEmpty) {
      yield Monkey.fromNotes(lines);
    }
  }
}

class Monkey {
  int inspections = 0;
  final List<int> items;

  final int Function(int) inspectOp;
  final int testVal, toPos, toNeg;

  Monkey(this.items, this.inspectOp, this.testVal, this.toPos, this.toNeg);

  factory Monkey.fromNotes(List<String> notes) {
    final items = notes[1]
        .split(":")
        .last
        .split(",")
        .map((i) => int.parse(i.trim()))
        .toList();

    return Monkey(
        items,
        parseInspectOp(notes[2]),
        int.parse(notes[3].split(" ").last),
        int.parse(notes[4].split(" ").last),
        int.parse(notes[5].split(" ").last));
  }

  int inspect(final int item) {
    inspections++;
    return inspectOp(items[item]);
  }

  int passTo(final int item) {
    if (item % testVal == 0) {
      return toPos;
    } else {
      return toNeg;
    }
  }
}

int Function(int) parseInspectOp(String notes) {
  final parts = notes.split("=").last.split(" ").toList();
  // I feel like there might be a better way to do this but...
  if (parts[3] == "old") {
    switch (parts[2]) {
      case "*":
        return (old) => (old * old);
      case "+":
        return (old) => (old + old);
      case "-":
        return (old) => (old - old);
    }
  } else {
    final v = int.parse(parts[3]);
    switch (parts[2]) {
      case "*":
        return (old) => (old * v);
      case "+":
        return (old) => (old + v);
      case "-":
        return (old) => (old - v);
    }
  }
  throw Exception();
}
