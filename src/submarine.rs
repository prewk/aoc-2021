#[derive(PartialEq, Debug, Clone)]
pub enum Instr {
    Forward(u64),
    Down(u64),
    Up(u64),
}

impl From<&str> for Instr {
    fn from(input: &str) -> Self {
        let parts: Vec<&str> = input.split(' ').collect();

        match (parts.get(0), parts.get(1)) {
            (Some(&"forward"), Some(v)) => Instr::Forward(v.parse::<u64>().expect("Erroneous input")),
            (Some(&"down"), Some(v)) => Instr::Down(v.parse::<u64>().expect("Erroneous input")),
            (Some(&"up"), Some(v)) => Instr::Up(v.parse::<u64>().expect("Erroneous input")),
            _ => panic!("Invalid instruction"),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Submarine {
    pub instr: Vec<Instr>,
}

impl From<&str> for Submarine {
    fn from(input: &str) -> Self {
        Submarine {
            instr: input.lines().map(Instr::from).collect()
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Water {
    sub: Submarine,
    sub_horizontal: u64,
    sub_depth: u64,
}

impl Water {
    pub fn submerge(sub: &Submarine) -> Water {
        Water {
            sub: sub.clone(),
            sub_horizontal: 0,
            sub_depth: 0,
        }
    }

    pub fn vroom(&mut self) {
        for instr in &self.sub.instr {
            match instr {
                Instr::Forward(v) => {
                    self.sub_horizontal += v;
                }
                Instr::Down(v) => {
                    self.sub_depth += v;
                }
                Instr::Up(v) => {
                    self.sub_depth -= v;
                }
            }
        }
    }

    pub fn position_mult(&self) -> u64 {
        self.sub_horizontal * self.sub_depth
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_mult() {
        let input = "forward 5\n\
                           down 5\n\
                           forward 8\n\
                           up 3\n\
                           down 8\n\
                           forward 2";
        let sub = Submarine::from(input);

        let mut water = Water::submerge(&sub);
        water.vroom();

        assert_eq!(water.position_mult(), 150);
    }
}