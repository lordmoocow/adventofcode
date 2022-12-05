import 'dart:collection';
import 'dart:io';
import 'dart:convert';
import 'dart:async';

const path = '/workspaces/advent/2022-dart/5/input';

void main() {
  part1();
  part2();
}

void part1() async {
  final stacks = await parseCrates();

  await for (final move in parseMoves()) {
    for (var i = 0; i < move.amount; i++) {
      stacks[move.target]
          .stack
          .addFirst(stacks[move.source].stack.removeFirst());
    }
  }

  List<int> message = [];
  for (final s in stacks) {
    message.add(s.stack.first);
  }
  print("Part 1: ${utf8.decode(message)}");
}

void part2() async {
  final stacks = await parseCrates();

  await for (final move in parseMoves()) {
    final moving = Queue<int>();
    for (var i = 0; i < move.amount; i++) {
      moving.addFirst(stacks[move.source].stack.removeFirst());
    }
    for (final crate in moving) {
      stacks[move.target].stack.addFirst(crate);
    }
  }

  List<int> message = [];
  for (final s in stacks) {
    message.add(s.stack.first);
  }
  print("Part 2: ${utf8.decode(message)}");
}

class Stack {
  Queue<int> stack;

  Stack(this.stack);
}

class CraneMove {
  final int amount;
  final int source;
  final int target;

  const CraneMove(this.amount, this.source, this.target);
}

Future<List<Stack>> parseCrates() async {
  final lines =
      File(path).openRead().transform(utf8.decoder).transform(LineSplitter());

  List<Stack> stacks = [];
  await for (final line in lines) {
    if (line.isEmpty) {
      break;
    }

    final chars = line.codeUnits;
    for (var i = 0; i < ((chars.length + 1) / 4); i++) {
      while (stacks.length < i + 1) {
        stacks.add(Stack(Queue()));
      }

      final c = chars[(1 + i * 4)];
      if (c != 32 && c > 64) {
        stacks[i].stack.add(c);
      }
    }
  }
  return stacks;
}

Stream<CraneMove> parseMoves() {
  return File(path)
      .openRead()
      .transform(utf8.decoder)
      .transform(LineSplitter())
      .transform(StreamTransformer.fromBind(((stream) async* {
    await for (final value in stream) {
      if (value.startsWith("move")) {
        final s = value.split(' ');
        if (s.length == 6) {
          yield CraneMove(
              int.parse(s[1]), int.parse(s[3]) - 1, int.parse(s[5]) - 1);
        }
      }
    }
  })));
}
