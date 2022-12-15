import 'dart:async';
import 'dart:io';
import 'dart:convert';

const path = './14/input';

void main() {
  part1();
  part2();
}

void part1() async {
  final cave = await scanCave();

  while (cave.update()) {
    // print("$cave");
    // sleep(Duration(milliseconds: 16));
  }

  print("Part 1: ${cave.sand.length}");
}

void part2() async {
  final cave = await scanCave();
  cave.floor = true;

  while (cave.update()) {
    print("$cave");
    sleep(Duration(milliseconds: 8));
  }

  print("Part 2: ${cave.sand.length}");
}

Future<Cave> scanCave() async {
  final stream = File(path)
      .openRead()
      .transform(utf8.decoder)
      .transform(LineSplitter())
      .transform(PathScanner());

  Point min = Point(500, 0);
  Point max = Point(500, 0);
  final rocks = <Point>{};
  await for (final point in stream) {
    // lol
    if (point.x < min.x) min = Point(point.x, min.y);
    if (point.y < min.y) min = Point(min.x, point.y);
    if (point.x > max.x) max = Point(point.x, max.y);
    if (point.y > max.y) max = Point(max.x, point.y);
    rocks.add(point);
  }

  return Cave(rocks, min, max);
}

class PathScanner extends StreamTransformerBase<String, Point> {
  @override
  Stream<Point> bind(Stream<String> stream) async* {
    await for (final path in stream) {
      if (path.isEmpty) continue;

      final points = path
          .split(" -> ")
          .map((e) => e.split(","))
          .map((e) => Point(int.parse(e.first), int.parse(e.last)))
          .toList();

      for (var i = 0; i < points.length; i++) {
        final b = points[i];
        // return the end points as is
        yield b;

        // after the first point in each path, we need to draw a line
        // to the next point; so generate points in between
        if (i > 0) {
          final a = points[i - 1];
          if (a.x != b.x) {
            for (var x = 0; a.x + x != b.x; (a.x < b.x ? x++ : x--)) {
              yield Point(a.x + x, a.y);
            }
          }
          if (a.y != b.y) {
            for (var y = 0; a.y + y != b.y; (a.y < b.y ? y++ : y--)) {
              yield Point(a.x, a.y + y);
            }
          }
        }
      }
    }
  }
}

class Cave {
  final Set<Point> rock;
  final Set<Point> sand = <Point>{};
  Point min, max;
  final Point _source = Point(500, 0);

  late Set<Point> _fallingSand = {_source};
  bool floor = false;

  Cave(this.rock, this.min, this.max);

  bool update() {
    final newList = <Point>{};
    for (final s in _fallingSand) {
      if (!floor || s.y + 1 < max.y + 2) {
        // check if sand can move down
        final below = Point(s.x, s.y + 1);
        if (!rock.contains(below) && !sand.contains(below)) {
          newList.add(below);
          if (!floor && !isInBounds(s)) {
            return false;
          }
          continue;
        }

        // check if sand can move down left
        final left = Point(s.x - 1, s.y + 1);
        if (!rock.contains(left) && !sand.contains(left)) {
          newList.add(left);
          if (left.x < min.x) min = Point(left.x, min.y);
          if (!floor && !isInBounds(s)) {
            return false;
          }
          continue;
        }

        // check if sand can move down right
        final right = Point(s.x + 1, s.y + 1);
        if (!rock.contains(right) && !sand.contains(right)) {
          newList.add(right);
          if (right.x > max.x) max = Point(right.x, max.y);
          if (!floor && !isInBounds(s)) {
            return false;
          }
          continue;
        }
      }

      // if sand at rest then stop it
      sand.add(s);

      // if it is blocking the source then we can't produce more
      if (s == _source) return false;
    }

    // add more sand as we didn't finish yet (just keep it coming constantly)
    newList.add(_source);
    _fallingSand = newList;
    return true;
  }

  bool isInBounds(Point p) {
    return p.x >= min.x && p.x <= max.x && p.y >= min.y && p.y <= max.y;
  }

  @override
  String toString() {
    var s = "";
    for (var row = min.y; row <= (floor ? max.y + 2 : max.y); row++) {
      s += "$row ";
      for (var col = min.x; col <= max.x; col++) {
        final p = Point(col, row);
        if ((floor && p.y == max.y + 2) || rock.contains(p)) {
          s += "█";
        } else if (p == _source) {
          s += "+";
        } else if (_fallingSand.contains(p) || sand.contains(p)) {
          s += ".";
        } else {
          s += " ";
        }
      }
      s += "\r\n";
    }
    s += "RESTING SAND:${sand.length} FALLING SAND:${_fallingSand.length}";
    return s;
  }
}

class Point {
  final int x, y;

  Point(this.x, this.y);

  @override
  bool operator ==(Object p) => p is Point && x == p.x && y == p.y;

  @override
  int get hashCode => Object.hash(x, y);
}
