mod board;

fn main() -> color_eyre::Result<()> {
    let mut sudoku_game = board::Sudoku {
        board: [[0; 9]; 9],
        solved_board: [[0; 9]; 9],
    };

    sudoku_game.generator(30);

    let mut allowed = [[false; 9]; 9];
    let n = sudoku_game.board.len();
    for row_index in 0..n {
        for col_index in 0..n {
            if sudoku_game.board[row_index][col_index] != 0 {
                allowed[row_index][col_index] = true;
            }
        }
    }

    let mut game = board::App {
        sudoku_game,
        cursor_x: 0,
        cursor_y: 0,
        should_quit: false,
        generated: allowed,
        game_state: board::GameState::Menu,
        menu_cursor: 0,
        popup_selected: 0,
        missing_vals: 30,
    };

    color_eyre::install()?;
    ratatui::run(|mut terminal| game.run(&mut terminal))?;

    Ok(())
}
