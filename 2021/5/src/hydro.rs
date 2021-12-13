use std::collections::HashMap;
use std::str::FromStr;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Vent {
    x: u16,
    y: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseVentErr;

impl FromStr for Vent {
    type Err = ParseVentErr;
    fn from_str(data: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        let mut v = Vent::default();
        for (i, n) in data
            .split(',')
            .take(2)
            .filter_map(|n| n.parse().ok())
            .enumerate()
        {
            match i {
                0 => v.x = n,
                1 => v.y = n,
                _ => (),
            }
        }
        Ok(v)
    }
}

impl Vent {
    pub fn from(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
pub struct VentLine {
    a: Vent,
    b: Vent,
}

impl VentLine {
    pub fn from(a: Vent, b: Vent) -> Self {
        Self { a, b }
    }

    pub fn vents(&self) -> Vec<Vent> {
        let mut vents = Vec::default();
        //x1 = x2 or y1 = y2 for straight lines this is easy
        if self.a.x == self.b.x || self.a.y == self.b.y {
            for x in self.a.x.min(self.b.x)..=self.a.x.max(self.b.x) {
                for y in self.a.y.min(self.b.y)..=self.a.y.max(self.b.y) {
                    vents.push(Vent::from(x, y));
                }
            }
        } else {
            // diagonals always always have distinct ranges for x and y
            let a = self.a.x.min(self.b.x)..=self.a.x.max(self.b.x);
            let b = self.a.y.min(self.b.y)..=self.a.y.max(self.b.y);

            // need to determine direction of x and y now we have the ranges
            let x_asc = self.a.x < self.b.x;
            let y_asc = self.a.y < self.b.y;

            // cba to work this out any better now
            if x_asc {
                if y_asc {
                    for (x, y) in a.zip(b) {
                        vents.push(Vent::from(x, y));
                    }
                } else {
                    for (x, y) in a.zip(b.rev()) {
                        vents.push(Vent::from(x, y));
                    }
                }
            } else {
                if y_asc {
                    for (x, y) in a.rev().zip(b) {
                        vents.push(Vent::from(x, y));
                    }
                } else {
                    for (x, y) in a.rev().zip(b.rev()) {
                        vents.push(Vent::from(x, y));
                    }
                }
            }
        }
        vents
    }
}

#[derive(Debug)]
pub struct VentProcessor {
    vent_lines: Vec<VentLine>,
}

impl VentProcessor {
    pub fn from(vent_lines: Vec<VentLine>) -> Self {
        Self { vent_lines }
    }

    pub fn calc(&self) -> usize {
        let mut vents: HashMap<Vent, usize> = HashMap::new();
        for vent in self
            .vent_lines
            .iter()
            // .filter_map(|line| {
            //     //x1 = x2 or y1 = y2
            //     if line.a.x == line.b.x || line.a.y == line.b.y {
            //         Some(line)
            //     } else {
            //         None
            //     }
            // })
            .flat_map(|line| line.vents())
        {
            if vents.contains_key(&vent) {
                if let Some(c) = vents.get_mut(&vent) {
                    *c += 1;
                }
            } else {
                vents.insert(vent, 1);
            }
        }
        vents.iter().filter(|(_, count)| count >= &&2).count()
    }
}
