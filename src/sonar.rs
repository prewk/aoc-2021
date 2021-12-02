#[derive(PartialEq)]
pub struct Depth {
    depths: Vec<u64>,
}

impl From<&str> for Depth {
    fn from(input: &str) -> Self {
        Depth {
            depths: input
                .lines()
                .map(|line| line.parse::<u64>())
                .filter_map(Result::ok)
                .collect()
        }
    }
}

fn get_window(depths: &Vec<u64>) -> u64 {
    let mut depth = 0;
    if let Some(v) = depths.get(0) {
        depth += v
    }
    if let Some(v) = depths.get(1) {
        depth += v
    }
    if let Some(v) = depths.get(2) {
        depth += v
    }

    depth
}

fn get_windows(depths: &Vec<u64>) -> Vec<u64> {
    let mut windows = vec![];

    for i in 0..depths.len() {
        windows.push(get_window(&depths.iter().skip(i).map(|v| *v).collect()));
    }

    windows
}

fn get_increases(depths: &Vec<u64>) -> u64 {
    let mut increases = 0u64;
    let mut opt_last = None;
    for depth in depths {
        if let Some(l) = opt_last {
            if depth > l {
                increases += 1;
            }
        }

        opt_last = Some(depth);
    }

    increases
}

impl Depth {
    pub fn get_increases(&self) -> u64 {
        get_increases(&self.depths)
    }

    pub fn get_windowed_increases(&self) -> u64 {
        get_increases(&get_windows(&self.depths))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increases() {
        let input = "199\n\
                           200\n\
                           208\n\
                           210\n\
                           200\n\
                           207\n\
                           240\n\
                           269\n\
                           260\n\
                           263";
        let depth = Depth::from(input);

        assert_eq!(depth.get_increases(), 7);
    }

    #[test]
    fn test_windowed_increases() {
        let input = "199\n\
                           200\n\
                           208\n\
                           210\n\
                           200\n\
                           207\n\
                           240\n\
                           269\n\
                           260\n\
                           263";
        let depth = Depth::from(input);

        assert_eq!(depth.get_windowed_increases(), 5);
    }
}