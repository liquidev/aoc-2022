use std::{
    ops::Range,
    str::FromStr,
    time::{Duration, Instant},
    vec,
};

use aoc::{
    anyhow::{self, anyhow, bail, Context},
    bitmap::{Bitmap, OutOfBoundsError},
    math::Size,
    owo_colors::{AnsiColors, OwoColorize},
    wrap_main, Challenge,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Point {
    x: i32,
    y: i32,
}

fn point(x: i32, y: i32) -> Point {
    Point { x, y }
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .split_once(',')
            .ok_or_else(|| anyhow!("invalid point. points should be formatted like x,y"))?;
        let x = x.parse().context("invalid coordinate")?;
        let y = y.parse().context("invalid coordinate")?;
        Ok(Self { x, y })
    }
}

#[derive(Debug, Clone)]
struct Path {
    points: Vec<Point>,
}

impl Path {
    fn min_x(&self) -> Option<i32> {
        self.points.iter().map(|point| point.x).min()
    }

    fn min_y(&self) -> Option<i32> {
        self.points.iter().map(|point| point.y).min()
    }

    fn max_x(&self) -> Option<i32> {
        self.points.iter().map(|point| point.x).max()
    }

    fn max_y(&self) -> Option<i32> {
        self.points.iter().map(|point| point.y).max()
    }
}

impl FromStr for Path {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points = vec![];
        for coords in s.split(" -> ") {
            points.push(coords.parse().context("invalid point coordinates")?);
        }
        Ok(Self { points })
    }
}

#[derive(Debug, Default)]
struct PlayArea {
    paths: Vec<Path>,
    sand_source: Point,
    size: Size<u32>,
}

