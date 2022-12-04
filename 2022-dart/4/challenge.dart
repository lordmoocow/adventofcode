import 'dart:io';
import 'dart:convert';
import 'dart:async';

void main() async {
  part1();
  part2();
}

void part1() async {
  final elves = File('/workspaces/advent/2022-dart/4/input')
      .openRead()
      .transform(utf8.decoder)
      .transform(LineSplitter())
      .transform(AssignmentSplitter());

  var count = 0;
  await for (var assignment in elves) {
    if (assignment.a.fullyOverlap(assignment.b)) {
      count++;
    }
  }
  print('Part 1: $count');
}

void part2() async {
  final elves = File('/workspaces/advent/2022-dart/4/input')
      .openRead()
      .transform(utf8.decoder)
      .transform(LineSplitter())
      .transform(AssignmentSplitter());

  var count = 0;
  await for (var assignment in elves) {
    if (assignment.a.partialOverlap(assignment.b)) {
      count++;
    }
  }
  print('Part 2: $count');
}

class Assignment {
  final Elf a;
  final Elf b;

  const Assignment(this.a, this.b);
}

class Elf {
  final int start;
  final int end;

  const Elf(this.start, this.end);

  bool fullyOverlap(Elf other) {
    return (start <= other.start && end >= other.end) ||
        (other.start <= start && other.end >= end);
  }

  bool partialOverlap(Elf other) {
    return (start >= other.start && start <= other.end) ||
        (other.start >= start && other.start <= end);
  }
}

class AssignmentSplitter extends StreamTransformerBase<String, Assignment> {
  @override
  Stream<Assignment> bind(Stream<String> stream) async* {
    await for (var value in stream) {
      final s = value.split(',');
      if (s.length == 2) {
        final a = s[0].split('-');
        final elfA = Elf(int.parse(a[0]), int.parse(a[1]));

        final b = s[1].split('-');
        final elfB = Elf(int.parse(b[0]), int.parse(b[1]));

        yield Assignment(elfA, elfB);
      }
    }
  }
}
