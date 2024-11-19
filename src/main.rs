mod parser;

use core::panic;
use std::{io,time::Duration};
use parser::{ConfigYaml, ServerCommands};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction,Layout},
    style::{Color, Modifier, Style},
    text::{Span,Spans},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Terminal

};
use crossterm::{
    event::{self,DisableMouseCapture,EnableMouseCapture,Event,KeyCode}, execute, terminal::{disable_raw_mode,enable_raw_mode,EnterAlternateScreen,LeaveAlternateScreen}
};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout,EnterAlternateScreen,EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut input = String::new();

    let mut selected_index = 0;
    let mut mainblock_selected_index: Option<usize> = None;
    let mut focused_block = "sidebar";

    let path = "config.yaml";

    let servers = ConfigYaml::new(&path);

    let server_items = match &servers {
        Ok(servers) => servers.list_servers().clone(),
        Err(_) => panic!("Erro ao ler arquivo yaml")
    };

    let mut input_info = String::new();

    let mut commands_server: Vec<ServerCommands> = vec![];
    
    loop {
        terminal.draw(|f| {


            let size = f.size();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(80),
                    Constraint::Percentage(20),
                    ])
                .split(size);

            let top_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(30),
                    Constraint::Percentage(70),
                    ])
                .split(chunks[0]);

            let mut sidebar_items = vec![];

            if server_items.len() > 0 {
             sidebar_items = server_items.iter()
                                         .map(|item| ListItem::new(item.name.clone()))
                                         .collect();
            }

//            let mut list_state = ListState::default();
//            list_state.select(Some(selected_index));

            let mut sidebar_state = ListState::default();

            sidebar_state.select(Some(selected_index));

            let sidebar = List::new(sidebar_items.into_iter().enumerate().map(|(i,item)|{
                if i == selected_index {
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
                );

            f.render_stateful_widget(sidebar, top_chunks[0],&mut sidebar_state);
            //f.render_widget(sidebar, top_chunks[0]);


            let mut server_commands = vec![];

            if !commands_server.is_empty() {
                server_commands = commands_server.iter()
                                                  .map(|item| ListItem::new(item.name()))
                                                  .collect();
            }

            let mut mainblock_state = ListState::default();

            if let Some(index) = mainblock_selected_index {
                mainblock_state.select(Some(index));
            }


            let main_block_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(3),
                Constraint::Length(1),
                Constraint::Min(5)
            ]).split(top_chunks[1]);

            let info_paragraph = Paragraph::new(input_info.clone())
            .block(
                    Block::default()
                         .title("Informações do Servidor")
                         .borders(Borders::ALL)
                         .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                ).style(Style::default().fg(Color::White));

            f.render_widget(info_paragraph, main_block_chunks[0]);

            let main_block = List::new(server_commands)
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
                        );


            f.render_stateful_widget(main_block, main_block_chunks[2], &mut mainblock_state);
            //            let main_block = Block::default()
            //                .title("Opções")
            //                .borders(Borders::ALL)
            //                .style(Style::default().fg(Color::Green).add_modifier(Modifier::ITALIC));
            //
            //            let info_paragraph = Paragraph::new(input_info.clone())
            //            .block(main_block)
            //            .style(Style::default().fg(Color::White));
            //
            //
            //
            //            f.render_widget(info_paragraph, top_chunks[1]);

            let bottom_block = Paragraph::new(vec![
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
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

            f.render_widget(bottom_block, chunks[1]);

        })?;

        if crossterm::event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Up => {
                        if focused_block == "sidebar" {
                            if selected_index > 0 {
                                selected_index -= 1;
                            }
                        } else if focused_block == "mainblock" {
                            if let Some(index) = mainblock_selected_index {
                                if index > 0 {
                                    mainblock_selected_index = Some(index -1);
                                }
                            }
                        }

                    },
                    KeyCode::Down => {
                        if focused_block == "sidebar" {
                            if selected_index < server_items.len() - 1 {
                                selected_index += 1
                            }
                        } else if focused_block == "mainblock" {
                            if let Some(index) = mainblock_selected_index {
                                mainblock_selected_index = Some(index + 1);
                            }
                        }
                    },
                    KeyCode::Left => {
                        focused_block = "sidebar";
                    },
                    KeyCode::Right => {
                        if !commands_server.is_empty() {
                            focused_block = "mainblock";
                        }
                    }
                    KeyCode::Enter => {
                        if focused_block == "sidebar" {
                            if let Some(selected_server) = server_items.get(selected_index) {

                                let server_info = match &servers {
                                    Ok(server) => server.get_info_server(&selected_server.name),
                                    Err(erro) => panic!("Erro ao buscar servidor {:?}",erro)

                                };


                                match server_info {
                                    Some((config,connect,commands)) => {
                                        input_info = format!(
                                            "So: {:?}, Memória: {:?}, Disco: {:?}",
                                            config.os(), config.memory(), config.disk()
                                            );
                                        commands_server = commands;
                                        mainblock_selected_index = Some(0);
                                    },
                                    None => {
                                        input_info = "Não foi possivel obter as informações".to_string();
                                    }
                                }

                            }
                        } else if focused_block == "mainblock" {
                            if let Some(index) = mainblock_selected_index {
                                let selected_command = &commands_server[index];

                                println!("Executando comando: {}", selected_command.name());
                            }
                        }

                    },
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;

    execute!(
           terminal.backend_mut(),
           LeaveAlternateScreen,
           DisableMouseCapture
        )?;

    Ok(())
}
