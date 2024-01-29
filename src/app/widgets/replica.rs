use crate::app::state::{AppState, LocalReplicaState, LogsPane};
use crate::app::widgets::style::ListItem::{Header, Item, Last};
use ansi_to_tui::IntoText;
use ratatui::{prelude::*, text::Span, widgets::*};

use super::style::WidgetStyle;

impl AppState {
    pub fn widget_network(&self) -> Paragraph {
        let mut lines = vec![];
        lines.push(Header.build("Local replica: ", &self.replica.state.to_string()));
        lines.push(Header.build("Network: ", &self.selected_network));
        if (self.replica.state == LocalReplicaState::Running || self.selected_network == "ic")
            && self.replica.ping.is_some()
        {
            let ping = self.replica.ping.clone().unwrap();
            lines.push(Item.build("Replica dashboard URL: ", &self.replica.replica_url.clone()));
            lines.push(Item.build("Revision: ", &self.replica.replica_revision_url.clone()));
            lines.push(Item.build("Webserver: ", &self.replica.webserver_url));
            lines.push(Item.build("IC API version: ", &ping.ic_api_version));
            lines.push(Item.build("Replica health status: ", &ping.replica_health_status));
            if let Some(certified_height) = ping.certified_height {
                lines.push(Item.build("Certified height: ", &certified_height.to_string()));
            }
            if let Some(impl_hash) = &ping.impl_hash {
                lines.push(Item.build("Impl hash: ", &impl_hash));
            }
            lines.push(Last.build("Root key: ", &format!("{:?}", ping.root_key)));
        }

        let text = Text::from(lines);
        Paragraph::new(text)
            .wrap(Wrap { trim: true })
            .block(self.apply_style(WidgetStyle::Replica))
    }

    pub fn widget_network_selection_menu(&self) -> Paragraph {
        let mut lines = vec![];
        self.networks.iter().enumerate().for_each(|(idx, n)| {
            lines.push(Line::from(vec![if idx == self.selected_network_index {
                Span::styled(n.to_string(), self.style_selected())
            } else {
                Span::styled(n.to_string(), self.style_unselected())
            }]));
        });
        let text = Text::from(lines);
        Paragraph::new(text)
            .wrap(Wrap { trim: true })
            .block(self.apply_style(WidgetStyle::Replica))
    }

    pub fn widget_logs(&self) -> Paragraph {
        let mut text = Text::from(Line::from(vec![]));
        let collected_logs = &self.collected_logs;
        // let collected_logs = match &self.logs_source {
        //     LogsSource::Replica => self.collected_logs.clone(),
        //     LogsSource::File(Some(v)) => std::fs::read_to_string(v)
        //         .unwrap_or_default()
        //         .lines()
        //         .map(String::from)
        //         .collect::<Vec<String>>(),
        //     LogsSource::File(None) => vec![],
        // };

        if collected_logs.is_empty() {
            return Paragraph::new(Text::from(
                "No logs available, press 's' to start the replica".to_string(),
            ))
            .block(self.apply_style(WidgetStyle::Logs));
        } else {
            if self.logs_pane == LogsPane::CanisterLogs {
                if let Some(selected_canister_id) = self
                    .replica
                    .info
                    .as_ref()
                    .and_then(|i| i.canisters.get(self.selected_canister_index))
                    .and_then(|c| Some(c.canister_id.clone()))
                {
                    collected_logs.iter().for_each(|line| {
                        if line.contains(&selected_canister_id) {
                            text.extend(line.into_text().unwrap());
                        }
                    });
                }
            } else {
                collected_logs.iter().for_each(|log| {
                    text.extend(log.into_text().unwrap());
                });
            }
        }
        Paragraph::new(text).block(self.apply_style(WidgetStyle::Logs))
    }

    // pub fn widget_logs_file_selection(&self) -> Paragraph {
    //     let mut text = Text::from(Line::from(vec![]));
    //     let idx = self.selected_fs_index;
    //     if let Some(current_dir) = self.logfile_selection_menu_active.clone() {
    //         let dir_contents = std::fs::read_dir(current_dir);
    //         if let Ok(dir_contents) = dir_contents {
    //             dir_contents.enumerate().for_each(|(i, entry)| {
    //                 if let Ok(entry) = entry {
    //                     let path = entry.path();
    //                     let path = path.to_str().unwrap_or_default();
    //                     let path = path.replace(&self.path_to_dfx, "");
    //                     if idx == i {
    //                         text.extend(Text::from(Span::styled(
    //                             path,
    //                             Style::default()
    //                                 .add_modifier(Modifier::BOLD)
    //                                 .fg(Color::LightRed),
    //                         )));
    //                     } else {
    //                         text.extend(Text::from(Span::styled(path, Style::default())));
    //                     }
    //                 }
    //             });
    //         }
    //     }
    //     Paragraph::new(text).block(self.apply_style(WidgetStyle::Logs))
    // }
}
