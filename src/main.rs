use git2::Repository;
use git2::StatusOptions;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans, Text};
use tui::widgets::List;
use tui::widgets::ListItem;
use tui::widgets::ListState;
use tui::widgets::{Block, Borders, Paragraph, Widget};
use tui::Terminal;

use std::error::Error;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn Error>> {
    let repo = Repository::open(".").unwrap();
    let mut opts = StatusOptions::new();
    opts.include_untracked(true);
    opts.include_ignored(true);
    let status = repo.statuses(Some(&mut opts)).unwrap();
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    stdout.flush().unwrap();
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut files = vec![];
    for entry in status.iter() {
        // let status = entry.status();
        if let Some(path) = entry.path() {
            files.push(ListItem::new(path.to_string()));
        }
    }

    let mut list_status = ListState::default();
    list_status.select(Some(0));
    terminal.clear().unwrap();

    terminal.draw(|f| {
        let size = f.size();
        let list = List::new(files.clone())
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("> ");
        f.render_stateful_widget(list, size, &mut list_status);
    });

    // 終了するにはqギーを入力する
    let stdin = io::stdin();
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => {
                break;
            }
            Key::Char('k') => {
                let i = match list_status.selected() {
                    Some(i) if i > 0 => i - 1,
                    _ => 0,
                };
                list_status.select(Some(i));
            }
            Key::Char('j') => {
                let i = match list_status.selected() {
                    Some(i) if i < files.len() - 1 => i + 1,
                    Some(i) => i,
                    None => 0,
                };
                list_status.select(Some(i));
            }
            _ => {}
        }
        terminal.draw(|f| {
            let size = f.size();
            let list = List::new(files.clone())
                .block(Block::default().borders(Borders::ALL))
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol("> ");

            f.render_stateful_widget(list, size, &mut list_status);
        })?;
    }
    // ターミナルに追跡対象のファイルを表示する
    Ok(())
}
