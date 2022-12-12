use aoc::{
    anyhow::{self, anyhow, bail, Context},
    wrap_main, Challenge,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Noop,
    AddX(i64),
}

struct Vm {
    x: i64,
    x_history: Vec<i64>,
}

#[derive(Debug, Clone, Copy)]
struct ExecuteOptions {
    debug_instructions: bool,
}

impl Vm {
    fn new() -> Self {
        Self {
            x: 1,
            x_history: vec![],
        }
    }

    fn next_cycle(&mut self) {
        self.x_history.push(self.x);
    }

    fn execute(&mut self, program: &[Instruction], options: &ExecuteOptions) {
        for instruction in program {
            match instruction {
                Instruction::Noop => self.next_cycle(),
                Instruction::AddX(x) => {
                    self.next_cycle();
                    self.next_cycle();
                    self.x += *x;
                }
            }
            if options.debug_instructions {
                println!("{instruction:?}");
                println!(
                    " -> X:{} cycles:{} (history: {:?})",
                    self.x,
                    self.x_history.len(),
                    self.x_history
                );
            }
        }
        self.next_cycle();
    }

    fn signal_strength(&self, cycle: usize) -> i64 {
        cycle as i64 * self.x_history[cycle - 1]
    }
}

fn render_image(width: usize, x_history: &[i64]) -> Vec<bool> {
    let mut pixels = vec![];
    for (cycle, &x) in x_history.iter().enumerate() {
        let scanline_x = (cycle % width) as i64;
        pixels.push(scanline_x == x - 1 || scanline_x == x || scanline_x == x + 1);
    }
    pixels
}

fn challenge_main(challenge: Challenge) -> anyhow::Result<()> {
    let mut program = vec![];
    for line in challenge.input.lines() {
        let mut words = line.split_whitespace();
        let opcode = words
            .next()
            .ok_or_else(|| anyhow!("missing opcode: {line}"))?;
        program.push(match opcode {
            "noop" => Instruction::Noop,
            "addx" => {
                let x = words
                    .next()
                    .ok_or_else(|| anyhow!("missing operand for addx: {line}"))?;
                let x = x.parse().context("invalid integer")?;
                Instruction::AddX(x)
            }
            _ => bail!("invalid opcode: '{opcode}'"),
        });
    }

    let mut vm = Vm::new();
    vm.execute(
        &program,
        &ExecuteOptions {
            debug_instructions: challenge.debug_flags.contains("instructions"),
        },
    );

    if challenge.debug_flags.contains("full-history") {
        println!("full history: {:?}", vm.x_history);
    }

    let sum_of_signal_strengths: i64 = (20..=220)
        .step_by(40)
        .map(|cycle| (cycle, vm.signal_strength(cycle)))
        .inspect(|(cycle, signal_strength)| {
            if challenge.debug_flags.contains("signal-strengths") {
                println!("signal strength @ cycle {cycle}: {signal_strength}");
            }
        })
        .map(|(_, signal_strength)| signal_strength)
        .sum();
    println!("part 1: {sum_of_signal_strengths}");

    let width = 40;
    let image = render_image(width, &vm.x_history);
    let height = image.len() / width;
    println!("part 2:");
    for y in 0..height {
        for x in 0..width {
            let index = x + y * width;
            print!("{}", if image[index] { '#' } else { '.' });
        }
        println!();
    }

    Ok(())
}

fn main() {
    wrap_main(challenge_main)
}
