use either::Either;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BracketShape {
    Round,
    Angular,
    Square,
    Curly,
}

impl BracketShape {
    pub fn close(&self) -> Bracket {
        match *self {
            BracketShape::Round => Bracket::Close(*self),
            BracketShape::Angular => Bracket::Close(*self),
            BracketShape::Square => Bracket::Close(*self),
            BracketShape::Curly => Bracket::Close(*self),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Bracket {
    Open(BracketShape),
    Close(BracketShape),
}

impl Bracket {
    pub fn shape(&self) -> BracketShape {
        match *self {
            Bracket::Open(s) => s,
            Bracket::Close(s) => s,
        }
    }
}

impl From<char> for Bracket {
    fn from(input: char) -> Self {
        match input {
            '(' => Bracket::Open(BracketShape::Round),
            ')' => Bracket::Close(BracketShape::Round),
            '{' => Bracket::Open(BracketShape::Curly),
            '}' => Bracket::Close(BracketShape::Curly),
            '[' => Bracket::Open(BracketShape::Square),
            ']' => Bracket::Close(BracketShape::Square),
            '<' => Bracket::Open(BracketShape::Angular),
            '>' => Bracket::Close(BracketShape::Angular),
            _ => panic!("Invalid char"),
        }
    }
}

impl Display for Bracket {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Bracket::Open(BracketShape::Round) => '(',
                Bracket::Close(BracketShape::Round) => ')',
                Bracket::Open(BracketShape::Curly) => '{',
                Bracket::Close(BracketShape::Curly) => '}',
                Bracket::Open(BracketShape::Square) => '[',
                Bracket::Close(BracketShape::Square) => ']',
                Bracket::Open(BracketShape::Angular) => '<',
                Bracket::Close(BracketShape::Angular) => '>',
            }
        )
    }
}

pub fn parse_line(line: &str) -> Vec<Bracket> {
    line.chars().map(Bracket::from).collect()
}

pub fn parse_lines(input: &str) -> Vec<Vec<Bracket>> {
    input.lines().map(parse_line).collect()
}

#[derive(PartialEq, Debug)]
pub struct LintError {
    pub expected: BracketShape,
    pub found: BracketShape,
    pub open_pos: usize,
    pub err_pos: usize,
}

pub fn lint_line(line: &[Bracket]) -> Either<Vec<BracketShape>, LintError> {
    let mut stack: Vec<BracketShape> = vec![];
    let mut open_pos: Vec<usize> = vec![];

    for (i, bracket) in line.iter().enumerate() {
        match stack.last() {
            None => {
                stack.push(bracket.shape());
                open_pos.push(0)
            }
            Some(&current) => match current {
                BracketShape::Round => match bracket {
                    Bracket::Open(shape) => {
                        stack.push(*shape);
                        open_pos.push(i);
                    }
                    Bracket::Close(BracketShape::Round) => {
                        stack.pop();
                        open_pos.pop();
                    }
                    _ => {
                        return Either::Right(LintError {
                            expected: BracketShape::Round,
                            found: bracket.shape(),
                            open_pos: *open_pos.last().unwrap(),
                            err_pos: i,
                        });
                    }
                },
                BracketShape::Angular => match bracket {
                    Bracket::Open(shape) => {
                        stack.push(*shape);
                        open_pos.push(i);
                    }
                    Bracket::Close(BracketShape::Angular) => {
                        stack.pop();
                        open_pos.pop();
                    }
                    _ => {
                        return Either::Right(LintError {
                            expected: BracketShape::Angular,
                            found: bracket.shape(),
                            open_pos: *open_pos.last().unwrap(),
                            err_pos: i,
                        });
                    }
                },
                BracketShape::Square => match bracket {
                    Bracket::Open(shape) => {
                        stack.push(*shape);
                        open_pos.push(i);
                    }
                    Bracket::Close(BracketShape::Square) => {
                        stack.pop();
                        open_pos.pop();
                    }
                    _ => {
                        return Either::Right(LintError {
                            expected: BracketShape::Square,
                            found: bracket.shape(),
                            open_pos: *open_pos.last().unwrap(),
                            err_pos: i,
                        });
                    }
                },
                BracketShape::Curly => match bracket {
                    Bracket::Open(shape) => {
                        stack.push(*shape);
                        open_pos.push(i);
                    }
                    Bracket::Close(BracketShape::Curly) => {
                        stack.pop();
                        open_pos.pop();
                    }
                    _ => {
                        return Either::Right(LintError {
                            expected: BracketShape::Curly,
                            found: bracket.shape(),
                            open_pos: *open_pos.last().unwrap(),
                            err_pos: i,
                        });
                    }
                },
            },
        }
    }

    Either::Left(stack)
}

