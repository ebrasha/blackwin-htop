/*
 **********************************************************************
 * -------------------------------------------------------------------
 * Project Name : BlackWin htop
 * File Name    : main.rs
 * Author       : Ebrahim Shafiei (EbraSha)
 * Email        : Prof.Shafiei@Gmail.com
 * Created On   : 2024-03-17 12:00:00
 * Description  : Main entry point for BlackWin htop - A cyberpunk-themed system monitor
 * -------------------------------------------------------------------
 *
 * "Coding is an engaging and beloved hobby for me. I passionately and insatiably pursue knowledge in cybersecurity and programming."
 * – Ebrahim Shafiei
 *
 **********************************************************************
 */

use std::{io, time::Duration};
use crossterm::{
    event::{KeyEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use anyhow::Result;

mod ui;
mod system;
mod process;
mod event;

use crate::{
    event::{EventHandler, InputState},
    process::{ProcessList, SortField},
    system::SystemInfo,
};

pub struct App {
    should_quit: bool,
    system_info: SystemInfo,
    process_list: ProcessList,
    input_state: InputState,
}

impl App {
    fn new() -> Result<Self> {
        Ok(Self {
            should_quit: false,
            system_info: SystemInfo::new(),
            process_list: ProcessList::new(),
            input_state: InputState::default(),
        })
    }

    fn update(&mut self) {
        self.system_info.update();
        self.process_list.update();
    }

    fn handle_input(&mut self, key: KeyEvent) {
        if self.input_state.handle_input(key) {
            if !self.input_state.search_input.is_empty() {
                self.process_list.filter(&self.input_state.search_input);
            }
            return;
        }

        match key.code {
            // Quit
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                self.should_quit = true;
            }

            // Search
            KeyCode::F(3) => {
                self.input_state.enter_search_mode();
            }

            // Navigation
            KeyCode::Char('j') | KeyCode::Down => self.process_list.move_selection(1),
            KeyCode::Char('k') | KeyCode::Up => self.process_list.move_selection(-1),
            KeyCode::PageDown => self.process_list.move_selection(10),
            KeyCode::PageUp => self.process_list.move_selection(-10),
            KeyCode::Home => self.process_list.move_to_start(),
            KeyCode::End => self.process_list.move_to_end(),

            // Sorting
            KeyCode::Char('p') | KeyCode::Char('P') => self.process_list.set_sort_field(SortField::Pid),
            KeyCode::Char('n') | KeyCode::Char('N') => self.process_list.set_sort_field(SortField::Name),
            KeyCode::Char('c') | KeyCode::Char('C') => self.process_list.set_sort_field(SortField::Cpu),
            KeyCode::Char('m') | KeyCode::Char('M') => self.process_list.set_sort_field(SortField::Memory),

            // Kill process
            KeyCode::F(9) => {
                if let Some(pid) = self.process_list.selected_pid() {
                    self.process_list.kill_process(pid);
                }
            }

            // Help
            KeyCode::F(1) => {
                // TODO: Show help screen
            }

            _ => {}
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new()?;
    
    // Create event handler
    let event_handler = EventHandler::new(Duration::from_millis(250));

    // Run app
    let res = run_app(&mut terminal, &mut app, &event_handler).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    event_handler: &EventHandler,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        match event_handler.next()? {
            event::Event::Input(key) => {
                app.handle_input(key);
            }
            event::Event::Tick => {
                app.update();
            }
        }

        if app.should_quit {
            return Ok(());
        }
    }
}
