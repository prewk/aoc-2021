use crate::fold::*;

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Instructions {
    Instructions::from(input)
}

#[aoc(day13, part1)]
pub fn part1(instr: &Instructions) -> usize {
    Paper::from(instr).fold(instr.folds.first().unwrap()).unwrap().count_visible_dots()
}

#[aoc(day13, part2)]
pub fn part2(instr: &Instructions) -> String {
    let foldable = FoldablePaper::from(instr);

    foldable.paper.dump("fold-0.png", instr.folds.get(0));
    let mut last = None;
    for (i, fold) in foldable.enumerate() {
        fold.dump(format!("fold-{}.png", i + 1).as_str(), instr.folds.get(i + 1));

        last = Some(fold);
    }

    format!("\n{}", last.unwrap())
}