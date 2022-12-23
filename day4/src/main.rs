use itertools::Itertools;
use std::ops::Range;

fn main() -> color_eyre::Result<()> {
    let redundant = include_str!("input.txt")
        .lines()
        .map(|l| {
            l.split(',')
                .map(|range| {
                    range
                        .split('-')
                        .map(|n| n.parse().expect("range start/end should be u32"))
                        .collect_tuple::<(u32, u32)>()
                        .map(|(start, end)| start..=end)
                        .expect("each range should have a start and end")
                })
                .collect_tuple::<(_, _)>()
                .expect("each line must have a pair of ranges")
        })
        //.filter(|(a, b)| a.contains_or_is_contained(b))
        .count();
    dbg!(redundant);
    let p = include_str!("input.txt").lines().collect::<Vec<&str>>();

    println!("{:?}", p);
    Ok(())
}

struct Rang(Range<u8>);
impl TryFrom<&str> for Rang {
    type Error = color_eyre::Report;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        return Ok(Rang(std::ops::Range { start: 3, end: 5 }));
    }
}
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
