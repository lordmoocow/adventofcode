import 'dart:async';
import 'dart:io';
import 'dart:convert';

const path = './10/input';

void main() {
  part1();
  part2();
}

void part1() async {
  final instructions = File(path)
      .openRead()
      .transform(utf8.decoder)
      .transform(LineSplitter())
      .transform(InstructionParser());

  var system = System();

  await for (final instruction in instructions) {
    system.tick(instruction);
  }

  final sum = system.signals.values.fold(0, (h, i) => h + i);
  print("Part 1: $sum");
}

void part2() async {
  final instructions = File(path)
      .openRead()
      .transform(utf8.decoder)
      .transform(LineSplitter())
      .transform(InstructionParser());

  var system = System();
  await for (final instruction in instructions) {
    system.tick(instruction);
    //print(system);
    //sleep(Duration(milliseconds: 8));
  }

  print("Part 2:\r\n$system");
}

class System {
  int x = 1, cycle = 0;
  Map<int, int> signals = {};
  CRT screen = CRT(40, 6);

  tick(Instruction cmd) {
    cycle++;
    if ((cycle + 20) % 40 == 0) {
      signals[cycle] = signalStrength();
    }
    screen.tick(this);
    cmd.execute(this);
  }

  signalStrength() => x * cycle;

  @override
  String toString() {
    return "$screen";
  }
}

class CRT {
  final int width, height;
  late final List<bool> pixels = List.generate(width * height, (_) => false);

  CRT(this.width, this.height);

  tick(System s) {
    final pX = s.cycle % (pixels.length / height).round();
    if (pX == s.x || pX == s.x + 1 || pX == s.x + 2) {
      pixels[s.cycle % pixels.length] = true;
    } else {
      pixels[s.cycle % pixels.length] = false;
    }
  }

  @override
  String toString() {
    var s = "";
    for (var pixel = 0; pixel < pixels.length; pixel++) {
      if (pixels[(pixel + 1) % pixels.length]) {
        s += "█";
      } else {
        s += ".";
      }

      if ((pixel + 1) % width == 0) {
        s += "\r\n";
      }
    }
    return s;
  }
}

class InstructionParser extends StreamTransformerBase<String, Instruction> {
  @override
  Stream<Instruction> bind(Stream<String> stream) async* {
    await for (final value in stream) {
      if (value.isNotEmpty) {
        final parts = value.split(" ");
        if (parts.isEmpty) continue;

        var ins = Instruction.fromCommand(parts);
        // if the instruction takes multiple cycles just pad it with noops
        for (var i = 1; i < ins.cycles; i++) {
          yield Noop();
        }
        yield ins;
      }
    }
  }
}

abstract class Instruction {
  final int cycles;

  const Instruction(this.cycles);

  factory Instruction.fromCommand(List<String> command) {
    switch (command.first) {
      case "noop":
        return Noop();

      case "addx":
        return AddX(int.parse(command[1]));
    }

    throw ArgumentError("Unknown instruction: $command", "instruction");
  }

  execute(System s);
}

class Noop extends Instruction {
  const Noop() : super(1);

  @override
  execute(System _) {}
}

class AddX extends Instruction {
  final int v;

  const AddX(this.v) : super(2);

  @override
  execute(System s) {
    s.x += v;
  }
}
