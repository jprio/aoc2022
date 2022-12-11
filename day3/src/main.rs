fn main() -> color_eyre::Result<()> {
    let mut total_score = 0;

    for line in include_str!("input.txt").lines() {
        let (first, second) = line.split_at(line.len() / 2);

        let first_items = first
            .bytes()
            .map(Item::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        let dupe_score = second
            .bytes()
            .map(Item::try_from)
            .find_map(|item| {
                item.ok().and_then(|item| {
                    first_items
                        .iter()
                        // the iterator gives us `&Item`, but we want `Item`, and
                        // it's `Copy`, so we can call `copied()`
                        .copied()
                        // `find` gives us an `&Item`, but we want `Item`, so we
                        // destructure the reference here:
                        //    👇
                        .find(|&first_item| first_item == item)
                })
            })
            .expect("there should be exactly one duplicate")
            .score();
        dbg!(dupe_score);
        total_score += dupe_score;
    }

    dbg!(total_score);
    Ok(())
}
#[derive(Copy, Clone, PartialEq, Eq)]
struct Item(u8);

impl TryFrom<u8> for Item {
    type Error = color_eyre::Report;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'a'..=b'z' | b'A'..=b'Z' => Ok(Item(value)),
            _ => Err(color_eyre::eyre::eyre!(
                "{} is not a valid item",
                value as char
            )),
        }
    }
}
impl std::fmt::Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0 as char)
    }
}
impl Item {
    fn score(self) -> usize {
        match self {
            Item(b'a'..=b'z') => 1 + (self.0 - b'a') as usize,
            Item(b'A'..=b'Z') => 27 + (self.0 - b'A') as usize,
            _ => unreachable!(),
        }
    }
}
