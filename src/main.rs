use app::App;
use ratatui::Terminal;
use ratatui::crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, read};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::prelude::CrosstermBackend;
use std::error::Error;
use std::io;
use ui::ui;

mod app;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    //Setup
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // Running
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    //Nazorg
    disable_raw_mode();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor();

    if let Ok(do_print) = res {
        if do_print {
            app.print_json()?;
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<bool, io::Error> {
    loop {
        terminal.draw(|f| ui(f, app))?;
        if let Event::Key(key) = read()? {
            dbg!(key.code)
        }
    }
}