pub fn complete_line(line: &Vec<Bracket>) -> Option<Vec<Bracket>> {
    let lint = lint_line(line);

    match lint {
        Either::Left(stack) => {
            let mut line_out = line.to_owned();

            for shape in stack.iter().rev() {
                line_out.push(shape.close());
            }

            Some(line_out.clone())
        }
        Either::Right(_) => None,
    }
}

pub fn to_complete_score(shape: &BracketShape) -> usize {
    match shape {
        BracketShape::Round => 1,
        BracketShape::Angular => 4,
        BracketShape::Square => 2,
        BracketShape::Curly => 3,
    }
}

pub fn calc_complete_score(line: &Vec<Bracket>) -> Option<usize> {
    let mut score = 0;

    let incomplete_len = line.len();
    let completed = complete_line(line)?;

    for bracket in completed
        .iter()
        .rev()
        .take(completed.len() - incomplete_len)
        .rev()
    {
        score *= 5;
        score += to_complete_score(&bracket.shape());
    }

    Some(score)
}

pub fn middle_complete_score(lines: &[Vec<Bracket>]) -> usize {
    let mut scores: Vec<usize> = vec![];

    for line in lines {
        if let Some(score) = calc_complete_score(line) {
            scores.push(score);
        }
    }

    scores.sort_unstable();

    *scores.get(scores.len() / 2).unwrap()
}

pub fn to_error_score(shape: &BracketShape) -> usize {
    match shape {
        BracketShape::Round => 3,
        BracketShape::Angular => 25137,
        BracketShape::Square => 57,
        BracketShape::Curly => 1197,
    }
}

pub fn calc_syntax_score(lines: &[Vec<Bracket>]) -> usize {
    let mut score = 0;

    for line in lines {
        let lint = lint_line(line);

        if let Either::Right(lint_err) = lint {
            score += to_error_score(&lint_err.found);
        }
    }

    score
}

