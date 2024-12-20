use std::{collections::HashMap, hash::Hash, sync::Mutex};

use anyhow::anyhow;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Map<const N: usize, T> {
    data: [Option<T>; N],
}

impl<const N: usize, T: Default> Map<N, T> {
    pub fn get(&self, n: usize) -> Option<&T> {
        self.data[n].as_ref()
    }

    pub fn get_mut_or_insert_default(&mut self, n: usize) -> &mut T {
        if self.data[n].is_some() {
            self.data[n].as_mut().unwrap()
        } else {
            self.data[n] = Some(T::default());
            self.data[n].as_mut().unwrap()
        }
    }
}

impl<const N: usize, T> Default for Map<N, T> {
    fn default() -> Self {
        Self {
            data: [(); N].map(|_| Default::default()),
        }
    }
}

#[derive(Default, Debug, Clone, Hash, PartialEq, Eq)]
struct Tree {
    terminates: bool,
    next: Map<5, Box<Tree>>,
}

impl Tree {
    pub fn insert(&mut self, s: &[u8]) {
        s.iter()
            .copied()
            .fold(self, |current, c| {
                current.next.get_mut_or_insert_default(c as usize)
            })
            .terminates = true;
    }

    pub fn contains1(&self, chars: &[u8]) -> bool {
        if chars.is_empty() {
            return true;
        }

        let mut current = self;

        for (i, c) in chars.iter().enumerate() {
            match current.next.get(*c as usize) {
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

    pub fn contains2(&self, chars: &[u8]) -> u64 {
        if chars.is_empty() {
            return 0;
        }
        // dbg!(chars.len());

        let mut current = self;
        let mut count = 0;

        for (i, c) in chars.iter().enumerate() {
            // dbg!(std::ptr::addr_of!(current));

            match current.next.get(*c as usize) {
                Some(tree) => current = tree,
                None => return count,
            }

            if current.terminates {
                // Split here
                // 1 - Terminate lookup here, start next char at root
                // 2 - Keep looking here, start next char at current

                // count += self.contains2(&chars.iter().skip(i + 1).copied().collect::<Vec<_>>());
                count += TREE_CONTAINS2_MEM.with(|contains2_mem| {
                    contains2_mem(self, &chars.iter().skip(i + 1).copied().collect::<Vec<_>>())
                });
            }
        }

        count + if current.terminates { 1 } else { 0 }
    }
}

thread_local! {
    pub static TREE_CONTAINS2_MEM: Box<dyn Fn(&Tree, &[u8]) -> u64> = {
        let contains2_mem = memoize(|(this, chars): (Tree, Vec<u8>)|
            Tree::contains2(&this, &chars)
        );
        Box::new(move |tree, chars| { contains2_mem((tree.clone(), chars.to_vec())) })
    };
}

pub fn memoize<I: Hash + Eq + Clone, O: Clone>(f: impl Fn(I) -> O) -> impl Fn(I) -> O {
    let map = Mutex::new(HashMap::<I, O>::new());

    move |input| {
        if let Some(output) = map.lock().unwrap().get(&input) {
            return output.clone();
        }

        let output = f(input.clone());
        map.lock().unwrap().insert(input, output.clone());
        output
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
    dbg!(reachable2.sum::<u64>());

    Ok(())
}
