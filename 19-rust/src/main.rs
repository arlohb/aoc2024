use std::collections::HashMap;

use anyhow::anyhow;

#[derive(Default, Debug)]
struct Tree {
    terminates: bool,
    // TODO: Could be a different structure as only 5 possible keys
    // May or may not be faster
    next: HashMap<u8, Tree>,
}

impl Tree {
    pub fn insert(&mut self, s: &[u8]) {
        s.iter()
            .copied()
            .fold(self, |current, c| current.next.entry(c).or_default())
            .terminates = true;
    }

    pub fn contains1(&self, chars: &[u8]) -> bool {
        if chars.is_empty() {
            return true;
        }

        let mut current = self;

        for (i, c) in chars.iter().enumerate() {
            match current.next.get(c) {
                Some(tree) => current = tree,
                None => return false,
            }

            if current.terminates {
                // Split here
                // 1 - Terminate lookup here, start next char at root
                // 2 - Keep looking here, start next char at current

                if self.contains1(&chars.iter().skip(i + 1).copied().collect::<Vec<_>>()) {
                    return true;
                }
            }
        }

        current.terminates
    }

    pub fn contains2(&self, chars: &[u8]) -> u32 {
        if chars.is_empty() {
            return 0;
        }
        // dbg!(chars.len());

        let mut current = self;
        let mut count = 0;

        for (i, c) in chars.iter().enumerate() {
            // dbg!(std::ptr::addr_of!(current));

            match current.next.get(c) {
                Some(tree) => current = tree,
                None => return count,
            }

            if current.terminates {
                // Split here
                // 1 - Terminate lookup here, start next char at root
                // 2 - Keep looking here, start next char at current

                count += self.contains2(&chars.iter().skip(i + 1).copied().collect::<Vec<_>>());
            }
        }

        count + if current.terminates { 1 } else { 0 }
    }
}

fn id(c: char) -> u8 {
    match c {
        'w' => 0,
        'u' => 1,
        'b' => 2,
        'r' => 3,
        'g' => 4,
        _ => 5,
    }
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let (available_str, targets_str) = input.split_once("\n\n").ok_or(anyhow!("Invalid input"))?;

    let available = available_str.split(", ");
    let targets = targets_str.split("\n").filter(|s| !s.is_empty());

    let mut tree = Tree::default();

    available.for_each(|s| tree.insert(&s.chars().map(id).collect::<Vec<_>>()));

    let reachable1 = targets
        .clone()
        .filter(|s| tree.contains1(&s.chars().map(id).collect::<Vec<_>>()));
    dbg!(targets.clone().count());
    let reachable2 = targets.enumerate().map(|(i, s)| {
        dbg!(i);
        tree.contains2(&s.chars().map(id).collect::<Vec<_>>())
    });

    dbg!(reachable1.count());
    // dbg!(reachable2.collect::<Vec<_>>());
    // dbg!(reachable2.sum::<u32>());

    Ok(())
}
