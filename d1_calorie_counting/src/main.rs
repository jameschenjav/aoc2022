#![feature(binary_heap_into_iter_sorted)]
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn split_carry_lists(lines: impl Iterator<Item = String>) -> Vec<Vec<u32>> {
    lines
        .map(|ln| ln.parse::<u32>().ok())
        .collect::<Vec<_>>()
        .split(|x| x.is_none())
        .map(|list| {
            list.iter()
                .map(|x| *x.as_ref().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn get_calorie_list(lists: &[Vec<u32>]) -> Vec<u32> {
    lists.iter().map(|list| list.iter().sum()).collect()
}

fn get_sum_of_top(top: usize, calorie_list: &[u32]) -> u32 {
    BinaryHeap::from_iter(calorie_list)
        .into_iter_sorted()
        .take(top)
        .sum()
}

fn main() {
    let lines = read_lines("./input/data").unwrap().filter_map(|ln| ln.ok());
    let carry_lists = split_carry_lists(lines);
    let calorie_list = get_calorie_list(&carry_lists);

    let max_carry = get_sum_of_top(1, &calorie_list);
    println!("part 1 - max carry: {max_carry}");

    let sum_top_3 = get_sum_of_top(3, &calorie_list);
    println!("part 2 - sum of top 3: {sum_top_3}");
}
