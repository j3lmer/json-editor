use crate::App;
use crate::app::{CurrentScreen, CurrentlyEditing};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap};

pub fn ui(frame: &mut Frame, app: &App) {
    // create the chunks
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    // render the title
    frame.render_widget(get_title(), chunks[0]);

    // render the list
    frame.render_widget(get_list(app), chunks[1]);

    // create navigation
    let current_navigation_text = get_nav_text(app);

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    //  current keys hint
    let current_keys_hint = get_current_keys_hint(app);
    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    frame.render_widget(mode_footer, footer_chunks[0]);
    frame.render_widget(key_notes_footer, footer_chunks[1]);

    if let Some(editing) = &app.currently_editing {
        let popup_block = Block::default()
            .title("Enter a new key-value pair")
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::DarkGray));

        let area = centered_rect(60, 25, frame.area());

        frame.render_widget(popup_block, area);

        let popup_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let mut key_block = Block::default()
            .title("Key")
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::DarkGray));

        let mut value_block = Block::default()
            .title("Value")
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::DarkGray));

        let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);

        match editing {
            CurrentlyEditing::Key => key_block = key_block.style(active_style),
            CurrentlyEditing::Value => value_block = value_block.style(active_style),
        }

        let key_text = Paragraph::new(app.key_input.clone()).block(key_block);
        frame.render_widget(key_text, popup_chunks[0]);

        let value_text = Paragraph::new(app.value_input.clone()).block(value_block);
        frame.render_widget(value_text, popup_chunks[1]);
    }

    // handle the exit screen
    if let CurrentScreen::Exiting = app.current_screen {
        frame.render_widget(Clear, frame.area()); //this clears the entire screen and anything already drawn
        let popup_block = Block::default()
            .title("Y/N")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let exit_text = Text::styled(
            "Would you like to output the buffer as json? (y/n)",
            Style::default().fg(Color::Red),
        );
        // the `trim: false` will stop the text from being cut off when over the edge of the block
        let exit_paragraph = Paragraph::new(exit_text)
            .block(popup_block)
            .wrap(Wrap { trim: false });

        let area = centered_rect(60, 25, frame.area());
        frame.render_widget(exit_paragraph, area);
    }
}

fn get_title<'a>() -> Paragraph<'a> {
    // create the title
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "JSON Editor",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    title
}

fn get_list_items<'a>(app: &'a App) -> Vec<ListItem<'a>> {
    app.pairs
        .iter()
        .map(|(key, value)| get_list_item(key, value))
        .collect()
}

fn get_list_item<'a>(key: &'a str, value: &'a str) -> ListItem<'a> {
    ListItem::new(Line::from(Span::styled(
        format!("{: <25} : {}", key, value),
        Style::default().fg(Color::Yellow),
    )))
}

fn get_list(app: &App) -> List {
    let list_items = get_list_items(app);
    let list = List::new(list_items);

    list
}

fn get_nav_text(app: &App) -> Vec<Span> {
    vec![
        // The first half of the text
        match app.current_screen {
            CurrentScreen::Main => Span::styled("Normal Mode", Style::default().fg(Color::Green)),
            CurrentScreen::Editing => {
                Span::styled("Editing Mode", Style::default().fg(Color::Yellow))
            }
            CurrentScreen::Exiting => Span::styled("Exiting", Style::default().fg(Color::LightRed)),
        }
        .to_owned(),
        // A white divider bar to separate the two sections
        Span::styled(" | ", Style::default().fg(Color::White)),
        // The final section of the text, with hints on what the user is editing
        {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::Key => {
                        Span::styled("Editing Json Key", Style::default().fg(Color::Green))
                    }
                    CurrentlyEditing::Value => {
                        Span::styled("Editing Json Value", Style::default().fg(Color::LightGreen))
                    }
                }
            } else {
                Span::styled("Not Editing Anything", Style::default().fg(Color::DarkGray))
            }
        },
    ]
}

fn get_current_keys_hint(app: &App) -> Span {
    match app.current_screen {
        CurrentScreen::Main => Span::styled(
            "(q) to quit / (e) to make new pair",
            Style::default().fg(Color::Red),
        ),
        CurrentScreen::Editing => Span::styled(
            "(ESC) to cancel/(Tab) to switch boxes/enter to complete",
            Style::default().fg(Color::Red),
        ),
        CurrentScreen::Exiting => Span::styled(
            "(q) to quit / (e) to make new pair",
            Style::default().fg(Color::Red),
        ),
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
