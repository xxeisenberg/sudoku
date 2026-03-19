use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use rand::prelude::*;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Paragraph, Widget},
};

pub struct Sudoku {
    pub board: [[u8; 9]; 9],
}

pub struct App {
    pub sudoku_game: Sudoku,
    pub cursor_x: u8,
    pub cursor_y: u8,
    pub generated: [[bool; 9]; 9],
    pub should_quit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
        while !self.should_quit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> color_eyre::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('k') => {
                if self.cursor_x > 0 {
                    self.cursor_x -= 1
                }
            }
            KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('j') => {
                if self.cursor_x < (self.sudoku_game.board.len() - 1) as u8 {
                    self.cursor_x += 1
                }
            }
            KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('h') => {
                if self.cursor_y > 0 {
                    self.cursor_y -= 1
                }
            }
            KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('l') => {
                if self.cursor_y < (self.sudoku_game.board.len() - 1) as u8 {
                    self.cursor_y += 1
                }
            }
            KeyCode::Char(c) => {
                if ('1'..='9').contains(&c)
                    && !self.generated[self.cursor_x as usize][self.cursor_y as usize]
                {
                    self.sudoku_game.board[self.cursor_x as usize][self.cursor_y as usize] =
                        c.to_digit(10).unwrap() as u8;
                }
            }
            KeyCode::Backspace => {
                if !self.generated[self.cursor_x as usize][self.cursor_y as usize] {
                    self.sudoku_game.board[self.cursor_x as usize][self.cursor_y as usize] = 0
                }
            }
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.should_quit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut sudoku: Text = Default::default();
        sudoku.push_line(Line::from("+---------+---------+---------+"));
        let n = self.sudoku_game.board.len();
        for row_index in 0..n {
            let mut row = Line::from("|");
            for col_index in 0..n {
                if col_index != 0 {
                    if col_index % 3 == 0 {
                        row.push_span(Span::from("|"));
                    }
                }

                let digit = self.sudoku_game.board[row_index][col_index];
                let ele = format!(" {} ", digit);
                let mut element = Span::raw("");

                if self.generated[row_index][col_index] == true {
                    element = Span::styled(ele.clone(), Style::default().fg(Color::Yellow))
                }

                if self.generated[row_index][col_index] == false
                    && self.sudoku_game.board[row_index][col_index] != 0
                {
                    element = Span::styled(ele.clone(), Style::default().fg(Color::Blue))
                }

                if digit == 0 {
                    element = Span::from(" . ")
                };

                if row_index == self.cursor_x as usize && col_index == self.cursor_y as usize {
                    let style;
                    if self.generated[self.cursor_x as usize][self.cursor_y as usize] {
                        style = Style::default().fg(Color::Yellow).bg(Color::DarkGray);
                    } else {
                        style = Style::default().fg(Color::Blue).bg(Color::DarkGray);
                    }
                    if digit != 0 {
                        element = Span::styled(ele.clone(), style)
                    } else {
                        element = Span::styled(" . ", Style::default().bg(Color::DarkGray))
                    }
                }

                row.push_span(element);
            }
            if row_index != 0 {
                if row_index % 3 == 0 {
                    sudoku.push_line(Line::from("+---------+---------+---------+"));
                }
            }
            row.push_span(Span::from("|"));
            sudoku.push_line(row);
        }
        sudoku.push_line(Line::from("+---------+---------+---------+"));

        let rect = area.centered(
            ratatui::layout::Constraint::Length(31),
            ratatui::layout::Constraint::Length(13),
        );

        Paragraph::new(sudoku).centered().render(rect, buf);
    }
}

impl Sudoku {
    pub fn print(&self) {
        println!("+-------+-------+-------+");
        for row in 0..9 {
            print!("| ");
            for col in 0..9 {
                let val = self.board[row][col];

                if val == 0 {
                    print!(". ");
                } else {
                    print!("{} ", val);
                }

                if (col + 1) % 3 == 0 {
                    print!("| ");
                }
            }
            println!();

            if (row + 1) % 3 == 0 {
                println!("+-------+-------+-------+");
            }
        }
    }

    pub fn is_safe(&self, row: u8, col: u8, number: u8) -> bool {
        let column_check = self
            .board
            .iter()
            .map(|r| r[col as usize])
            .any(|x| x == number);
        let rows_check = self.board[row as usize].iter().any(|x| x == &number);
        let check = !column_check && !rows_check;
        let res: bool = self.threexthree(row, col, number);
        if check && res { true } else { false }
    }

    fn threexthree(&self, row: u8, col: u8, number: u8) -> bool {
        let start_row = ((row / 3) * 3) as usize;
        let start_col = ((col / 3) * 3) as usize;
        let mut check = true;
        for i in 0..3 {
            for j in 0..3 {
                if self.board[start_row + i][start_col + j] == number {
                    check = false;
                }
            }
        }
        check
    }

    pub fn solve(&mut self) -> bool {
        let n = self.board.len();

        for row_index in 0..n {
            for col_index in 0..n {
                if self.board[row_index][col_index] == 0 {
                    for i in 1..10 {
                        if self.is_safe(row_index as u8, col_index as u8, i) {
                            self.board[row_index][col_index] = i;
                            if self.solve() {
                                return true;
                            } else {
                                self.board[row_index][col_index] = 0;
                            }
                        }
                    }
                    return false;
                }
            }
        }
        true
    }

    pub fn count_solutions(&mut self) -> u8 {
        let n = self.board.len();
        let mut number_of_solutions: u8 = 0;

        for row_index in 0..n {
            for col_index in 0..n {
                if self.board[row_index][col_index] == 0 {
                    for i in 1..10 {
                        if self.is_safe(row_index as u8, col_index as u8, i) {
                            self.board[row_index][col_index] = i;

                            number_of_solutions += self.count_solutions();

                            self.board[row_index][col_index] = 0;

                            if number_of_solutions > 1 {
                                return number_of_solutions;
                            }
                        }
                    }
                    return number_of_solutions;
                }
            }
        }

        return 1;
    }

    pub fn generator(&mut self) {
        let mut rng = rand::rng();
        let mut nums: Vec<u8> = (1..10).collect();
        let mut index = 0;

        // Top left subgrid
        nums.shuffle(&mut rng);
        for i in 0..3 {
            for j in 0..3 {
                self.board[i][j] = nums[index];
                index += 1;
            }
        }
        index = 0;

        // Middle subgrid
        nums.shuffle(&mut rng);
        for i in 3..6 {
            for j in 3..6 {
                self.board[i][j] = nums[index];
                index += 1;
            }
        }
        index = 0;

        // Bottom right subgrid
        nums.shuffle(&mut rng);
        for i in 6..9 {
            for j in 6..9 {
                self.board[i][j] = nums[index];
                index += 1;
            }
        }

        self.solve();

        // Removing numbers

        let mut target = 30;
        while target > 0 {
            let row = rng.random_range(0..9);
            let col = rng.random_range(0..9);
            let n = self.board[row as usize][col as usize];
            if n == 0 {
                continue;
            } else {
                self.board[row as usize][col as usize] = 0;
                if self.count_solutions() == 1 {
                    target -= 1;
                } else {
                    self.board[row as usize][col as usize] = n;
                    continue;
                }
            }
        }
    }
}
