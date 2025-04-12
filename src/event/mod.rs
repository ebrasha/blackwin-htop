/*
 **********************************************************************
 * -------------------------------------------------------------------
 * Project Name : BlackWin htop
 * File Name    : event/mod.rs
 * Author       : Ebrahim Shafiei (EbraSha)
 * Email        : Prof.Shafiei@Gmail.com
 * Created On   : 2024-03-17 12:00:00
 * Description  : Event handling module for BlackWin htop
 * -------------------------------------------------------------------
 *
 * "Coding is an engaging and beloved hobby for me. I passionately and insatiably pursue knowledge in cybersecurity and programming."
 * – Ebrahim Shafiei
 *
 **********************************************************************
 */

use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};
use anyhow::Result;
use crossterm::event::{self, KeyEvent};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputMode {
    Normal,
    Search,
}

pub struct InputState {
    pub mode: InputMode,
    pub search_input: String,
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            mode: InputMode::Normal,
            search_input: String::new(),
        }
    }
}

impl InputState {
    pub fn enter_search_mode(&mut self) {
        self.mode = InputMode::Search;
        self.search_input.clear();
    }

    pub fn handle_input(&mut self, key: KeyEvent) -> bool {
        match self.mode {
            InputMode::Normal => false,
            InputMode::Search => {
                match key.code {
                    event::KeyCode::Esc => {
                        self.mode = InputMode::Normal;
                        self.search_input.clear();
                        true
                    }
                    event::KeyCode::Enter => {
                        self.mode = InputMode::Normal;
                        true
                    }
                    event::KeyCode::Backspace => {
                        self.search_input.pop();
                        true
                    }
                    event::KeyCode::Char(c) => {
                        self.search_input.push(c);
                        true
                    }
                    _ => false,
                }
            }
        }
    }
}

pub enum Event {
    Input(KeyEvent),
    Tick,
}

pub struct EventHandler {
    rx: mpsc::Receiver<Event>,
    _tx: mpsc::Sender<Event>,
    _input_handle: thread::JoinHandle<()>,
    _tick_handle: thread::JoinHandle<()>,
}

impl EventHandler {
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = mpsc::channel();
        let input_tx = tx.clone();

        let input_handle = thread::spawn(move || {
            let mut last_input = Instant::now();
            loop {
                if event::poll(Duration::from_millis(50)).expect("failed to poll events") {
                    if let Ok(key) = event::read() {
                        if let event::Event::Key(key) = key {
                            let now = Instant::now();
                            // Increase debounce time to 150ms
                            if now.duration_since(last_input) >= Duration::from_millis(150) {
                                input_tx.send(Event::Input(key)).expect("failed to send input event");
                                last_input = now;
                            }
                        }
                    }
                }
            }
        });

        let tick_handle = {
            let tx = tx.clone();
            thread::spawn(move || loop {
                tx.send(Event::Tick).expect("failed to send tick event");
                thread::sleep(tick_rate);
            })
        };

        Self {
            rx,
            _tx: tx,
            _input_handle: input_handle,
            _tick_handle: tick_handle,
        }
    }

    pub fn next(&self) -> Result<Event> {
        Ok(self.rx.recv()?)
    }
} 