mod app;
mod event;
mod filesystem;
mod ui;
mod utils;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut app = app::App::new();

    ratatui::run(|terminal| app.run(terminal))?;
    Ok(())
}