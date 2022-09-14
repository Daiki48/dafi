use tui::{
    backend::Backend,
    layout::{
        Constraint,
        Layout,
        Alignment,
    },
    style::{
        Color,
        Style,
    },
    widgets::{
        Block,
        Borders,
        BorderType,
    },
    Frame,
};

pub fn ui<B: Backend>(f: &mut Frame<B>) {
    let size = f.size();

    let chunk = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(size);

    let br_style = Style::default().bg(Color::Gray).fg(Color::Black);
    let blck_style = Style::default().bg(Color::LightBlue);
    let block = Block::default()
    .title("Block Title")
    .title_alignment(Alignment::Center)
    .borders(Borders::ALL)
    .border_type(BorderType::Double)
    .border_style(br_style)
    .style(blck_style);

    f.render_widget(block, chunk[0]);
}


