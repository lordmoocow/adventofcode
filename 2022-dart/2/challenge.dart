import 'dart:io';
import 'dart:convert';
import 'dart:async';

const path = './2/input';

void main() {
  part1();
  part2();
}

void part1() async {
  final strategies = File(path)
      .openRead()
      .transform(utf8.decoder)
      .transform(LineSplitter())
      .transform(Strategies());

  var score = 0;
  await for (final strategy in strategies) {
    score += strategy.play();
  }
  print('Part 1: $score');
}

void part2() async {
  final strategies = File(path)
      .openRead()
      .transform(utf8.decoder)
      .transform(LineSplitter())
      .transform(Strategies());

  var score = 0;
  await for (final strategy in strategies) {
    score += strategy.strategicPlay();
  }
  print('Part 2: $score');
}

enum Shape {
  rock(score: 1),
  paper(score: 2),
  scissors(score: 3);

  final int score;

  const Shape({required this.score});

  static Shape from(int i) {
    switch (i) {
      case 1:
        return Shape.rock;

      case 2:
        return Shape.paper;

      case 3:
        return Shape.scissors;

      default:
        throw Exception('Unkown shape "$i"');
    }
  }

  Shape loses() {
    return Shape.from(((score) % 3) + 1);
  }

  Shape beats() {
    return Shape.from(((score + 1) % 3) + 1);
  }
}

class Strategy {
  static const int win = 6;
  static const int draw = 3;

  final Shape predicted;
  final Shape response;

  const Strategy(this.predicted, this.response);

  int play() {
    var score = response.score;
    if (response == predicted) {
      score += draw;
    } else if (response.beats() == predicted) {
      score += win;
    }
    return score;
  }

  int strategicPlay() {
    switch (response) {
      case Shape.rock:
        return predicted.beats().score;

      case Shape.paper:
        return predicted.score + draw;

      case Shape.scissors:
        return predicted.loses().score + win;

      default:
        throw Exception('Unknown strategy "$response"');
    }
  }
}

class Strategies extends StreamTransformerBase<String, Strategy> {
  @override
  Stream<Strategy> bind(Stream<String> stream) async* {
    await for (final value in stream) {
      if (value != '') {
        yield Strategy(Shape.from(value.codeUnitAt(0) - 64),
            Shape.from(value.codeUnitAt(2) - 87));
      }
    }
  }
}
