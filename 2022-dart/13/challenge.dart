import 'dart:async';
import 'dart:io';
import 'dart:convert';

const path = './13/input';

void main() {
  part1();
  part2();
}

void part1() async {
  final stream = File(path)
      .openRead()
      .transform(utf8.decoder)
      .transform(LineSplitter())
      .transform(PacketParser());

  var index = 0;
  var sum = 0;
  await for (final pair in stream) {
    //print("$pair\r\n");
    index++;
    if (pair.left.compareTo(pair.right) < 0) {
      sum += index;
    }
  }
  print("Part 1: $sum");
}

void part2() async {
  final packets = await File(path)
      .openRead()
      .transform(utf8.decoder)
      .transform(LineSplitter())
      .transform(PacketParser())
      // split out the packet pairs
      .transform(StreamTransformer.fromHandlers(
    handleData: (data, sink) {
      sink.add(data.left);
      sink.add(data.right);
    },
  )).toList();

  // create the special packets, keeping a reference for later
  final packetA = Packet.parse("[[2]]");
  final packetB = Packet.parse("[[6]]");
  packets.add(packetA);
  packets.add(packetB);

  // sort all the packets
  packets.sort();

  // find the special packets and do the maths
  var decoder = (packets.indexOf(packetA) + 1) * (packets.indexOf(packetB) + 1);
  print("Part 2: $decoder");
}

class PacketParser extends StreamTransformerBase<String, Pair> {
  @override
  Stream<Pair> bind(Stream<String> stream) async* {
    final list = <String>[];
    await for (final packet in stream) {
      if (packet.isEmpty) continue;

      list.add(packet);
      if (list.length == 2) {
        yield Pair(Packet.parse(list[0]), Packet.parse(list[1]));
        list.clear();
      }
    }
  }
}

class Pair {
  final Packet left, right;
  const Pair(this.left, this.right);

  @override
  String toString() => "$left\r\n$right";
}

class Packet extends Comparable {
  final List<Packet>? list;
  final int? value;

  Packet._(this.value, this.list);
  Packet(List<Packet> list) : this._(null, list);
  Packet.int(int v) : this._(v, null);

  factory Packet.parse(final String s) {
    // if we don't have [ it's a single value (hopefully)
    if (!s.startsWith("[")) {
      return Packet.int(int.parse(s));
    }

    // the packets within this packet list
    var packets = <Packet>[];

    // empty packet
    if (s.length == 2) {
      return Packet(packets);
    }

    // remove outer [ ]
    final sub = s.substring(1, s.length - 1);

    // track nesting
    var brackets = 0;

    // track start of next string to parse
    var subI = 0;

    // find start and end of each sub packet
    for (var i = 0; i < sub.codeUnits.length; i++) {
      final c = String.fromCharCode(sub.codeUnitAt(i));
      if (c == "[") brackets++;
      if (c == "]") brackets--;
      if (c == "," && brackets == 0) {
        // parse all the way down
        packets.add(Packet.parse(sub.substring(subI, i)));
        // next sub string starts after this
        subI = i + 1;
      }
    }
    // catch any stragglers that don't have , or ] at the end
    packets.add(Packet.parse(sub.substring(subI, sub.length)));

    return Packet(packets);
  }

  bool isInt() => value != null;

  @override
  String toString() => isInt() ? "$value" : "$list";

  @override
  int compareTo(other) {
    if (other is! Packet) throw UnimplementedError();

    // both ints, compare
    if (isInt() && other.isInt()) return value!.compareTo(other.value!);

    // both lists, iterate and compate items in list
    if (!isInt() && !other.isInt()) {
      for (var i = 0; i < list!.length; i++) {
        // if b runs out first then it is wrong
        if (i > other.list!.length - 1) {
          return 1;
        }
        // if a is ever less than b it is correct
        final c = list![i].compareTo(other.list![i]);
        if (c != 0) return c;
      }
      // left ran out first?
      return list!.length < other.list!.length ? -1 : 0;
    }

    // mixed, convert to single item list if one value is int
    return (isInt() ? Packet([this]) : this)
        .compareTo(other.isInt() ? Packet([other]) : other);
  }
}
