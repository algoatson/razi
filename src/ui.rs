use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph, List, ListItem},
    style::{Color, Style, Modifier},
};

use crate::app::App;
use crate::filesystem;

pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(frame.area());

    let middle_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(13),
            Constraint::Percentage(50),
            Constraint::Length(37),
        ])
        .split(chunks[1]);

    // top panel
    let path_name = Paragraph::new(
        crate::utils::path::display_path(&app.current_dir)
    );

    // middle panel items
    let items: Vec<ListItem> = app.current_entries.iter().enumerate()
        .map(|(idx, entry)| {
            let item = ListItem::new(entry.name.clone());

            if idx == app.selected {
                item.style(Style::default().bg(Color::Blue).fg(Color::White).add_modifier(Modifier::BOLD))
            } else {
                item
            }
        })
        .collect();

    let file_view = List::new(items)
        .block(Block::default().borders(Borders::RIGHT));

    // left panel items
    let subitems: Vec<ListItem> = app.parent_entries.iter()
        .map(|entry| ListItem::new(entry.name.clone()))
        .collect(); 

    let dir_view = List::new(subitems)
        .block(Block::default().borders(Borders::RIGHT));

    // render the widgets
    frame.render_widget(path_name, chunks[0]);
    frame.render_widget(dir_view, middle_chunks[0]);
    frame.render_widget(file_view, middle_chunks[1]);



    // let main_chunks = Layout::default()
    //     .direction(Direction::Horizontal)
    //     .constraints([
    //         Constraint::Percentage(70),
    //         Constraint::Percentage(30),
    //     ])
    //     .split(chunks[1]);

    // let system_chunks = Layout::default()
    //     .direction(Direction::Horizontal)
    //     .constraints([
    //         Constraint::Percentage(50),
    //         Constraint::Percentage(50),
    //     ])
    //     .split(main_chunks[0]);

    // let security_chunks = Layout::default()
    //     .direction(Direction::Vertical)
    //     .constraints([
    //         Constraint::Percentage(50),
    //         Constraint::Percentage(50),
    //     ])
    //     .split(main_chunks[1]);

    // let header = Paragraph::new("")
    //         .block(Block::default().title(" pwntop ").borders(Borders::ALL));

    // let system = Paragraph::new("")
    //         .block(Block::default().title(" System ").borders(Borders::ALL));

    // let s1 = Paragraph::new("")
    //     .block(Block::default().title(" s1 ").borders(Borders::ALL));

    // let s2 = Paragraph::new("")
    //     .block(Block::default().title(" s2 ").borders(Borders::ALL));

    // let security = Paragraph::new("")
    //         .block(Block::default().title(" Security ").borders(Borders::ALL));

    // let sec1 = Paragraph::new("")
    //     .block(Block::default().title(" sec1 ").borders(Borders::ALL));

    // let sec2 = Paragraph::new("")
    //     .block(Block::default().title(" sec2 ").borders(Borders::ALL));

    // let footer = Paragraph::new("")
    //         .block(Block::default().title(" q Quit ").borders(Borders::ALL));


    // frame.render_widget(header, chunks[0]);
    // // frame.render_widget(system, main_chunks[0]);
    // frame.render_widget(s1, system_chunks[0]);
    // frame.render_widget(s2, system_chunks[1]);
    // frame.render_widget(sec1, security_chunks[0]);
    // frame.render_widget(sec2, security_chunks[1]);
    // frame.render_widget(footer, chunks[2]);
}