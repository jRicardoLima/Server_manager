use std::{io,thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    symbols::block,
    style::{Color,Modifier,Style},
    widgets::{Block, Borders, ListItem,List,ListState, Paragraph, Widget},
    Terminal

};
use crossterm::{
    event::{self,DisableMouseCapture,EnableMouseCapture,Event,KeyCode},
    execute,
    terminal::{disable_raw_mode,enable_raw_mode,EnterAlternateScreen,LeaveAlternateScreen},
};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout,EnterAlternateScreen,EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut input = String::new();

    let mut selected_index = 0;

    
    loop {
        terminal.draw(|f| {
            let size = f.size();

            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(8),
//                    Constraint::Percentage(1),
                    Constraint::Percentage(91),
                ])
                .split(size);

            let sidebar_items = vec![
                ListItem::new("Item 1"),
                ListItem::new("Item 2"),
                ListItem::new("Item 3")
            ];

            let mut list_state = ListState::default();
            list_state.select(Some(selected_index));

            let sidebar = List::new(sidebar_items.into_iter().enumerate().map(|(i,item)|{
                if i == selected_index {
                    item.style(Style::default().fg(Color::Black).bg(Color::White))
                } else {
                    item
                }
            }).collect::<Vec<_>>())
            .block(Block::default().title("Menu").borders(Borders::ALL));

            //            let sidebar = List::new(sidebar_items)
            //                .block(Block::default().title("Servidores").borders(Borders::ALL));

            f.render_widget(sidebar, chunks[0]);

            //            let separator = Block::default().borders(Borders::LEFT);
            //            f.render_widget(separator, chunks[1]);

            let main_block = Block::default()
                .title("Opções")
                .borders(Borders::ALL);

            f.render_widget(main_block, chunks[1]);
        })?;

        if crossterm::event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Up => {
                        if selected_index > 0 {
                            selected_index -= 1;
                        }
                    },
                    KeyCode::Down => {
                        if selected_index < 2 {
                            selected_index += 1;
                        }
                    },
                    _ => {}
                }
            }
        }
//        terminal.draw(|f| {
//            let size = f.size();
//            let block = Block::default()
//                .title("Welcome to Automation Server")
//                .borders(Borders::ALL);
//
//            let paragraph = Paragraph::new(input.as_ref()).block(block);
//             f.render_widget(paragraph, size);
//         })?;

//        if crossterm::event::poll(Duration::from_millis(100))? {
//            if let Event::Key(key) = event::read()? {
//                match key.code {
//                    KeyCode::Char(c) => {
//                        input.push(c);
//                    }
//                    KeyCode::Backspace => {
//                        input.pop();
//                    }
//                    KeyCode::Esc => {
//                        break;
//                    }
//                    _ => {}
//                }
//            }
//        }
    }

    disable_raw_mode()?;

    execute!(
           terminal.backend_mut(),
           LeaveAlternateScreen,
           DisableMouseCapture
        )?;

    Ok(())
}
