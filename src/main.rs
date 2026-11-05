use std::process::exit;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use notemanager::TaskManager;
use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, HighlightSpacing, List, ListItem, ListState, Paragraph},
};

enum InputMode {
    Normal,
    Adding,
}

fn main() {
    let mut manager = match TaskManager::new() {
        Ok(m) => m,
        Err(error) => {
            eprintln!("Error: {}", error);
            exit(1);
        }
    };

    let mut terminal = ratatui::init();
    let mut list_state = ListState::default();
    if !manager.tasks.is_empty() {
        list_state.select(Some(0));
    }

    let mut mode = InputMode::Normal;
    let mut new_task_input = String::new();

    loop {
        terminal
            .draw(|frame| {
                let chunks = Layout::vertical([
                    Constraint::Length(3),
                    Constraint::Min(3),
                    Constraint::Length(3),
                ])
                .split(frame.area());

                let help_text = match mode {
                    InputMode::Normal => Line::from(vec![
                        Span::styled("[q]", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                        Span::raw(" Quit  "),
                        Span::styled("[a]", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                        Span::raw(" Add  "),
                        Span::styled("[Space]", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                        Span::raw(" Toggle Done  "),
                        Span::styled("[d]", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                        Span::raw(" Delete  "),
                        Span::styled("[↑/↓]", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
                        Span::raw(" Navigate"),
                    ]),
                    InputMode::Adding => Line::from(vec![
                        Span::styled("[Enter]", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                        Span::raw(" Confirm  "),
                        Span::styled("[Esc]", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                        Span::raw(" Cancel"),
                    ]),
                };
                let header = Paragraph::new(help_text)
                    .block(Block::default().borders(Borders::ALL).title(" NoteManager "));
                frame.render_widget(header, chunks[0]);

                let items: Vec<ListItem> = manager
                    .tasks
                    .iter()
                    .map(|task| {
                        let (icon, style) = if task.completed {
                            (
                                "[✓] ",
                                Style::default()
                                    .fg(Color::Green)
                                    .add_modifier(Modifier::CROSSED_OUT),
                            )
                        } else {
                            ("[ ] ", Style::default().fg(Color::White))
                        };
                        let line = Line::from(vec![
                            Span::styled(format!("{:<4}", task.id), Style::default().fg(Color::DarkGray)),
                            Span::styled(icon, if task.completed { Style::default().fg(Color::Green) } else { Style::default().fg(Color::Yellow) }),
                            Span::styled(&task.title, style),
                        ]);
                        ListItem::new(line)
                    })
                    .collect();

                let list = List::new(items)
                    .block(Block::default().borders(Borders::ALL).title(" Tasks "))
                    .highlight_style(
                        Style::default()
                            .bg(Color::DarkGray)
                            .add_modifier(Modifier::BOLD),
                    )
                    .highlight_symbol(">> ")
                    .highlight_spacing(HighlightSpacing::Always);

                frame.render_stateful_widget(list, chunks[1], &mut list_state);

                let input_block = match mode {
                    InputMode::Normal => Paragraph::new("Press 'a' to add a new task...")
                        .style(Style::default().fg(Color::DarkGray))
                        .block(Block::default().borders(Borders::ALL).title(" New Task ")),
                    InputMode::Adding => Paragraph::new(new_task_input.as_str())
                        .style(Style::default().fg(Color::Yellow))
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .border_style(Style::default().fg(Color::Green))
                                .title(" Adding Task (Type title and press Enter) "),
                        ),
                };
                frame.render_widget(input_block, chunks[2]);
            })
            .unwrap();

        if let Event::Key(key) = event::read().unwrap()
            && key.kind == KeyEventKind::Press
        {
            match mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('a') => {
                            mode = InputMode::Adding;
                            new_task_input.clear();
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            if !manager.tasks.is_empty() {
                                let i = match list_state.selected() {
                                    Some(i) => {
                                        if i + 1 >= manager.tasks.len() {
                                            0
                                        } else {
                                            i + 1
                                        }
                                    }
                                    None => 0,
                                };
                                list_state.select(Some(i));
                            }
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            if !manager.tasks.is_empty() {
                                let i = match list_state.selected() {
                                    Some(i) => {
                                        if i == 0 {
                                            manager.tasks.len() - 1
                                        } else {
                                            i - 1
                                        }
                                    }
                                    None => 0,
                                };
                                list_state.select(Some(i));
                            }
                        }
                        KeyCode::Char(' ') => {
                            if let Some(i) = list_state.selected()
                                && let Some(task) = manager.tasks.get(i)
                            {
                                let id = task.id;
                                manager.toggle(id);
                            }
                        }
                        KeyCode::Char('d') | KeyCode::Delete | KeyCode::Backspace => {
                            if let Some(i) = list_state.selected()
                                && let Some(task) = manager.tasks.get(i)
                            {
                                let id = task.id;
                                manager.delete(vec![id]);
                                if manager.tasks.is_empty() {
                                    list_state.select(None);
                                } else if i >= manager.tasks.len() {
                                    list_state.select(Some(manager.tasks.len() - 1));
                                }
                            }
                        }
                        _ => {}
                    },
                    InputMode::Adding => match key.code {
                        KeyCode::Enter => {
                            let title = new_task_input.trim();
                            if !title.is_empty() {
                                manager.add(title.to_string());
                                list_state.select(Some(manager.tasks.len() - 1));
                            }
                            new_task_input.clear();
                            mode = InputMode::Normal;
                        }
                        KeyCode::Char(c) => {
                            new_task_input.push(c);
                        }
                        KeyCode::Backspace => {
                            new_task_input.pop();
                        }
                        KeyCode::Esc => {
                            new_task_input.clear();
                            mode = InputMode::Normal;
                        }
                        _ => {}
                    },
                }
            }
        }

    ratatui::restore();
}
