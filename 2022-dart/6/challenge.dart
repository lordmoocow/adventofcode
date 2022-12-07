import 'dart:io';
import 'dart:async';

const path = './6/input';

void main() {
  part1();
  part2();
}

void part1() async {
  final markers = File(path).openRead().transform(PacketMarker(4));
  print("Part 1: ${await markers.first}");
}

void part2() async {
  final markers = File(path).openRead().transform(PacketMarker(14));
  print("Part 2: ${await markers.first}");
}

class PacketMarker extends StreamTransformerBase<List<int>, int> {
  final int markerLength;
  late final List<int> marker;

  PacketMarker(this.markerLength) {
    marker = List.filled(markerLength, 0);
  }

  @override
  Stream<int> bind(Stream<List<int>> stream) async* {
    await for (final value in stream) {
      outer:
      for (var i = 0; i < value.length; i++) {
        marker.setAll(0, marker.skip(1));
        marker[markerLength - 1] = value[i];

        if (i > markerLength) {
          for (var x = 0; x < marker.length; x++) {
            for (var y = 0; y < marker.length; y++) {
              if (x != y && marker[x] == marker[y]) {
                continue outer;
              }
            }
          }
          yield i + 1;
        }
      }
    }
  }
}