fn compute_play_area(
    mut paths: Vec<Path>,
    sand_source: Point,
    override_min_x: Option<i32>,
    override_max_x: Option<i32>,
) -> PlayArea {
    paths.retain(|path| !path.points.is_empty());
    if paths.is_empty() {
        return PlayArea::default();
    }

    let min_x = override_min_x.unwrap_or_else(|| {
        paths
            .iter()
            .flat_map(|path| path.min_x())
            .min()
            .unwrap_or(0)
            .min(sand_source.x)
    });
    let min_y = paths
        .iter()
        .flat_map(|path| path.min_y())
        .min()
        .unwrap_or(0)
        .min(sand_source.y);
    let max_x = override_max_x.unwrap_or_else(|| {
        paths
            .iter()
            .flat_map(|path| path.max_x())
            .max()
            .unwrap_or(0)
            .max(sand_source.x)
    });
    let max_y = paths
        .iter()
        .flat_map(|path| path.max_y())
        .max()
        .unwrap_or(0)
        .max(sand_source.y);

    for path in &mut paths {
        for point in &mut path.points {
            point.x -= min_x;
            point.y -= min_y;
        }
    }

    PlayArea {
        paths,
        sand_source: point(sand_source.x - min_x, sand_source.y - min_y),
        size: Size {
            width: (max_x - min_x + 1) as u32,
            height: (max_y - min_y + 1) as u32,
        },
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Tile {
    Blank,
    Rock,
    Sand,
}

impl Tile {
    fn color(&self) -> AnsiColors {
        match self {
            Tile::Blank => AnsiColors::Black,
            Tile::Rock => AnsiColors::White,
            Tile::Sand => AnsiColors::Yellow,
        }
    }
}

#[derive(Debug, Default)]
struct SimulationStats {
    out_of_bounds_writes: usize,
    moved_tiles: usize,
}

impl SimulationStats {
    fn move_tile(&mut self, cave: &mut Cave, from: Point, to: Point) {
        match cave.move_tile(from, to) {
            Ok(false) => (),
            Ok(true) => self.moved_tiles += 1,
            Err(OutOfBoundsError) => self.out_of_bounds_writes += 1,
        }
    }
}

struct Cave {
    bitmap: Bitmap<Tile>,
    has_floor: bool,
}

impl Cave {
    fn set(&mut self, point: Point, to: Tile) -> Result<(), OutOfBoundsError> {
        self.bitmap.set((point.x, point.y), to)
    }

    fn get(&self, point: Point) -> Tile {
        if self.has_floor && point.y > self.bitmap.height as i32 {
            Tile::Rock
        } else {
            self.bitmap[(point.x, point.y)]
        }
    }

    fn draw_straight_line(&mut self, from: Point, to: Point, with: Tile) -> anyhow::Result<()> {
        if from.y == to.y {
            let (min, max) = (from.x.min(to.x), from.x.max(to.x));
            for x in min..=max {
                let _ = self.bitmap.set((x, from.y), with);
            }
        } else if from.x == to.x {
            let (min, max) = (from.y.min(to.y), from.y.max(to.y));
            for y in min..=max {
                let _ = self.bitmap.set((from.x, y), with);
            }
        } else {
            bail!("line from {from:?} to {to:?} is not straight")
        }

        Ok(())
    }

    fn from_play_area(play_area: &PlayArea) -> anyhow::Result<Self> {
        let bitmap = Bitmap::new(play_area.size.width, play_area.size.height, Tile::Blank);
        let mut cave = Self {
            bitmap,
            has_floor: false,
        };

        for path in &play_area.paths {
            for pair in path.points.windows(2) {
                let (start, end) = (pair[0], pair[1]);
                cave.draw_straight_line(start, end, Tile::Rock)?;
            }
        }

        Ok(cave)
    }

    fn move_tile(&mut self, from: Point, to: Point) -> Result<bool, OutOfBoundsError> {
        if self.get(to) == Tile::Blank {
            let ok = self.set(to, self.get(from));
            let _ = self.set(from, Tile::Blank);
            ok.map(|_| true)
        } else {
            Ok(false)
        }
    }

    fn simulate(&mut self) -> SimulationStats {
        let mut stats = SimulationStats::default();

        for y in (0..self.bitmap.height).rev() {
            for x in 0..self.bitmap.width {
                let (x, y) = (x as i32, y as i32);
                if self.get(point(x, y)) == Tile::Sand && self.get(point(x, y + 1)) == Tile::Blank {
                    stats.move_tile(self, point(x, y), point(x, y + 1));
                }
            }

            for x in 0..self.bitmap.width {
                let (x, y) = (x as i32, y as i32);
                if self.get(point(x, y)) == Tile::Sand
                    && self.get(point(x, y + 1)) != Tile::Blank
                    && self.get(point(x - 1, y + 1)) == Tile::Blank
                {
                    stats.move_tile(self, point(x, y), point(x - 1, y + 1));
                }
            }

            for x in (0..self.bitmap.width).rev() {
                let (x, y) = (x as i32, y as i32);
                if self.get(point(x, y)) == Tile::Sand
                    && self.get(point(x, y + 1)) != Tile::Blank
                    && self.get(point(x + 1, y + 1)) == Tile::Blank
                {
                    stats.move_tile(self, point(x, y), point(x + 1, y + 1));
                }
            }
        }

        stats
    }

    fn print_to_stdout(&self, x_range: Option<Range<u32>>) {
        let x_range = x_range.unwrap_or(0..self.bitmap.width);
        for y in (0..self.bitmap.height).step_by(2) {
            for x in x_range.clone() {
                let top = self.bitmap[(x as i32, y as i32)];
                let bottom = self.bitmap[(x as i32, (y + 1) as i32)];
                print!("{}", "▄".color(bottom.color()).on_color(top.color()));
            }
            println!("{}", "".default_color().on_default_color());
        }
    }
}

fn do_part(
    challenge: &Challenge,
    paths: Vec<Path>,
    override_min_x: Option<i32>,
    override_max_x: Option<i32>,
    with_floor: bool,
) -> anyhow::Result<usize> {
    let play_area = compute_play_area(
        paths,
        Point { x: 500, y: 0 },
        override_min_x,
        override_max_x,
    );
    dbg!(&play_area);

    let mut cave = Cave::from_play_area(&play_area)?;
    cave.has_floor = with_floor;

    cave.set(play_area.sand_source, Tile::Sand).unwrap();

    let mut units_of_sand = 0;
    let mut delay_f = 0.01;

    let target_ms = Duration::from_secs_f64(1.0 / 15.0);
    let mut last_render = Instant::now();

    let nice = challenge.debug_flags.contains("cave");
    let print_stats = challenge.debug_flags.contains("stats");

    if nice {
        print!("\x1B[1;1H\x1B[J");
        cave.print_to_stdout(None);
    }

    loop {
        let now = Instant::now();
        let stats = cave.simulate();
        let sim_end = Instant::now();

        if Instant::now() - last_render > target_ms {
            if nice {
                print!("\x1B[1;1H");
                cave.print_to_stdout(None);
            }
            if print_stats {
                println!(
                    "{stats:?} sim: {:?}, units: {units_of_sand} delay: {delay_f} ",
                    sim_end - now
                );
                println!();
            }
            last_render = now;
        }

        if stats.out_of_bounds_writes > 0 || cave.get(play_area.sand_source) == Tile::Sand {
            break;
        }
        if stats.moved_tiles == 0 {
            cave.set(play_area.sand_source, Tile::Sand).unwrap();
            units_of_sand += 1;
        }

        if nice {
            let delay = Duration::from_secs_f64(delay_f);
            std::thread::sleep(delay);
            delay_f *= 0.999;
        }
    }

    if nice {
        cave.print_to_stdout(None);
    }

    Ok(units_of_sand)
}

fn challenge_main(challenge: Challenge) -> anyhow::Result<()> {
    let mut paths = vec![];
    for line in challenge.input.lines() {
        paths.push(line.parse::<Path>()?);
    }

    // let units_of_sand = do_part(&challenge, paths.clone(), None, None, false)?;
    // println!("part 1: {units_of_sand}");
    let units_of_sand = do_part(&challenge, paths, Some(0), Some(1000), true)?;
    println!("part 2: {units_of_sand}");

    Ok(())
}

fn main() {
    wrap_main(challenge_main)
}
