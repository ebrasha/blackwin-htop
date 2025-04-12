/*
 **********************************************************************
 * -------------------------------------------------------------------
 * Project Name : BlackWin htop
 * File Name    : ui/mod.rs
 * Author       : Ebrahim Shafiei (EbraSha)
 * Email        : Prof.Shafiei@Gmail.com
 * Created On   : 2024-03-17 12:00:00
 * Description  : UI module for BlackWin htop with cyberpunk theme
 * -------------------------------------------------------------------
 *
 * "Coding is an engaging and beloved hobby for me. I passionately and insatiably pursue knowledge in cybersecurity and programming."
 * – Ebrahim Shafiei
 *
 **********************************************************************
 */

use ratatui::{
    Frame,
    widgets::*,
    style::{Color, Style, Modifier},
    layout::{Layout, Direction, Constraint, Rect},
    prelude::Alignment,
    text::{Line, Span},
};
use crate::{App, event::InputMode};

// Cyberpunk color theme
pub struct CyberpunkTheme {
    pub neon_pink: Color,
    pub electric_blue: Color,
    pub cyber_yellow: Color,
    pub neon_green: Color,
    pub dark_bg: Color,
    pub light_bg: Color,
    pub cpu_low: Color,
    pub cpu_medium: Color,
    pub cpu_high: Color,
}

impl Default for CyberpunkTheme {
    fn default() -> Self {
        Self {
            neon_pink: Color::Rgb(255, 0, 153),
            electric_blue: Color::Rgb(0, 255, 255),
            cyber_yellow: Color::Rgb(255, 255, 0),
            neon_green: Color::Rgb(0, 255, 128),
            dark_bg: Color::Rgb(13, 2, 33),
            light_bg: Color::Rgb(30, 11, 66),
            cpu_low: Color::Rgb(0, 255, 0),      // Green for low usage
            cpu_medium: Color::Rgb(255, 165, 0),  // Orange for medium usage
            cpu_high: Color::Rgb(255, 0, 0),      // Red for high usage
        }
    }
}

fn get_cpu_color(usage: f32, theme: &CyberpunkTheme) -> Color {
    if usage < 50.0 {
        theme.cpu_low
    } else if usage < 80.0 {
        theme.cpu_medium
    } else {
        theme.cpu_high
    }
}

fn draw_cpu_gauges(f: &mut Frame, area: Rect, cpu_cores: &[(String, f32)], theme: &CyberpunkTheme) {
    // Calculate how many rows we need (each row will have 3 CPU cores)
    let cores_per_row = 3;
    let rows = (cpu_cores.len() + cores_per_row - 1) / cores_per_row;
    
    // Create vertical layout for rows
    let row_constraints = vec![Constraint::Length(1); rows];
    let row_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(row_constraints)
        .split(area);

    // Process each row
    for row in 0..rows {
        let start_idx = row * cores_per_row;
        let end_idx = (start_idx + cores_per_row).min(cpu_cores.len());
        let cores_in_this_row = &cpu_cores[start_idx..end_idx];

        // Create horizontal layout for this row
        let mut col_constraints = vec![Constraint::Percentage((100 / cores_per_row) as u16); cores_in_this_row.len()];
        // Add any remaining space to the last column
        if cores_in_this_row.len() < cores_per_row {
            col_constraints.push(Constraint::Min(0));
        }

        let col_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(col_constraints)
            .split(row_layout[row]);

        // Draw each CPU core in this row
        for (i, (name, usage)) in cores_in_this_row.iter().enumerate() {
            let bar_width = 20;
            let filled = ((usage * bar_width as f32) / 100.0) as usize;
            let bar: String = format!(
                "{}{}",
                "█".repeat(filled),
                "░".repeat(bar_width - filled)
            );
            
            let line = Line::from(vec![
                Span::styled(
                    format!("{:<4}", name),
                    Style::default().fg(theme.electric_blue)
                ),
                Span::styled(
                    bar,
                    Style::default().fg(get_cpu_color(*usage, theme))
                ),
                Span::styled(
                    format!("{:>5.1}%", usage),
                    Style::default().fg(theme.neon_green)
                ),
            ]);

            let paragraph = Paragraph::new(line)
                .alignment(Alignment::Left);
            f.render_widget(paragraph, col_layout[i]);
        }
    }
}

