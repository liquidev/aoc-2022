use std::str::FromStr;

use aoc::{
    anyhow::{self, anyhow},
    astar::AStar,
    bitmap::{Bitmap, BitmapParser},
    wrap_main, Challenge,
};

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Elevation(u8);

impl Elevation {
    fn can_visit_from(self, other: Self) -> bool {
        self.0 <= other.0 + 1
    }
}

#[derive(Default)]
struct Parser {
    start: Option<(i32, i32)>,
    goal: Option<(i32, i32)>,
}

impl BitmapParser for Parser {
    type Element = Elevation;

    fn parse_element(&mut self, (x, y): (u32, u32), c: char) -> Option<Self::Element> {
        let c = match c {
            'S' => {
                self.start = Some((x as i32, y as i32));
                'a'
            }
            'E' => {
                self.goal = Some((x as i32, y as i32));
                'z'
            }
            _ => c,
        };
        Some(Elevation(c as u8 - b'a'))
    }
}

struct Hills {
    start: (i32, i32),
    goal: (i32, i32),
    bitmap: Bitmap<Elevation>,
}

impl FromStr for Hills {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (bitmap, parser) = Bitmap::parse(Parser::default(), s)?;
        Ok(Self {
            start: parser
                .start
                .ok_or_else(|| anyhow!("heightmap is missing start point"))?,
            goal: parser
                .goal
                .ok_or_else(|| anyhow!("heightmap is missing goal point"))?,
            bitmap,
        })
    }
}

fn run_a_star(hills: &Hills, start: (i32, i32)) -> Option<Vec<(i32, i32)>> {
    AStar {
        start,
        goal: hills.goal,
        heuristic: &|(x, y)| {
            let (goal_x, goal_y) = hills.goal;
            let dx = goal_x - x;
            let dy = goal_y - y;
            ((dx * dx + dy * dy) as f32).sqrt()
        },
        visit_neighbors: &|&(x, y), visit| {
            let here = hills.bitmap[(x, y)];
            let mut try_visit = |(dx, dy)| {
                let position = (x + dx, y + dy);
                if hills.bitmap.is_in_bounds(position)
                    && hills.bitmap[position].can_visit_from(here)
                {
                    visit(&position, 1.0);
                }
            };
            try_visit((-1, 0));
            try_visit((1, 0));
            try_visit((0, -1));
            try_visit((0, 1));
        },
    }
    .find_path()
}

fn challenge_main(challenge: Challenge) -> anyhow::Result<()> {
    let hills = challenge.input.parse::<Hills>()?;

    let part_1 = run_a_star(&hills, hills.start);
    if let Some(path) = part_1 {
        if challenge.debug_flags.contains("path") {
            println!("{path:?}");
        }
        println!("part 1: {}", path.len());
    }

    let all_possible_paths = hills
        .bitmap
        .positions()
        .filter(|&(x, y)| hills.bitmap[(x, y)] == Elevation(0))
        .filter_map(|(x, y)| run_a_star(&hills, (x, y)))
        .collect::<Vec<_>>();
    if challenge.debug_flags.contains("part2") {
        println!("{} possible paths found", all_possible_paths.len());
    }
    let part_2 = all_possible_paths
        .into_iter()
        .map(|path| path.len())
        .min()
        .ok_or_else(|| anyhow!("no optimal path found"))?;
    println!("part 2: {part_2}");

    Ok(())
}

fn main() {
    wrap_main(challenge_main)
}
