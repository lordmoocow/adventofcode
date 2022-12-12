import 'dart:async';
import 'dart:io';
import 'dart:convert';

const path = './12/input';

void main() {
  part1();
  part2();
}

void part1() async {
  final input = await parseMap();

  final path = shortestPath(input.start, input.finish, input.map);
  print("Part 1: $path");
}

void part2() async {
  final input = await parseMap();

  int? shortest;
  for (var start in input.map.entries.where((x) => x.value == 1)) {
    final steps = shortestPath(start.key, input.finish, input.map);
    if (shortest == null || (steps != null && steps < shortest)) {
      shortest = steps;
    }
  }
  print("Part 2: $shortest");
}

int? shortestPath(Point start, Point finish, Map<Point, int> map) {
  // track visited points and their "cost"
  final visited = <Point, int>{};

  // keep track of the parent of each point along it's shortest route
  final parents = <Point, Point>{};

  // add start position as initial query point
  final queries = <Query>[];
  visited[start] = map[start]!;
  queries.add(Query(start, map[start]!));

  while (queries.isNotEmpty) {
    // no priority queue so a sort will do i guess;
    // get the lowest cost query
    queries.sort((a, b) => b.v.compareTo(a.v));
    final q = queries.removeLast();

    // if this is our target then we are done - just count how many back to start
    // this could probably be better
    if (q.p == finish) {
      var steps = 1;
      var parent = parents[q.p];
      while (parent != start) {
        steps++;
        parent = parents[parent];
      }
      return steps;
    }

    // get the cost of reaching our current point
    final cost = visited[q.p] ?? 0;

    // check each neigboring point
    for (final n in nieghbours(q.p, map)) {
      // make sure it's not too much effort to go this way
      final stepSize = map[n]! - map[q.p]!;
      if (stepSize <= 1) {
        // calculate cost to go this route
        final nCost = cost + stepSize + 1;

        // check if that is cheaper than anything we may have found already
        final prevCost = visited[n];

        if (prevCost == null || nCost < prevCost) {
          // this is the best we found so far so let's keep going!
          parents[n] = q.p;
          visited[n] = nCost;
          queries.add(Query(n, nCost));
        }
      }
    }
  }

  return null;
}

class Query {
  final Point p;
  final int v;

  const Query(this.p, this.v);
}

class Point {
  final int x, y;

  const Point(this.x, this.y);

  @override
  bool operator ==(Object p) => p is Point && x == p.x && y == p.y;

  @override
  int get hashCode => Object.hash(x, y);

  @override
  String toString() => "[$x,$y]";
}

Iterable<Point> nieghbours(final Point p, final Map<Point, int> map) sync* {
  final diffs = [
    [1, 0],
    [0, -1],
    [-1, 0],
    [0, 1]
  ];
  for (final d in diffs) {
    final n = Point(p.x + d[0], p.y + d[1]);
    if (map.containsKey(n)) {
      yield n;
    }
  }
}

Future<MapData> parseMap() async {
  Point? start;
  Point? finish;
  final map = <Point, int>{};

  final stream =
      File(path).openRead().transform(utf8.decoder).transform(LineSplitter());

  var row = 0;
  await for (final line in stream) {
    for (var col = 0; col < line.codeUnits.length; col++) {
      final p = Point(col, row);
      switch (line.codeUnitAt(col)) {
        case 83: //start
          start = p;
          map[p] = 1;
          break;
        case 69: //end
          finish = p;
          map[p] = 26;
          break;
        default:
          map[p] = line.codeUnitAt(col) - 96;
          break;
      }
    }
    row++;
  }

  return MapData(start!, finish!, map);
}

class MapData {
  final Point start, finish;
  final Map<Point, int> map;

  const MapData(this.start, this.finish, this.map);
}
