use std::fs::read_to_string;
use std::io::Error;

fn main() -> Result<(), Error> {
    let (sheet, instructions) = read_input("/workspaces/advent/2021/13/input")?;
    //println!("{:?}", &sheet);
    println!("{:?}", &sheet.total_dots());
    println!("{:?}", part1(&sheet, &instructions));
    part2(&sheet, &instructions);

    Ok(())
}

fn part1(sheet: &Sheet, instructions: &[Instruction]) -> usize {
    let folded_sheet = sheet.fold(&instructions[0]);
    folded_sheet.total_dots()
}

fn part2(sheet: &Sheet, instructions: &[Instruction]) {
    let folded_sheet = instructions
        .iter()
        .fold(sheet.clone(), |sheet, instruction| sheet.fold(instruction));
    for line in folded_sheet.data {
        for x in line {
            if let Some(_) = x {
                print!("#");
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }
}

#[derive(Debug, Clone)]
struct Sheet {
    data: Vec<Vec<Option<Mark>>>,
}

impl Sheet {
    pub fn with_dimensions(x: usize, y: usize) -> Self {
        Self {
            data: vec![vec![None; x + 1]; y + 1],
        }
    }

    pub fn fold(&self, instruction: &Instruction) -> Sheet {
        match instruction {
            Instruction::FoldX(x) => {
                let data: Vec<Vec<_>> = self
                    .data
                    .iter()
                    .map(|y| {
                        y[..*x]
                            .iter()
                            .zip(y[*x..].iter().rev())
                            .map(|(a, b)| {
                                if a.is_some() || b.is_some() {
                                    Some(Mark::Dot)
                                } else {
                                    None
                                }
                            })
                            .collect()
                    })
                    .collect();

                Sheet { data }
            }
            Instruction::FoldY(y) => {
                let data: Vec<Vec<_>> = self.data[..*y]
                    .iter()
                    .zip(self.data[*y..].iter().rev())
                    .map(|(a, b)| {
                        a.iter()
                            .zip(b)
                            .map(|(a, b)| {
                                if a.is_some() || b.is_some() {
                                    Some(Mark::Dot)
                                } else {
                                    None
                                }
                            })
                            .collect()
                    })
                    .collect();

                Sheet { data }
            }
        }
    }

    pub fn total_dots(&self) -> usize {
        self.data.iter().fold(0, |total, x| {
            total + x.iter().filter_map(|x| x.as_ref()).count()
        })
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum Mark {
    Dot,
}

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    FoldX(usize),
    FoldY(usize),
}

fn read_input(path: &str) -> Result<(Sheet, Vec<Instruction>), Error> {
    let input = read_to_string(path)?;
    // each section of the input is separated by double line feed
    let mut iter = input.split("\n\n");

    let mut max_x = 0;
    let mut max_y = 0;
    let data: Vec<_> = iter
        .next()
        .unwrap()
        .split('\n')
        .map(|coord| {
            let mut coord = coord.split(',').map(|x| x.parse().unwrap());
            (coord.next().unwrap(), coord.next().unwrap())
        })
        .inspect(|(x, y)| {
            max_x = max_x.max(*x);
            max_y = max_y.max(*y);
        })
        .collect();

    let mut sheet = Sheet::with_dimensions(max_x, max_y);
    for (x, y) in data {
        sheet.data[y][x].replace(Mark::Dot);
    }

    let instructions = iter
        .next()
        .unwrap()
        .split('\n')
        .filter_map(|x| {
            if let Some(fold) = x.split_whitespace().skip(2).next() {
                let fold: Vec<_> = fold.split('=').collect();
                return match fold[0] {
                    "x" => Some(Instruction::FoldX(fold[1].parse().unwrap())),
                    "y" => Some(Instruction::FoldY(fold[1].parse().unwrap())),
                    _ => None,
                };
            }
            None
        })
        .collect();

    Ok((sheet, instructions))
}
