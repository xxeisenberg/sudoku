use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use rand::prelude::*;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Clear, List, ListItem, Paragraph, Widget},
};

pub struct Sudoku {
    pub board: [[u8; 9]; 9],
    pub solved_board: [[u8; 9]; 9],
}

pub struct App {
    pub sudoku_game: Sudoku,
    pub cursor_x: u8,
    pub cursor_y: u8,
    pub generated: [[bool; 9]; 9],
    pub should_quit: bool,
    pub game_state: GameState,
    pub popup_selected: u8,
    pub menu_cursor: u8,
    pub missing_vals: u8,
}

#[derive(PartialEq)]
pub enum GameState {
    Menu,
    Playing,
    Won,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
        if self.missing_vals == 0 {
            self.missing_vals = 30;
        }

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
        match self.game_state {
            GameState::Won => match key_event.code {
                KeyCode::Left | KeyCode::Right => {
                    if self.popup_selected == 0 {
                        self.popup_selected = 1;
                    } else {
                        self.popup_selected = 0;
                    }
                }
                KeyCode::Enter => {
                    if self.popup_selected == 0 {
                        self.reset_game();
                    } else {
                        self.exit();
                    }
                }
                KeyCode::Char('q') => self.exit(),
                _ => {}
            },

            GameState::Playing => match key_event.code {
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
                        self.check_win_condition();
                    }
                }
                KeyCode::Backspace => {
                    if !self.generated[self.cursor_x as usize][self.cursor_y as usize] {
                        self.sudoku_game.board[self.cursor_x as usize][self.cursor_y as usize] = 0
                    }
                }
                _ => {}
            },

            GameState::Menu => match key_event.code {
                KeyCode::Up => {
                    if self.menu_cursor == 0 {
                        self.menu_cursor = 3;
                    } else {
                        self.menu_cursor -= 1;
                    }
                }
                KeyCode::Down => {
                    if self.menu_cursor == 3 {
                        self.menu_cursor = 0;
                    } else {
                        self.menu_cursor += 1;
                    }
                }
                KeyCode::Left => {
                    if self.menu_cursor == 3 && self.missing_vals > 5 {
                        self.missing_vals -= 1;
                    }
                }
                KeyCode::Right => {
                    if self.menu_cursor == 3 && self.missing_vals < 56 {
                        self.missing_vals += 1;
                    }
                }
                KeyCode::Enter => {
                    let target = match self.menu_cursor {
                        0 => 30,
                        1 => 40,
                        2 => 50,
                        _ => self.missing_vals,
                    };
                    self.missing_vals = target;

                    self.sudoku_game.board = [[0; 9]; 9];
                    self.sudoku_game.generator(self.missing_vals);

                    self.generated = [[false; 9]; 9];
                    let n = self.sudoku_game.board.len();
                    for row_index in 0..n {
                        for col_index in 0..n {
                            if self.sudoku_game.board[row_index][col_index] != 0 {
                                self.generated[row_index][col_index] = true;
                            }
                        }
                    }

                    self.cursor_x = 0;
                    self.cursor_y = 0;
                    self.game_state = GameState::Playing;
                }
                KeyCode::Char('q') => self.exit(),
                _ => {}
            },
        }
    }

    fn reset_game(&mut self) {
        self.game_state = GameState::Menu;
        self.popup_selected = 0;
    }

    fn exit(&mut self) {
        self.should_quit = true;
    }

    fn check_win_condition(&mut self) {
        let n = self.sudoku_game.board.len();
        let mut correct = true;
        for i in 0..n {
            for j in 0..n {
                if self.sudoku_game.board[i][j] != self.sudoku_game.solved_board[i][j] {
                    correct = false;
                }
            }
        }
        if correct {
            self.game_state = GameState::Won;
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.game_state == GameState::Menu {
            let title_text = vec![
                Line::from(Span::styled(
                    " ____  _   _ ____   ___  _  ___   _ ",
                    Style::default().fg(Color::Cyan),
                )),
                Line::from(Span::styled(
                    "/ ___|| | | |  _ \\ / _ \\| |/ / | | |",
                    Style::default().fg(Color::Cyan),
                )),
                Line::from(Span::styled(
                    "\\___ \\| | | | | | | | | | ' /| | | |",
                    Style::default().fg(Color::Cyan),
                )),
                Line::from(Span::styled(
                    " ___) | |_| | |_| | |_| | . \\| |_| |",
                    Style::default().fg(Color::Cyan),
                )),
                Line::from(Span::styled(
                    "|____/ \\___/|____/ \\___/|_|\\_\\\\___/ ",
                    Style::default().fg(Color::Cyan),
                )),
                Line::from(Span::styled(
                    "                        ~xeisenberg ",
                    Style::default().fg(Color::DarkGray),
                )),
            ];

            let custom_label = format!("Custom ({} missing)  <-/->", self.missing_vals);
            let raw_items = vec![
                "Easy (30 missing)",
                "Medium (40 missing)",
                "Hard (50 missing)",
                &custom_label,
            ];

            let mut items = Vec::new();
            for (i, text) in raw_items.iter().enumerate() {
                if i == self.menu_cursor as usize {
                    items.push(
                        ListItem::new(*text)
                            .style(Style::default().bg(Color::DarkGray).fg(Color::Yellow)),
                    );
                } else {
                    items.push(ListItem::new(*text).style(Style::default().fg(Color::Gray)));
                }
            }

            let list = List::new(items)
                .block(Block::bordered().title(" SELECT DIFFICULTY "))
                .style(Color::White);

            let menu_area = area.centered(
                ratatui::layout::Constraint::Length(38),
                ratatui::layout::Constraint::Length(14),
            );

            let menu_layout = Layout::default()
                .direction(ratatui::layout::Direction::Vertical)
                .constraints(vec![
                    Constraint::Length(6),
                    Constraint::Length(1),
                    Constraint::Length(7),
                ])
                .split(menu_area);

            Paragraph::new(title_text)
                .centered()
                .render(menu_layout[0], buf);
            list.render(menu_layout[2], buf);

            return;
        }

        let layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(vec![Constraint::Min(0), Constraint::Length(1)])
            .split(area);

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

        let rect = layout[0].centered(
            ratatui::layout::Constraint::Length(31),
            ratatui::layout::Constraint::Length(13),
        );

        Paragraph::new(sudoku).centered().render(rect, buf);
        let controls = "[WASD/Arrows]: Move   [1-9]: Input   [Backspace]: Clear   [Q]: Quit";
        Paragraph::new(Span::styled(controls, Style::default().fg(Color::Gray)))
            .centered()
            .render(layout[1], buf);

        if self.game_state == GameState::Won {
            let pop_rec = layout[0].centered(
                ratatui::layout::Constraint::Length(31),
                ratatui::layout::Constraint::Length(5),
            );

            let restart_style = if self.popup_selected == 0 {
                Style::default().fg(Color::Black).bg(Color::White)
            } else {
                Style::default()
            };

            let quit_style = if self.popup_selected == 1 {
                Style::default().fg(Color::Black).bg(Color::White)
            } else {
                Style::default()
            };

            let bottom_line = Line::from(vec![
                Span::styled(" [ Restart ] ", restart_style),
                Span::from("   "),
                Span::styled(" [ Quit ] ", quit_style),
            ]);

            Clear.render(pop_rec, buf);
            Paragraph::new("YOU WIN!!!\n\nPerfectly Solved.")
                .block(
                    Block::bordered()
                        .title(" CONGRATS! ")
                        .title_bottom(bottom_line.centered()),
                )
                .centered()
                .render(pop_rec, buf);
        }
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

    pub fn generator(&mut self, missing_values: u8) {
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

        self.solved_board = self.board;

        // Removing numbers

        let mut target = missing_values;
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
