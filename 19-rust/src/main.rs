use std::collections::HashMap;

use anyhow::anyhow;

#[derive(Default, Debug)]
struct Tree {
    terminates: bool,
    // TODO: Could be a different structure as only 5 possible keys
    // May or may not be faster
    next: HashMap<char, Tree>,
}

impl Tree {
    pub fn insert(&mut self, s: &str) {
        s.chars()
            .fold(self, |current, c| current.next.entry(c).or_default())
            .terminates = true;
    }

    pub fn contains(&self, chars: &[char]) -> bool {
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

                if self.contains(&chars.iter().skip(i + 1).copied().collect::<Vec<_>>()) {
                    return true;
                }
            }
        }

        current.terminates
    }
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let (available_str, targets_str) = input.split_once("\n\n").ok_or(anyhow!("Invalid input"))?;

    let available = available_str.split(", ");
    let targets = targets_str.split("\n").filter(|s| !s.is_empty());

    let mut tree = Tree::default();

    available.for_each(|s| tree.insert(s));

    let reachable = targets.filter(|s| tree.contains(&s.chars().collect::<Vec<_>>()));

    // dbg!(reachable.collect::<Vec<_>>());
    dbg!(reachable.count());

    Ok(())
}
