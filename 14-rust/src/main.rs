use std::{
    ops::{Add, AddAssign},
    str::FromStr,
};

use anyhow::anyhow;

#[allow(dead_code)]
const EXAMPLE_WIDTH: u32 = 11;
#[allow(dead_code)]
const EXAMPLE_HEIGHT: u32 = 7;

#[allow(dead_code)]
const REAL_WIDTH: u32 = 101;
#[allow(dead_code)]
const REAL_HEIGHT: u32 = 103;

const WIDTH: u32 = REAL_WIDTH;
const HEIGHT: u32 = REAL_HEIGHT;

/// Starts at top left
#[derive(Debug, Clone, Copy)]
pub struct Vector2 {
    pub x: i32,
    pub y: i32,
}

impl FromStr for Vector2 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, s) = s.split_once("=").ok_or(anyhow!("Invalid input"))?;
        let (x, y) = s.split_once(",").ok_or(anyhow!("Invalid input"))?;

        Ok(Self::new(x.parse()?, y.parse()?))
    }
}

impl Add for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Vector2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone)]
pub struct Robot {
    pub pos: Vector2,
    pub vel: Vector2,
}

impl FromStr for Robot {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p, v) = s.split_once(" ").ok_or(anyhow!("Invalid input"))?;

        let p: Vector2 = p.parse()?;
        let v: Vector2 = v.parse()?;

        Ok(Robot { pos: p, vel: v })
    }
}

impl Robot {
    pub fn advance(&mut self) {
        self.pos += self.vel;

        if self.pos.x < 0 {
            self.pos.x += WIDTH as i32;
        }

        if self.pos.x >= WIDTH as i32 {
            self.pos.x -= WIDTH as i32;
        }

        if self.pos.y < 0 {
            self.pos.y += HEIGHT as i32;
        }

        if self.pos.y >= HEIGHT as i32 {
            self.pos.y -= HEIGHT as i32;
        }
    }

    pub fn advance_n(&mut self, n: u32) {
        for _ in 0..n {
            self.advance();
        }
    }
}

pub fn safety_factor(robots: &[Robot]) -> u32 {
    let mut tl = 0u32;
    let mut tr = 0u32;
    let mut bl = 0u32;
    let mut br = 0u32;

    let half_x = (WIDTH + 1) / 2 - 1;
    let half_y = (HEIGHT + 1) / 2 - 1;

    robots.iter().for_each(|robot| match robot.pos {
        Vector2 { x, y } if x as u32 == half_x || y as u32 == half_y => (),
        Vector2 { x, y } if (x as u32) < half_x && (y as u32) > half_y => tl += 1,
        Vector2 { x, y } if (x as u32) > half_x && (y as u32) > half_y => tr += 1,
        Vector2 { x, y } if (x as u32) < half_x && (y as u32) < half_y => bl += 1,
        Vector2 { x, y } if (x as u32) > half_x && (y as u32) < half_y => br += 1,
        _ => (),
    });

    tl * tr * bl * br
}

pub type Map = [[u32; WIDTH as usize]; HEIGHT as usize];

pub fn build_map(robots: &[Robot]) -> Map {
    let mut map = [[0; WIDTH as usize]; HEIGHT as usize];

    robots.iter().for_each(|Robot { pos, .. }| {
        map[pos.y as usize][pos.x as usize] += 1;
    });

    map
}

pub fn build_map_str(robots: &[Robot]) -> String {
    let map = build_map(robots);

    map.into_iter()
        .map(|row| {
            row.into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join("")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn diff_score(map: Map) -> u32 {
    let mut diff = 0;

    map.into_iter().for_each(|row| {
        for x in 0..WIDTH as usize - 1 {
            diff += (row[x] as i32 - row[x + 1] as i32).unsigned_abs();
        }
    });

    diff
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;

    let mut robots1: Vec<_> = input
        .lines()
        .filter_map(|line| line.parse::<Robot>().ok())
        .collect();

    let mut robots2 = robots1.clone();

    {
        robots1.iter_mut().for_each(|robot| robot.advance_n(100));

        let safety = safety_factor(&robots1);
        println!("{safety}");
        drop(robots1);
    }

    for i in 1..100000 {
        robots2.iter_mut().for_each(Robot::advance);

        if diff_score(build_map(&robots2)) < 500 {
            let map = build_map_str(&robots2);
            println!("{map}\n{i}");
            break;
        }
    }

    Ok(())
}
