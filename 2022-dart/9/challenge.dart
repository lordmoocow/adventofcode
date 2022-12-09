import 'dart:async';
import 'dart:io';
import 'dart:convert';
import 'dart:math';

const path = './9/input';

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

  final visited = <Point>{};
  var hPos = Point.origin();
  var tPos = Point.origin();

  //printDebug([hPos, tPos], visited);

  await for (final move in instructions) {
    // move H by instruction
    hPos += move;

    tPos = follow(tPos, hPos);
    visited.add(tPos);

    //printDebug([hPos, tPos], visited);
  }

  print("Part 1: ${visited.length}");
}

void part2() async {
  final instructions = File(path)
      .openRead()
      .transform(utf8.decoder)
      .transform(LineSplitter())
      .transform(InstructionParser());

  final visited = <Point>{};
  final rope = List<Point>.generate(10, (_) => Point.origin());

  //printDebug(rope, visited);

  await for (final move in instructions) {
    // move H by instruction
    rope.first += move;

    // each knot follows the previous knot (skip head as that has done it's move)
    for (var x = 1; x < rope.length; x++) {
      rope[x] = follow(rope[x], rope[x - 1]);
    }
    // track visited points for tail knot
    visited.add(rope.last);

    //printDebug(rope, visited);
  }

  print("Part 2: ${visited.length}");
}

Point follow(Point src, Point tgt) {
  final delta = tgt - src;
  final distance = delta.magnitude();

  if (distance >= 2) {
    return src + delta.norm() * (distance - 1);
  }
  return src;
}

printDebug(List<Point> rope, Set<Point> visited) {
  sleep(Duration(milliseconds: 100));

  for (var row = -40; row <= 40; row++) {
    var s = "";
    for (var col = -40; col <= 40; col++) {
      final x = Point(col, row);
      if (x == rope.first) {
        s += " H ";
      } else if (rope.skip(1).contains(x)) {
        s += " ${rope.indexOf(x)} ";
      } else if (row | col == 0) {
        s += " o ";
      } else if (visited.contains(x)) {
        s += " . ";
      } else {
        s += "   ";
      }
    }
    print(s);
  }
  print("DEBUG: ${visited.length}");
}

class Point {
  final int x, y;

  const Point(this.x, this.y);
  const Point.origin() : this(0, 0);

  Point operator +(Vector v) => Point(x + v.x.round(), y + v.y.round());

  Vector operator -(Point p) =>
      Vector((x - p.x).toDouble(), (y - p.y).toDouble());

  @override
  bool operator ==(Object p) => p is Point && x == p.x && y == p.y;

  @override
  int get hashCode => Object.hash(x, y);

  @override
  String toString() => "[$x,$y]";
}

class Vector {
  final double x, y;

  const Vector(this.x, this.y);

  const Vector.up() : this(0, -1);
  const Vector.down() : this(0, 1);
  const Vector.left() : this(-1, 0);
  const Vector.right() : this(1, 0);

  factory Vector.fromDirection(int direction) {
    // convert from ascii values
    switch (direction) {
      case 85: // U
        return Vector.up();

      case 68: // D
        return Vector.down();

      case 76: // L
        return Vector.left();

      case 82: // R
        return Vector.right();
    }

    throw ArgumentError.value(direction, "direction");
  }

  Vector operator *(double mag) => Vector(x * mag, y * mag);

  double magnitude() {
    return sqrt((x * x) + (y * y));
  }

  Vector norm() {
    final mag = magnitude();
    return Vector(x / mag, y / mag);
  }

  @override
  String toString() => "[$x,$y]";
}

class InstructionParser extends StreamTransformerBase<String, Vector> {
  @override
  Stream<Vector> bind(Stream<String> stream) async* {
    await for (final value in stream) {
      if (value.isNotEmpty) {
        final parts = value.split(" ");
        if (parts.length != 2) continue;

        final dir = parts[0].codeUnits.first;
        final mag = int.parse(parts[1]);
        for (var i = 0; i < mag; i++) {
          yield Vector.fromDirection(dir);
        }
      }
    }
  }
}
