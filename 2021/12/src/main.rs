use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let caves = read_input("/workspaces/advent/2021/12/input")?;
    let system = CaveSystem::from(caves);
    println!("{:?}", part1(&system));

    Ok(())
}

fn part1(system: &CaveSystem) -> usize {
    let paths = system.enter();
    paths.len()
}

#[derive(Debug, Eq, PartialEq, Default)]
struct CaveSystem {
    caves: HashMap<String, Cave>,
}

impl CaveSystem {
    pub fn from(data: Vec<String>) -> Self {
        let mut system = Self::default();

        for link in data.iter().map(|x| x.split('-').collect::<Vec<_>>()) {
            let cave = system.caves.entry(link[0].to_string()).or_insert(Cave {
                id: link[0].to_owned(),
                caves: HashSet::new(),
            });
            cave.caves.insert(link[1].to_string());

            let cave = system.caves.entry(link[1].to_string()).or_insert(Cave {
                id: link[1].to_owned(),
                caves: HashSet::new(),
            });
            // if it's not an exit, add a link back to the parent
            if link[1] != "end" {
                cave.caves.insert(link[0].to_string());
            }
        }

        system
    }

    fn enter(&self) -> Vec<Vec<Cave>> {
        let mut paths = Vec::default();
        if let Some(start) = self.caves.get("start") {
            self.explore(&start, Vec::default(), &mut paths, false);
        }
        paths
    }

    fn explore(
        &self,
        cave: &Cave,
        mut breadcrumbs: Vec<Cave>,
        paths: &mut Vec<Vec<Cave>>,
        doubled_up: bool,
    ) {
        breadcrumbs.push(cave.clone());

        if cave.is_end() {
            paths.push(breadcrumbs)
        } else {
            for next in cave
                .caves
                .iter()
                .filter_map(|x| self.caves.get(x))
                .filter(|c| !c.is_start())
            {
                if next.is_end() || next.is_big() {
                    self.explore(next, breadcrumbs.clone(), paths, doubled_up);
                } else if !breadcrumbs.contains(&next) {
                    self.explore(next, breadcrumbs.clone(), paths, doubled_up);
                } else if !doubled_up {
                    self.explore(next, breadcrumbs.clone(), paths, true);
                }
            }
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Cave {
    id: String,
    caves: HashSet<String>,
}

impl Cave {
    pub fn is_big(&self) -> bool {
        self.id == self.id.to_uppercase()
    }

    pub fn is_start(&self) -> bool {
        self.id == "start"
    }

    pub fn is_end(&self) -> bool {
        self.id == "end"
    }
}

fn read_input(path: &str) -> Result<Vec<String>, Error> {
    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let data: Vec<_> = reader
        .lines()
        .into_iter()
        .filter_map(|line| line.ok())
        .collect();
    Ok(data)
}
