use std::io;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::Alignment,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app_result = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    app_result
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let area = frame.area();
            let message = Text::from(vec![
                Line::from("Hello, Ratatui 👋"),
                Line::from(""),
                Line::from("Appuie sur q pour quitter."),
            ]);

            let paragraph = Paragraph::new(message)
                .block(Block::default().title("noted").borders(Borders::ALL))
                .alignment(Alignment::Center);

            frame.render_widget(paragraph, area);
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}
