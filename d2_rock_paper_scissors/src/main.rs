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

fn get_compare_score(opponent: Shape, mine: Shape) -> u32 {
    let m = get_shape_score(mine);
    let o = get_shape_score(opponent);

    if m == o {
        return 3;
    }
    if (m + 3 - o) % 3 == 1 {
        6
    } else {
        0
    }
}

fn get_shape(s: &str) -> Option<Shape> {
    match s {
        "A" | "X" => Some(Shape::Rock),
        "B" | "Y" => Some(Shape::Paper),
        "C" | "Z" => Some(Shape::Scissors),
        _ => None,
    }
}

fn get_line_score(ln: &str) -> u32 {
    let shapes = ln.split(' ').collect::<Vec<_>>();
    let opponent = get_shape(shapes[0]).unwrap();
    let mine = get_shape(shapes[1]).unwrap();
    // let s = get_shape_score(m);
    // let c = get_compare_score(o, m);
    // println!("{:?} vs {:?} shape: {s} + won: {c}", o, m);
    get_shape_score(mine) + get_compare_score(opponent, mine)
}

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn main() {
    let lines = read_lines("./input/data").unwrap().filter_map(|ln| ln.ok());
    let total_score: u32 = lines.map(|ln| get_line_score(&ln)).sum();
    print!("total score: {total_score}");
}
