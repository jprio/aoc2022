use itertools::Itertools;

fn main() -> color_eyre::Result<()> {
    let redundant = include_str!("input.txt")
        .lines()
        .map(|line| line.split(',').collect_tuple::<(&str, &str)>());
    println!("{:?}", redundant);
    Ok(())
}
