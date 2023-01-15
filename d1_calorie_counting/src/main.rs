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

/* old
fn split_carry_lists(lines: Lines<BufReader<File>>) -> Vec<Vec<u32>> {
    let mut lists = vec![];
    let mut cur_list: Option<Vec<u32>> = None;

    for line in lines {
        if let Ok(ln) = line {
            if let Ok(num) = ln.parse::<u32>() {
                if let Some(ref mut list) = cur_list {
                    list.push(num);
                } else {
                    cur_list = Some(vec![num]);
                }
            } else {
                if let Some(list) = cur_list {
                    lists.push(list);
                }
                cur_list = None;
            }
        }
    }
    lists
}
*/

fn split_carry_lists(lines: Lines<BufReader<File>>) -> Vec<Vec<u32>> {
    lines
        .map(|ln| ln.map_or(None, |s| s.parse::<u32>().ok()))
        .collect::<Vec<_>>()
        .split(|x| x.is_none())
        .map(|list| {
            list.iter()
                .map(|x| *x.as_ref().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn get_max_carry(lists: &[Vec<u32>]) -> u32 {
    lists.iter().map(|list| list.iter().sum()).max().unwrap()
}

fn main() {
    let lines = read_lines("./input/data").unwrap();
    let lists = split_carry_lists(lines);
    let max_carry = get_max_carry(&lists);
    println!("{max_carry}");
}
