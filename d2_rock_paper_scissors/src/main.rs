use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

#[derive(Clone, Copy, Debug)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

fn get_shape_score(shape: Shape) -> u32 {
    shape as u32
}

fn get_shape(s: &str) -> Option<Shape> {
    match s {
        "A" | "X" => Some(Shape::Rock),
        "B" | "Y" => Some(Shape::Paper),
        "C" | "Z" => Some(Shape::Scissors),
        _ => None,
    }
}

#[derive(Clone, Copy, Debug)]
enum RoundResult {
    Lost = 0,
    Draw = 3,
    Won = 6,
}

fn get_result_score(result: RoundResult) -> u32 {
    result as u32
}

fn get_result(s: &str) -> Option<RoundResult> {
    match s {
        "X" => Some(RoundResult::Lost),
        "Y" => Some(RoundResult::Draw),
        "Z" => Some(RoundResult::Won),
        _ => None,
    }
}

fn get_round_result(opponent: Shape, mine: Shape) -> RoundResult {
    match (opponent, mine) {
        (Shape::Rock, Shape::Paper) => RoundResult::Won,
        (Shape::Rock, Shape::Scissors) => RoundResult::Lost,
        (Shape::Paper, Shape::Rock) => RoundResult::Lost,
        (Shape::Paper, Shape::Scissors) => RoundResult::Won,
        (Shape::Scissors, Shape::Rock) => RoundResult::Won,
        (Shape::Scissors, Shape::Paper) => RoundResult::Lost,
        _ => RoundResult::Draw,
    }
}

fn get_round_score(opponent: Shape, mine: Shape) -> u32 {
    let result = get_round_result(opponent, mine);
    get_result_score(result)
}

fn get_line_score_p1(ln: &str) -> u32 {
    let shapes = ln.split(' ').collect::<Vec<_>>();
    let opponent = get_shape(shapes[0]).unwrap();
    let mine = get_shape(shapes[1]).unwrap();
    get_shape_score(mine) + get_round_score(opponent, mine)
}

fn get_shape_from_result(opponent: Shape, result: RoundResult) -> Shape {
    match (result, opponent) {
        (RoundResult::Draw, mine) => mine,
        (RoundResult::Lost, Shape::Rock) => Shape::Scissors,
        (RoundResult::Lost, Shape::Paper) => Shape::Rock,
        (RoundResult::Lost, Shape::Scissors) => Shape::Paper,
        (RoundResult::Won, Shape::Rock) => Shape::Paper,
        (RoundResult::Won, Shape::Paper) => Shape::Scissors,
        (RoundResult::Won, Shape::Scissors) => Shape::Rock,
    }
}

fn get_line_score_p2(ln: &str) -> u32 {
    let shapes = ln.split(' ').collect::<Vec<_>>();
    let opponent = get_shape(shapes[0]).unwrap();
    let result = get_result(shapes[1]).unwrap();
    let mine = get_shape_from_result(opponent, result);
    get_shape_score(mine) + get_result_score(result)
}

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn main() {
    let lines = read_lines("./input/data")
        .unwrap()
        .filter_map(|ln| ln.ok())
        .collect::<Vec<_>>();

    let total_score_p1: u32 = lines.iter().map(|ln| get_line_score_p1(ln)).sum();
    println!("total p1 score: {total_score_p1}");

    let total_score_p2: u32 = lines.iter().map(|ln| get_line_score_p2(ln)).sum();
    println!("total p2 score: {total_score_p2}");
}
