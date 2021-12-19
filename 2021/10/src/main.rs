use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let syntax = read_input("/workspaces/advent/2021/10/input")?;

    //println!("{:?}", &syntax);
    println!("{:?}", part1(&syntax));

    Ok(())
}

fn part1(syntax: &[String]) -> usize {
    let mut score = 0;
    let mut tmp = Vec::default();
    for line in syntax {
        tmp.clear();
        for x in line.chars() {
            match x {
                '(' | '[' | '{' | '<' => tmp.push(Chunk::from(Symbol::from(x))),
                ')' | ']' | '}' | '>' => {
                    if let Some(chunk) = tmp.pop() {
                        if !chunk.is_complete(Symbol::from(x)) {
                            score += Symbol::score(Symbol::from(x));
                            break;
                        }
                    }
                },
                _ => (),
            }
        }
    }
    score
}



#[derive(Eq,PartialEq)]
enum Symbol {
    OpenBracket,
    OpenSquareBracket,
    OpenSquigglyBracket,
    OpenAngleBracket,
    CloseBracket,
    CloseSquareBracket,
    CloseSquigglyBracket,
    CloseAngleBracket,
}

impl Symbol {
    pub fn from(symbol: char) -> Self {
        match symbol {
            '(' => Self::OpenBracket,
            '[' => Self::OpenSquareBracket,
            '{' => Self::OpenSquigglyBracket,
            '<' => Self::OpenAngleBracket,
            ')' => Self::CloseBracket,
            ']' => Self::CloseSquareBracket,
            '}' => Self::CloseSquigglyBracket,
            '>' => Self::CloseAngleBracket,
            _ => panic!(),
        }
    }

    pub fn score(symbol: Self) -> usize {
        match symbol {
            Symbol::CloseBracket => 3,
            Symbol::CloseSquareBracket => 57,
            Symbol::CloseSquigglyBracket => 1197,
            Symbol::CloseAngleBracket => 25137,
            _ => 0,
        }
    }
}

struct Chunk {
    kind: Symbol
}

impl Chunk {
    pub fn from(kind: Symbol) -> Self {
        Self {
            kind,
        }
    }

    pub fn is_complete(&self, symbol: Symbol) -> bool {
        if let Some(closer) = match self.kind {
            Symbol::OpenBracket => Some(Symbol::CloseBracket),
            Symbol::OpenSquareBracket => Some(Symbol::CloseSquareBracket),
            Symbol::OpenSquigglyBracket => Some(Symbol::CloseSquigglyBracket),
            Symbol::OpenAngleBracket => Some(Symbol::CloseAngleBracket),
            _ => None,
        } {
            symbol == closer
        } else {
            false
        }
    }
}

// fn part2(heightmap: &Vec<Vec<usize>>) -> usize {
//     let lowpoints = lowpoints(&heightmap);
//     let mut visited = HashSet::default(); // need keep track of the points that have already been counted
//     let mut basins = Vec::default();

//     for point in lowpoints {
//         basins.push(search_basin(&point, &heightmap, &mut visited))
//     }

//     basins.sort_unstable();
//     basins.iter().rev().take(3).product()
// }

fn read_input(path: &str) -> Result<Vec<String>, Error> {
    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let data = reader
        .lines()
        .into_iter()
        .filter_map(|line| {
            if let Ok(line) = line {
                return Some(line);
            }
            None
        })
        .collect();

    Ok(data)
}
