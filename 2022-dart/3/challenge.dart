import 'dart:io';
import 'dart:convert';
import 'dart:async';

const path = './3/input';

void main() {
  part1();
  part2();
}

void part1() async {
  final rucksacks = File(path)
      .openRead()
      .transform(utf8.decoder)
      .transform(LineSplitter())
      .transform(Rucksacks());

  var score = 0;
  await for (final rucksack in rucksacks) {
    for (final item
        in findDuplicates(rucksack.compartment1(), rucksack.compartment2())) {
      score += toPriority(item);
    }
  }
  print('Part 1: $score');
}

void part2() async {
  final groups = File(path)
      .openRead()
      .transform(utf8.decoder)
      .transform(LineSplitter())
      .transform(Rucksacks())
      .transform(Groups());

  var score = 0;
  await for (final group in groups) {
    var dupes = findDuplicates(
        findDuplicates(group[0].items, group[1].items), group[2].items);
    score += toPriority(dupes.first);
  }
  print('Part 2: $score');
}

class Rucksack {
  final List<int> items;

  const Rucksack(this.items);

  Iterable<int> compartment1() {
    return items.sublist(0, (items.length / 2).round());
  }

  Iterable<int> compartment2() {
    return items.sublist((items.length / 2).round());
  }
}

Set<int> findDuplicates(Iterable<int> a, Iterable<int> b) {
  var dupes = <int>{};
  for (var item in a) {
    if (b.contains(item)) {
      dupes.add(item);
    }
  }
  return dupes;
}

int toPriority(int i) {
  if (i >= 97) {
    return i - 96;
  } else {
    return i - 38;
  }
}

class Rucksacks extends StreamTransformerBase<String, Rucksack> {
  @override
  Stream<Rucksack> bind(Stream<String> stream) async* {
    await for (var value in stream) {
      if (value != '') {
        yield Rucksack(value.codeUnits);
      }
    }
  }
}

class Groups extends StreamTransformerBase<Rucksack, List<Rucksack>> {
  @override
  Stream<List<Rucksack>> bind(Stream<Rucksack> stream) async* {
    List<Rucksack> group = [];
    await for (var value in stream) {
      if (group.length < 3) {
        group.add(value);
      } else {
        yield group.toList();
        group.clear();
        group.add(value);
      }
    }
    yield group;
  }
}
