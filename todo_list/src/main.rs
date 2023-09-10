// use std::{io::{self, prelude::*, BufReader}, fs:: {File}};
// use std::fs::OpenOptions;
// use tui::{
//     backend::CrosstermBackend,
//     widgets::{Block, List, ListItem, Borders},
//     layout::{Layout, Constraint, Direction},
//     Terminal
// };
// use crossterm::{
//     event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
//     execute,
//     terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
// };
// use clap::Parser;
//
// #[derive(Parser, Debug)]
// struct Todos {
//     #[arg(short, long, value_delimiter = ',', num_args = 1..)]
//     tasks: Vec<String>,
// }
//
// impl Todos {
//     fn add(task: String) {
//         write_to_file("todos.txt".to_string(), 1, task);
//     }
// }
//
// fn read_from_file(filepath: String) -> io::Result<Vec<String>> {
//     let file = File::open(filepath)?;
//     let reader = BufReader::new(file);
//
//     let (lines, errors): (Vec<_>, Vec<_>) = reader.lines().into_iter().partition(Result::is_ok);
//
//     let mut lines_result = Vec::new();
//
//     for line in lines
//     {
//         lines_result.push(line.unwrap())
//     }
//
//     for error in errors
//     {
//         eprintln!("{}", error.unwrap());
//     }
//
//     Ok(lines_result)
// }
//
// fn write_to_file(filepath: String, task_index: u32, task: String) {
//     let mut file = OpenOptions::new()
//         .write(true)
//         .append(true)
//         .open(&filepath)
//         .unwrap();
//
//     if let Err(r) = writeln!(file, "{task_index}. [ ] {task}") {
//         eprintln!("Couldn't write to file: {filepath}");
//     }
// }
//
// fn main() -> Result<(), io::Error> {
//     let todos = read_from_file("todos.txt".to_string()).unwrap();
//     //
//     // let todo_args = Todos::parse();
//     //
//     // for task in todo_args.tasks {
//     //     Todos::add(task);
//     // }
//
//     // TODO: WHEN WRITING TODOS TO FILE, PREPEND [ ] OR [X] BEFORE EACH TODO TO INDICATE WHETHER IT'S CHECKED OFF
//     // TODO: AND THEN REPLACE X WITH WHITESPACE AND VICE-VERSA TO TOGGLE
//
//     setup terminal
//     enable_raw_mode()?;
//     let mut stdout = io::stdout();
//     execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
//     let backend = CrosstermBackend::new(stdout);
//     let mut terminal = Terminal::new(backend)?;
//
//     terminal.draw(|f| {
//         let size = f.size();
//         let todos = [ListItem::new("Storm"), ListItem::new("Storm again")];
//         let block = Block::default()
//             .title("Todo List")
//             .borders(Borders::ALL);
//         let todo_list = List::new(todos).block(block);
//         f.render_widget(todo_list, size);
//     })?;
//
//     // restore terminal
//         disable_raw_mode()?;
//         execute!(
//         terminal.backend_mut(),
//         LeaveAlternateScreen,
//         DisableMouseCapture
//     )?;
//         terminal.show_cursor()?;
//
//     Ok(())
// }

use crossterm::{
    cursor::{Hide, Show},
    event::{read, DisableMouseCapture, EnableMouseCapture, Event as CrosstermEvent, KeyCode, KeyEvent},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use std::io::{stdout, Result, Write};
use tui_input::backend::crossterm as backend;
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;

fn main() -> Result<()> {
    enable_raw_mode()?;
    let stdout = stdout();
    let mut stdout = stdout.lock();
    execute!(stdout, Hide, EnterAlternateScreen, EnableMouseCapture)?;

    let mut input: Input = "Hello ".into();
    backend::write(&mut stdout, input.value(), input.cursor(), (0, 0), 15)?;
    stdout.flush()?;

    loop {
        let event: CrosstermEvent = read()?;

        if let CrosstermEvent::Key(KeyEvent { code, .. }) = event {
            match code {
                KeyCode::Esc | KeyCode::Enter => {
                    break;
                }
                _ => {
                    if input.handle_event(&event).is_some() {
                        backend::write(
                            &mut stdout,
                            input.value(),
                            input.cursor(),
                            (0, 0),
                            15,
                        )?;
                        stdout.flush()?;
                    }
                }
            }
        }
    }

    execute!(stdout, Show, LeaveAlternateScreen, DisableMouseCapture)?;
    disable_raw_mode()?;
    println!("{}", input);
    Ok(())
}