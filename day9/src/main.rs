use aochelpers::get_daily_input;

#[derive(Debug)]
struct Span {
    index: usize,
    sector: usize,
    size: usize,
}

// Disk is a tuple of two vectors, the first one is the files, the second one is the free space
type Disk = (Vec<Span>, Vec<Span>);

struct Input<'a> {
    data: &'a str,
}

fn part1(data: &str) -> usize {
    let (mut files, free) = parse_input(data, |f, i, p, s| {
        f.extend((0..s).map(|k| Span {
            index: i,
            sector: p + k,
            size: 1,
        }))
    });
    let mut l = 0;
    let mut r = files.len() - 1;
    // swap free space and files
    while free[l].sector < files[r].sector {
        (0..free[l].size).for_each(|i| {
            files[r - i].sector = free[l].sector;
        });
        r -= free[l].size;
        l += 1;
    }
    checksum(&files)
}

fn part2(data: &str) -> usize {
    let (mut files, mut free) = parse_input(data, |f, i, p, s| {
        f.push(Span {
            index: i,
            sector: p,
            size: s,
        })
    });
    files.iter_mut().rev().for_each(|f| {
        if let Some(ff) = free
            .iter_mut()
            .find(|ff| ff.sector < f.sector && ff.size >= f.size)
        {
            f.sector = ff.sector;
            ff.sector += f.size;
            ff.size -= f.size;
        }
    });
    checksum(&files)
}

fn parse_input(data: &str, extend: impl Fn(&mut Vec<Span>, usize, usize, usize) -> ()) -> Disk {
    let mut sector = 0;
    let mut files = Vec::new();
    let mut free = Vec::new();
    let mut r = vec![&mut files, &mut free];
    for (i, c) in data.chars().enumerate() {
        parse(c).map(|v| {
            extend(r[i % 2], i / 2, sector, v as usize);
            sector += v as usize;
        });
    }
    (files, free)
}

fn parse(c: char) -> Option<u8> {
    c.to_digit(10).map(|v| v as u8)
}

fn checksum(files: &Vec<Span>) -> usize {
    files
        .iter()
        .map(|f| {
            (f.sector..f.sector + f.size)
                .map(|s| f.index * s)
                .sum::<usize>()
        })
        .sum()
}

fn solve(input: &Input) -> (usize, usize) {
    let part1 = part1(input.data);
    let part2 = part2(input.data);
    (part1, part2)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = get_daily_input(9, 2024)?;
    let input = Input { data: &data };
    let (part1, part2) = solve(&input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTDATA: &str = "2333133121414131402";

    #[test]
    fn test_part1() {
        let input = Input { data: TESTDATA };
        assert_eq!(solve(&input).0, 1928);
    }

    // #[test]
    // fn test_part2() {
    //     let input = parse_data(TESTDATA);
    //     assert_eq!(solve(&input).1, 0);
    // }
}