pub fn lint_graphically(lines: &[Vec<Bracket>]) {
    let mut score = 0;

    for line in lines {
        let lint = lint_line(line);

        for c in line {
            print!("{}", c);
        }
        println!();

        if let Either::Right(lint_err) = lint {
            let line_score = to_error_score(&lint_err.found);

            if lint_err.open_pos > 0 {
                for _ in 0..lint_err.open_pos {
                    print!(" ");
                }
            }
            print!("^");
            for _ in 0..(lint_err.err_pos - lint_err.open_pos - 1) {
                print!("-");
            }
            println!(
                "^-Expected {:?}, Found {:?}     {} + {}",
                lint_err.expected, lint_err.found, score, line_score
            );

            score += line_score;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lint_line() {
        assert_eq!(
            lint_line(&parse_line("{([(<{}[<>[]}>{[]{[(<()>")),
            (Either::Right(LintError {
                expected: BracketShape::Square,
                found: BracketShape::Curly,
                open_pos: 7,
                err_pos: 12
            }))
        );

        assert_eq!(
            lint_line(&parse_line("[[<[([]))<([[{}[[()]]]")),
            (Either::Right(LintError {
                expected: BracketShape::Square,
                found: BracketShape::Round,
                open_pos: 3,
                err_pos: 8
            }))
        );

        assert_eq!(
            lint_line(&parse_line("[{[{({}]{}}([{[{{{}}([]")),
            (Either::Right(LintError {
                expected: BracketShape::Round,
                found: BracketShape::Square,
                open_pos: 4,
                err_pos: 7
            }))
        );

        assert_eq!(
            lint_line(&parse_line("[<(<(<(<{}))><([]([]()")),
            (Either::Right(LintError {
                expected: BracketShape::Angular,
                found: BracketShape::Round,
                open_pos: 7,
                err_pos: 10
            }))
        );

        assert_eq!(
            lint_line(&parse_line("<{([([[(<>()){}]>(<<{{")),
            (Either::Right(LintError {
                expected: BracketShape::Square,
                found: BracketShape::Angular,
                open_pos: 5,
                err_pos: 16
            }))
        );
    }

    #[test]
    fn test_score() {
        let lines = parse_lines(
            "[({(<(())[]>[[{[]{<()<>>\n\
                                                [(()[<>])]({[<{<<[]>>(\n\
                                                {([(<{}[<>[]}>{[]{[(<()>\n\
                                                (((({<>}<{<{<>}{[]{[]{}\n\
                                                [[<[([]))<([[{}[[()]]]\n\
                                                [{[{({}]{}}([{[{{{}}([]\n\
                                                {<[[]]>}<{[{[{[]{()[[[]\n\
                                                [<(<(<(<{}))><([]([]()\n\
                                                <{([([[(<>()){}]>(<<{{\n\
                                                <{([{{}}[<[[[<>{}]]]>[]]",
        );

        assert_eq!(calc_syntax_score(&lines), 26397);
    }

    #[test]
    fn test_complete() {
        assert_eq!(
            complete_line(&parse_line("[({(<(())[]>[[{[]{<()<>>"))
                .unwrap()
                .iter()
                .rev()
                .take(8)
                .rev()
                .copied()
                .collect::<Vec<Bracket>>(),
            parse_line("}}]])})]")
        );

        assert_eq!(
            complete_line(&parse_line("[(()[<>])]({[<{<<[]>>("))
                .unwrap()
                .iter()
                .rev()
                .take(6)
                .rev()
                .copied()
                .collect::<Vec<Bracket>>(),
            parse_line(")}>]})")
        );

        assert_eq!(
            complete_line(&parse_line("(((({<>}<{<{<>}{[]{[]{}"))
                .unwrap()
                .iter()
                .rev()
                .take(9)
                .rev()
                .copied()
                .collect::<Vec<Bracket>>(),
            parse_line("}}>}>))))")
        );

        assert_eq!(
            complete_line(&parse_line("{<[[]]>}<{[{[{[]{()[[[]"))
                .unwrap()
                .iter()
                .rev()
                .take(9)
                .rev()
                .copied()
                .collect::<Vec<Bracket>>(),
            parse_line("]]}}]}]}>")
        );

        assert_eq!(
            complete_line(&parse_line("<{([{{}}[<[[[<>{}]]]>[]]"))
                .unwrap()
                .iter()
                .rev()
                .take(4)
                .rev()
                .copied()
                .collect::<Vec<Bracket>>(),
            parse_line("])}>")
        );
    }

    #[test]
    fn test_complete_score() {
        assert_eq!(
            calc_complete_score(&parse_line("[({(<(())[]>[[{[]{<()<>>")),
            Some(288957)
        );
        assert_eq!(
            calc_complete_score(&parse_line("[(()[<>])]({[<{<<[]>>(")),
            Some(5566)
        );
        assert_eq!(
            calc_complete_score(&parse_line("(((({<>}<{<{<>}{[]{[]{}")),
            Some(1480781)
        );
        assert_eq!(
            calc_complete_score(&parse_line("{<[[]]>}<{[{[{[]{()[[[]")),
            Some(995444)
        );
        assert_eq!(
            calc_complete_score(&parse_line("<{([{{}}[<[[[<>{}]]]>[]]")),
            Some(294)
        );
    }

    #[test]
    fn test_middle_score() {
        let lines = parse_lines(
            "[({(<(())[]>[[{[]{<()<>>\n\
                                                [(()[<>])]({[<{<<[]>>(\n\
                                                {([(<{}[<>[]}>{[]{[(<()>\n\
                                                (((({<>}<{<{<>}{[]{[]{}\n\
                                                [[<[([]))<([[{}[[()]]]\n\
                                                [{[{({}]{}}([{[{{{}}([]\n\
                                                {<[[]]>}<{[{[{[]{()[[[]\n\
                                                [<(<(<(<{}))><([]([]()\n\
                                                <{([([[(<>()){}]>(<<{{\n\
                                                <{([{{}}[<[[[<>{}]]]>[]]",
        );

        assert_eq!(middle_complete_score(&lines), 288957);
    }
}
