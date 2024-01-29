mod app;
mod dfx_commands;
mod dfx_project;
mod parse_replica_dashboard;

use crate::app::state::AppState;
use crate::{app::layout::get_layout, dfx_project::ProjectDatabase};
use anyhow::Result;
use clap::{Parser, Subcommand};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use std::path::PathBuf;
use std::time::Duration;
use std::time::Instant;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CliArgs {
    /// Path to cache of DFX which executed this extension.
    #[clap(long, env = "DFX_CACHE_PATH", global = true)]
    dfx_cache_path: Option<PathBuf>,

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    RegisterDfxProject { project_path: PathBuf },
}

fn main() -> Result<()> {
    let cli_args = CliArgs::parse();
    if cli_args.dfx_cache_path.is_none() {
        println!("Please provide a path to the DFX cache using the --dfx-cache-path argument.");
        return Ok(());
    }
    let mut db_path = cli_args.dfx_cache_path.clone().unwrap();
    db_path.push("extensions");
    db_path.push("top");
    db_path.push("dfx_projects_database.json");
    if !db_path.exists() {
        println!("Creating DFX projects database...");
        ProjectDatabase::init(&db_path)?;
    }

    if let Some(command) = cli_args.command {
        match command {
            Commands::RegisterDfxProject { project_path } => {
                println!("Registering DFX project at {:?}", project_path);
                ProjectDatabase::register_dfx_project(project_path, db_path)?;
                return Ok(());
            }
        }
    }

    let db = ProjectDatabase::load(&db_path)?;
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    // Initialize the app state
    let path_to_dfx = cli_args.dfx_cache_path.unwrap().join("dfx");
    let path_to_dfx = path_to_dfx.to_str().unwrap();
    let mut app_state = AppState::new(path_to_dfx, db);

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Main loop
    let mut last_tick = Instant::now();

    'mainloop: loop {
        // Handle user input
        if event::poll(app_state.refresh_interval)? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.code == KeyCode::Char('q') {
                    break 'mainloop;
                } else {
                    app_state.handle_input(key_event, path_to_dfx);
                }
            }
        }
        terminal.draw(|frame| {
            let network_widget = if app_state.network_selection_menu_active {
                app_state.widget_network_selection_menu()
            } else {
                app_state.widget_network()
            };
            let identity_widget = if app_state.identity_selection_menu_active {
                app_state.widget_identity_selection_menu()
            } else {
                app_state.widget_identity()
            };
            let canisters_list_widget = app_state.widget_canisters_list();
            let canister_info_widget = app_state.widget_canister_info();
            let logs_widget = app_state.widget_logs();
            // let logs_widget = if app_state.logfile_selection_menu_active.is_some() {
            //     app_state.widget_logs_file_selection()
            // } else {
            //     app_state.widget_logs()
            // };

            let (canisters_chunk, network_chunk, canister_info_chunk, identity_chunk, logs_chunk) =
                get_layout(frame.size());

            frame.render_widget(canisters_list_widget, canisters_chunk);
            frame.render_widget(canister_info_widget, canister_info_chunk);
            frame.render_widget(identity_widget, identity_chunk);
            frame.render_widget(network_widget, network_chunk);
            frame.render_widget(logs_widget, logs_chunk);
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        if last_tick.elapsed() >= app_state.refresh_interval {
            // Update the UI as needed
            last_tick = Instant::now();
            app_state.refresh(path_to_dfx);
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        crossterm::cursor::Show
    )?;
    terminal.show_cursor()?;

    Ok(())
}
