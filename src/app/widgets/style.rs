use crate::app::state::{AppState, LocalReplicaState, LogsPane};
use chrono::Local;
use ratatui::{
    prelude::*,
    style::{Color, Style},
    widgets::{
        block::{Position, Title},
        Block, BorderType, Borders,
    },
};

pub enum WidgetStyle {
    Logs,
    CanisterInfo,
    Replica,
    Canisters,
    Identity,
}

pub enum ListItem {
    Header,
    Item,
    Last,
    Empty,
}

impl ListItem {
    pub fn build<'a>(self, key: &'a str, value: &'a str) -> Line<'static> {
        let style_bold = Style::default().add_modifier(Modifier::BOLD);
        let style_bullet = style_bold.fg(Color::LightGreen);
        let style_key = style_bold.fg(Color::White);
        let style_value = Style::default().fg(Color::Yellow);
        Line::from(vec![
            match self {
                Self::Header => Span::styled("".to_string(), style_bullet),
                Self::Item => Span::styled("  ├ ".to_string(), style_bullet),
                Self::Last => Span::styled("  ╰ ".to_string(), style_bullet),
                Self::Empty => Span::styled("".to_string(), Style::default()),
            },
            Span::styled(format!("{key}"), style_key),
            Span::styled(value.to_string(), style_value),
        ])
        .to_owned()
    }
}

impl AppState {
    pub fn style_selected(&self) -> Style {
        Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::LightRed)
    }
    pub fn style_unselected(&self) -> Style {
        Style::default().add_modifier(Modifier::BOLD)
    }

    pub fn apply_style(&self, widget: WidgetStyle) -> Block {
        match widget {
            WidgetStyle::Logs => Block::default()
                .border_style(Style::default().fg(Color::LightCyan))
                .border_type(BorderType::Rounded)
                .title(
                    Title::from(format!("┤ {} ├", Local::now().format("%H:%M:%S")))
                        .alignment(Alignment::Center),
                )
                .title(
                    Title::from(format!(
                        "┤ view {} logs [LEFT]/[RIGHT] ├",
                        // "┤ view {} logs [LEFT]/[RIGHT], {} [f] ├",
                        match self.logs_pane {
                            LogsPane::CanisterLogs => "replica",
                            LogsPane::ReplicaLogs => "canister",
                        },
                        // match self.logs_source {
                        //     LogsSource::Replica => "load logs from file",
                        //     LogsSource::File(Some(_)) => "detach from file",
                        //     LogsSource::File(None) => "detach from file",
                        // }
                    ))
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
                )
                .title(match self.logs_pane {
                    LogsPane::CanisterLogs => "┤ canister logs ├",
                    LogsPane::ReplicaLogs => "┤ replica logs ├",
                })
                .borders(Borders::ALL),
            WidgetStyle::CanisterInfo => Block::default()
                .border_style(Style::default().fg(Color::LightMagenta))
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL)
                .title("┤ canister info ├"),
            WidgetStyle::Identity => Block::default()
                .border_style(Style::default().fg(Color::Blue))
                .border_type(BorderType::Rounded)
                .title("┤ dfx identity ├")
                .title(
                    Title::from(format!("┤ select identity [i] ├",))
                        .alignment(Alignment::Center)
                        .position(Position::Bottom),
                )
                .borders(Borders::ALL),
            WidgetStyle::Canisters => Block::default()
                .border_style(Style::default().fg(Color::LightYellow))
                .border_type(BorderType::Rounded)
                .title("┤ canisters ├")
                .borders(Borders::ALL),
            WidgetStyle::Replica => Block::default()
                .title("┤ networks ├")
                .title(
                    Title::from(format!(
                        "┤ quit [q], {} replica [s], [-] {}ms [+] ├",
                        if self.replica.state == LocalReplicaState::Running {
                            "stop"
                        } else {
                            "start"
                        },
                        self.refresh_interval.as_millis()
                    ))
                    .alignment(Alignment::Right)
                    .position(Position::Top),
                )
                .title(
                    Title::from(format!("┤ select network [n] ├",))
                        .alignment(Alignment::Center)
                        .position(Position::Bottom),
                )
                .border_style(Style::default().fg(Color::Red))
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL),
        }
    }
}
