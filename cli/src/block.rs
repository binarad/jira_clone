//! # [Ratatui] `Block` example
//!
//! The latest version of this example is available in the [widget examples] folder in the
//! repository.
//!
//! Please note that the examples are designed to be run against the `main` branch of the Github
//! repository. This means that you may not be able to compile with the latest release version on
//! crates.io, or the one that you have installed locally.
//!
//! See the [examples readme] for more information on finding examples that match the version of the
//! library you are using.
//!
//! [Ratatui]: https://github.com/ratatui/ratatui
//! [widget examples]: https://github.com/ratatui/ratatui/blob/main/ratatui-widgets/examples
//! [examples readme]: https://github.com/ratatui/ratatui/blob/main/examples/README.md

use color_eyre::Result;
use crossterm::event;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType};
use ratatui::Frame;

fn main() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| loop {
        terminal.draw(render)?;
        if event::read()?.is_key_press() {
            break Ok(());
        }
    })
}

/// Render the UI with various blocks.
fn render(frame: &mut Frame) {
    let vertical = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
    let horizontal = Layout::horizontal([Constraint::Percentage(33); 3]).spacing(1);
    let [top, main] = frame.area().layout(&vertical);
    let [left, middle, right] = main.layout(&horizontal);

    let title = Line::from_iter([
        Span::from("Block Widget").bold(),
        Span::from(" (Press 'q' to quit)"),
    ]);
    frame.render_widget(title.centered(), top);

    render_bordered_block(frame, main);
}

/// Render a block with borders.
pub fn render_bordered_block(frame: &mut Frame, area: Rect) {
    let block = Block::bordered().title("Bordered block");
    frame.render_widget(block, area);
}
