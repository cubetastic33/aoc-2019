use crate::intcode_computer::Computer;
/*use crate::util::event::{Config, Event, Events};

use itertools::Itertools;
use std::io;
use std::time::Duration;

//with graphics, for manual play
use termion::screen::AlternateScreen;
use termion::{event::Key, raw::IntoRawMode};
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Paragraph, Text, Widget, Block, Borders};
use tui::style::{Style, Color};
use tui::layout::Alignment;

#[derive(Clone)]
pub struct ArcadeCabinet {
    computer: Computer,
    completed: bool,
    screen: Vec<Vec<char>>,
    score: i64,
}

impl ArcadeCabinet {
    fn update(&mut self) -> bool {
        if self.computer.inputs.len() > 0 {
            self.completed = self.computer.run();
            if self.completed {
                return false;
            }
            for instruction in self.computer.outputs.chunks(3) {
                if instruction[0] == -1 && instruction[1] == 0 {
                    self.score = instruction[2];
                    continue;
                }
                if instruction[0] as usize >= self.screen[0].len() {
                    let width = self.screen[0].len();
                    for row in self.screen.iter_mut() {
                        row.append(&mut vec![' '; instruction[0] as usize - (width - 1)]);
                    }
                }
                let width = self.screen[0].len();
                while instruction[1] as usize >= self.screen.len() {
                    self.screen.push(vec![' '; width]);
                }
                match instruction[2] {
                    1 => self.screen[instruction[1] as usize][instruction[0] as usize] = '█',
                    2 => self.screen[instruction[1] as usize][instruction[0] as usize] = '▒',
                    3 => self.screen[instruction[1] as usize][instruction[0] as usize] = 'T',
                    4 => self.screen[instruction[1] as usize][instruction[0] as usize] = 'O',
                    _ => self.screen[instruction[1] as usize][instruction[0] as usize] = ' ',
                }
            }
            true
        } else {
            false
        }
    }
}*/

// Solver function for part 1
pub fn number_of_block_tiles(intcode: Vec<i64>) -> usize {
    let mut arcade_cabinet = Computer {
        memory: intcode.to_vec(),
        ..Default::default()
    };
    arcade_cabinet.run();
    let mut block_tiles = 0;
    for instruction in arcade_cabinet.outputs.chunks(3) {
        if instruction[2] == 2 {
            block_tiles += 1;
        }
    }
    block_tiles
}

// Solver function for part 2
pub fn highest_score(intcode: Vec<i64>) -> i64 {
    let mut intcode = intcode.to_vec();
    intcode[0] = 2;
    let mut score = 0;
    let mut arcade_cabinet = Computer {
        memory: intcode.clone(),
        ..Default::default()
    };
    arcade_cabinet.inputs.push_back(0);
    let mut ball = (0, 0);
    let mut paddle_x = 0;
    let mut completed = false;
    while !completed {
        completed = arcade_cabinet.run();
        for instruction in arcade_cabinet.outputs.chunks(3) {
            if instruction[0] == -1 {
                score = instruction[2];
                continue;
            }
            if instruction[2] == 3 {
                paddle_x = instruction[0];
            } else if instruction[2] == 4 {
                ball = (instruction[0], instruction[1]);
            }
        }
        arcade_cabinet.inputs.push_back(if ball.0 > paddle_x { 1 } else if ball.0 < paddle_x { -1 } else { 0 });
    }
    score
}

/*
With graphics, for manual play
#[aoc(day13, part2)]
pub fn play(intcode: &[i64]) -> i64 {
    let mut intcode = intcode.to_vec();
    intcode[0] = 2;

    let stdout = io::stdout().into_raw_mode().unwrap();
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.hide_cursor().unwrap();

    // Setup event handlers
    let config = Config {
        tick_rate: Duration::from_millis(75),
        ..Default::default()
    };
    let events = Events::with_config(config);

    // Create app instance
    let mut arcade_cabinet = ArcadeCabinet {
        computer: Computer {
            memory: intcode.clone(),
            ..Default::default()
        },
        completed: false,
        screen: vec![vec![' ']],
        score: 0,
    };

    let mut states = Vec::new();
    let mut fresh = false;
    loop {
        if !arcade_cabinet.completed {
            match events.next().unwrap() {
                Event::Input(input) => match input {
                    Key::Char('q') => {
                        // Quit the program
                        break;
                    }
                    Key::Char('u') => {
                        // Undo the previous move
                        if fresh {
                            states.pop();
                            fresh = false;
                        }
                        if let Some(previous_state) = states.pop() {
                            arcade_cabinet = previous_state;
                        }
                    }
                    Key::Right => arcade_cabinet.computer.inputs.push_back(1),
                    Key::Left => arcade_cabinet.computer.inputs.push_back(-1),
                    _ => arcade_cabinet.computer.inputs.push_back(0),
                }
                Event::Tick => {
                    if arcade_cabinet.update() {
                        fresh = true;
                        states.push(arcade_cabinet.clone());
                    }
                }
            }

            terminal.draw(|mut f| {
                let size = f.size();
                let stuff_to_draw = vec![
                    Text::styled(format!("Score: {}\n\n", arcade_cabinet.score), Style::default().fg(Color::Indexed(226))),
                    Text::raw(arcade_cabinet.screen.iter().map(|x| x.iter().join("")).join("\n")),
                ];
                Paragraph::new(stuff_to_draw.iter())
                    .block(Block::default().title("Arcade Cabinet").borders(Borders::ALL))
                    .alignment(Alignment::Center)
                    .render(&mut f, size);
            }).unwrap();
        } else {
            match events.next().unwrap() {
                Event::Input(input) => match input {
                    Key::Char('q') => {
                        // Quit the program
                        break;
                    }
                    Key::Char('r') => {arcade_cabinet = ArcadeCabinet {
                        computer: Computer {
                            memory: intcode.clone(),
                            ..Default::default()
                        },
                        completed: false,
                        screen: vec![vec![' ']],
                        score: 0,
                    };
                    },
                    _ => arcade_cabinet.computer.inputs.push_back(0),
                }
                Event::Tick => {
                    arcade_cabinet.update();
                },
            }

            terminal.draw(|mut f| {
                let size = f.size();
                let stuff_to_draw = vec![
                    Text::styled("\n\n\nGame Over! Press `r` to play again, or `q` to quit", Style::default().fg(Color::Indexed(197))),
                ];
                Paragraph::new(stuff_to_draw.iter())
                    .block(Block::default().title("Arcade Cabinet").borders(Borders::ALL))
                    .alignment(Alignment::Center)
                    .render(&mut f, size);
            }).unwrap();
        }
    }

    arcade_cabinet.score
}
*/
