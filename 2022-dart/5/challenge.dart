import 'dart:collection';
import 'dart:io';
import 'dart:convert';
import 'dart:async';

void main() async {
  part1();
  part2();
}

void part1() async {
  var crates = await parseCrates();

  final moves = File('/workspaces/adventofcode/2022-dart/5/testinput')
      .openRead()
      .transform(utf8.decoder)
      .transform(LineSplitter())
      .transform(CraneMoveParser());

  await for (var move in moves) {
    print('move ${move.amount} from ${move.source} to ${move.target}');
    for (var crate in crates[move.source - 1].stack.take(move.amount)) {
      crates[move.target - 1].stack.add(crate);
    }
  }
  print('Part 1: 0');
}

void part2() async {
  print('Part 2: 0');
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
  final lines = File('/workspaces/adventofcode/2022-dart/5/testinput')
      .openRead()
      .transform(utf8.decoder)
      .transform(LineSplitter());

  var stacks = List<Stack>.empty(growable: true);
  await for (var line in lines) {
    if (line.isEmpty) {
      break;
    }

    var chars = line.codeUnits;
    for (var i = 0; i < chars.length; i += 4) {
      if (stacks.length <= i % 3) {
        stacks.add(Stack(Queue()));
      }
      if (chars[i + 1] != 32 && chars[i + 1] > 64) {
        stacks[i % 3].stack.add(chars[i + 1]);
      }
    }
  }
  return stacks;
}

class CraneMoveParser extends StreamTransformerBase<String, CraneMove> {
  @override
  Stream<CraneMove> bind(Stream<String> stream) async* {
    await for (var value in stream) {
      if (value.startsWith("move")) {
        final s = value.split(' ');
        if (s.length == 6) {
          yield CraneMove(int.parse(s[1]), int.parse(s[3]), int.parse(s[5]));
        }
      }
    }
  }
}
