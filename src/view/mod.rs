use std::{fmt::Result, io::{self, Stdout}, sync::{Arc, Mutex}};

use crossterm::{event::EnableMouseCapture, execute, terminal::{enable_raw_mode, EnterAlternateScreen}};

use tui::
{
    backend::CrosstermBackend, layout::
    {
        Constraint, Direction, Layout, Rect
    }, style::
    {
        Color, Modifier, Style
    }, terminal::CompletedFrame, text::{Span, Spans}, widgets::
    {
        Block, Borders, List, ListItem, ListState, Paragraph
    }, Terminal
};

use crate::parser::{ServerCommands, ServerDetails};

pub trait ManagerItems<'a> {
    fn sidebar_items(server_details: Vec<ServerDetails>) -> Vec<ListItem<'a>>;
    fn command_items(server_commands: Vec<ServerCommands>) -> Vec<ListItem<'a>>;
}

pub trait RenderComponent<'a> {
    fn dimensions(direction: Direction,constraints: Vec<Constraint>,area: Rect) -> Vec<Rect>;
    fn sidebar_component(list_details: Vec<ListItem<'a>>, selected_index: &'a Option<usize>) -> List<'a>;
    fn main_component(list_commands: Vec<ListItem<'a>>) -> List<'a>;
    fn bottom_component() -> Paragraph<'a>;
    fn info_paragraph_component(input_info: & mut String) -> Paragraph<'a>;
}

pub struct MainView<'a>{
    selected_index: &'a mut usize,
    input_info: &'a mut String,
    focused_block: &'a mut str,

}

impl<'a> MainView<'a> {
    pub fn new(selected_index: &'a mut usize,input_info: &'a mut String, focused_block: &'a mut str) -> Self {
        Self {
            selected_index,
            input_info,
            focused_block,
        }
    }

    pub async fn update_info_paragraph(output_message: Arc<Mutex<String>>,area: tui::layout::Rect) {
        let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout())).unwrap();
        let mut out = output_message.lock().unwrap();

        let info_paragraph = Paragraph::new(out.clone())
        .block(
            Block::default()
                             .title("Saída das informações")
                             .borders(Borders::ALL)
                             .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            ).style(Style::default().fg(Color::White));

        terminal.draw(|f| {
            f.render_widget(info_paragraph, area);
        }).unwrap();
    }
}

impl<'a> ManagerItems<'a> for MainView<'a> {

     fn sidebar_items(server_details: Vec<ServerDetails>) -> Vec<ListItem<'a>> {
        server_details.iter()
                      .map(|item| ListItem::new(item.name.clone()))
                      .collect()
    }

    fn command_items(server_commands: Vec<ServerCommands>) -> Vec<ListItem<'a>> {
         server_commands.iter()
                        .map(|item| ListItem::new(item.name().to_string()))
                        .collect()
     }
}

pub struct RenderizeComponents {
    stdout: Stdout,
    backend: CrosstermBackend<Stdout>,
    terminal: Terminal<CrosstermBackend<Stdout>>
}

impl<'a> RenderComponent<'a> for RenderizeComponents {
    fn dimensions(direction: Direction,constraints: Vec<Constraint>,area: Rect) -> Vec<Rect>{
        Layout::default()
            .direction(direction)
            .constraints(constraints)
            .split(area)
    }

    fn sidebar_component(list_details: Vec<ListItem<'a>>, selected_index: &'a Option<usize>) -> List<'a> {
        List::new(list_details.into_iter()
    .enumerate()
    .map(|(i,item)| {
        if Some(i) == *selected_index {
            item.style(Style::default().fg(Color::Black).bg(Color::White))
        } else {
            item
        }
    }).collect::<Vec<_>>())
    .block(Block::default().title("Menu").borders(Borders::ALL))
    .style(Style::default().fg(Color::Green).add_modifier(Modifier::ITALIC))
    .highlight_style(
        Style::default()
                        .fg(Color::Black)
                        .bg(Color::White)
                        .add_modifier(Modifier::BOLD)
        )
    }

    fn main_component(list_commands: Vec<ListItem<'a>>) -> List<'a> {
        List::new(list_commands)
    .block(
            Block::default()
                 .title("Opções")
                 .borders(Borders::ALL)
                 .style(Style::default().fg(Color::Green).add_modifier(Modifier::ITALIC)),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(
                 Style::default()
                       .fg(Color::Black)
                       .bg(Color::White)
                       .add_modifier(Modifier::BOLD)
            )
    }

    fn bottom_component() -> Paragraph<'a> {
        Paragraph::new(vec![
        Spans::from(vec![
            Span::raw("1 - Para acessar o item, aperte a tecla "),
            Span::styled("Up/Down", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(" e selecione com "),
            Span::styled("Enter", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
        ]),
        Spans::from(vec![
            Span::raw("2 - Altere entre o menu lateral e comandos com "),
            Span::styled("Left/Right", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        ]),
        Spans::from(vec![
             Span::raw("2 - Sai com "),
             Span::styled("Esc",Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        ])
    ])
    .block(Block::default().title("Instruções").borders(Borders::ALL))
    .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
    }

    fn info_paragraph_component(input_info: & mut String) -> Paragraph<'a> {
        Paragraph::new(input_info.clone())
    .block(
           Block::default()
                 .title("Saída das informações")
                 .borders(Borders::ALL)
                 .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        )
        .style(Style::default().fg(Color::White))
    }


}

impl RenderizeComponents {
    pub fn new(stdout: Stdout,backend: CrosstermBackend<Stdout>,terminal: Terminal<CrosstermBackend<Stdout>>)-> Self {
        Self{
            stdout,
            backend,
            terminal
        }
    }

    pub fn mount(&mut self){

        execute!(&self.stdout,EnterAlternateScreen,EnableMouseCapture).unwrap();

        enable_raw_mode().unwrap();

       loop {
           self.terminal.draw(|f| {
           }).unwrap();
       }
    }
}

//pub fn mount() -> Result<CompletedFrame> {
//
//    let mut stdout = io::stdout();
//    execute!(stdout,EnterAlternateScreen,EnableMouseCapture).unwrap();
//    let backend = CrosstermBackend::new(stdout);
//    let mut terminal = Terminal::new(backend).unwrap();
//}