pub fn draw(f: &mut Frame, app: &App) {
    let theme = CyberpunkTheme::default();
    
    // Create the layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Length(8),  // CPU cores (increased height for more rows)
            Constraint::Length(3),  // System stats
            Constraint::Min(0),     // Process list
            Constraint::Length(1),  // Status bar
        ])
        .split(f.size());

    // Draw header
    let header = Paragraph::new("BlackWin htop - Advanced Windows Process Monitor ::: Crafted with precision by Ebrahim Shafiei (EbraSha)")
        .style(Style::default()
            .fg(theme.neon_pink)
            .add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center);
    f.render_widget(header, chunks[0]);

    // Draw CPU cores
    let cpu_cores = app.system_info.cpu_cores_usage();
    draw_cpu_gauges(f, chunks[1], &cpu_cores, &theme);

    // Draw system stats
    let (used_mem, total_mem) = app.system_info.memory_usage();
    let mem_percentage = (used_mem as f64 / total_mem as f64 * 100.0) as u64;
    let cpu_usage = app.system_info.cpu_usage();
    let (load1, load5, load15) = app.system_info.load_average();

    let stats = format!(
        "CPU: {:.1}% | Mem: {}/{}MB ({:.1}%) | Load: {:.2} {:.2} {:.2}",
        cpu_usage,
        used_mem / 1024 / 1024,
        total_mem / 1024 / 1024,
        mem_percentage,
        load1, load5, load15
    );

    let stats_widget = Paragraph::new(stats)
        .style(Style::default().fg(theme.electric_blue))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.electric_blue))
            .title("System Stats"));
    f.render_widget(stats_widget, chunks[2]);

    // Draw process list
    let processes = app.process_list.processes();
    let items: Vec<ListItem> = processes
        .iter()
        .enumerate()
        .map(|(i, process)| {
            let content = format!(
                "{:>6} {:>8.1}% {:>8}MB {}",
                process.pid,
                process.cpu_usage,
                process.memory_usage / 1024 / 1024,
                process.name,
            );
            let style = if i == app.process_list.selected_index() {
                Style::default()
                    .fg(theme.cyber_yellow)
                    .add_modifier(Modifier::REVERSED)
            } else {
                Style::default().fg(theme.neon_green)
            };
            ListItem::new(content).style(style)
        })
        .collect();

    let process_list = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.cyber_yellow))
            .title("Processes"))
        .highlight_style(
            Style::default()
                .fg(theme.cyber_yellow)
                .add_modifier(Modifier::REVERSED)
        )
        .highlight_symbol(">> ");
    
    // Calculate scroll offset to keep selected item in view
    let list_height = chunks[3].height as usize - 2; // Subtract 2 for borders
    let selected_index = app.process_list.selected_index();
    let scroll_threshold = list_height / 4;
    let scroll = if selected_index > list_height - scroll_threshold {
        selected_index - (list_height - scroll_threshold)
    } else {
        0
    };

    let list_state = &mut ListState::default();
    list_state.select(Some(selected_index));
    
    f.render_stateful_widget(process_list, chunks[3], list_state);

    // Draw status bar
    let status = match app.input_state.mode {
        InputMode::Normal => {
            format!(
                "Press: q-Quit | F3-Search | F9-Kill | j/k-Move | c-CPU | m-Memory | n-Name | p-PID"
            )
        }
        InputMode::Search => {
            format!("Search: {} (Press Enter to confirm, Esc to cancel)", app.input_state.search_input)
        }
    };

    let status_style = match app.input_state.mode {
        InputMode::Normal => Style::default().fg(theme.neon_green),
        InputMode::Search => Style::default().fg(theme.neon_pink),
    };

    let status_widget = Paragraph::new(status)
        .style(status_style)
        .alignment(Alignment::Left);
    f.render_widget(status_widget, chunks[4]);
} 