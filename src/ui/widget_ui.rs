use tui::{
    backend::Backend,
    layout::{
        Constraint,
        Layout,
        Alignment,
        Direction,
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
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(30),
                Constraint::Percentage(70),
            ]
            .as_ref()
        )
        .split(size);

    let border_style = Style::default().bg(Color::Magenta).fg(Color::Black);
    let block_style = Style::default().bg(Color::LightBlue);

    let left_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(40),
                Constraint::Percentage(40),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(chunk[0]);

    let status_block = Block::default()
        .title("Status")
        .title_alignment(Alignment::Left)
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .border_style(border_style)
        .style(block_style);
    f.render_widget(status_block, left_chunk[0]);

    let file_top_block = Block::default()
        .title("File")
        .title_alignment(Alignment::Left)
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .border_style(border_style)
        .style(block_style);
    f.render_widget(file_top_block, left_chunk[1]);

    let file_bottom_block = Block::default()
        .title("File")
        .title_alignment(Alignment::Left)
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .border_style(border_style)
        .style(block_style);
    f.render_widget(file_bottom_block, left_chunk[2]);

    let mode_block = Block::default()
        .title("Mode")
        .title_alignment(Alignment::Left)
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .border_style(border_style)
        .style(block_style);
    f.render_widget(mode_block, left_chunk[3]);


    let right_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(90),
            ]
            .as_ref(),
        )
        .split(chunk[1]);

    let path_block = Block::default()
        .title("Path")
        .title_alignment(Alignment::Left)
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .border_style(border_style)
        .style(block_style);
    f.render_widget(path_block, right_chunk[0]);

    let preview_block = Block::default()
        .title("Preview")
        .title_alignment(Alignment::Left)
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .border_style(border_style)
        .style(block_style);
    f.render_widget(preview_block, right_chunk[1]);

}


