use crate::app::state::AppState;
use crate::app::widgets::style::ListItem::{Empty, Header, Item, Last};
use ratatui::{prelude::*, text::Span, widgets::*};

use super::style::WidgetStyle;

impl AppState {
    pub fn widget_canisters_list(&self) -> Paragraph {
        let mut lines = vec![];
        if self.replica.info.is_none() {
            // return all canisters
            return Paragraph::new(Span::raw("Loading..."))
                .block(self.apply_style(WidgetStyle::Canisters));
        }
        if self.replica.info.clone().unwrap().canisters.len() == 0 {
            // return all canisters
            return Paragraph::new(Span::raw("No canisters found, try deploying some first."))
                .wrap(Wrap { trim: true })
                .block(self.apply_style(WidgetStyle::Canisters));
        }
        self.replica
            .info
            .clone()
            .unwrap()
            .canisters
            .iter()
            .enumerate()
            .for_each(|(idx, c)| {
                if let Some((_, canister_name, _)) =
                    self.db.get_info(&c.canister_id, &self.selected_network)
                {
                    lines.push(Line::from(vec![Span::styled(
                        format!(
                            "Canister ID: {}, Canister name: {canister_name}",
                            c.canister_id.clone()
                        ),
                        if idx == self.selected_canister_index {
                            self.style_selected()
                        } else {
                            self.style_unselected()
                        },
                    )]));
                } else {
                    lines.push(Line::from(vec![Span::styled(
                        format!("Canister ID: {}", c.canister_id.clone()),
                        if idx == self.selected_canister_index {
                            self.style_selected()
                        } else {
                            self.style_unselected()
                        },
                    )]));
                }
            });

        let text = Text::from(lines);
        Paragraph::new(text)
            .wrap(Wrap { trim: true })
            .block(self.apply_style(WidgetStyle::Canisters))
    }

    pub fn widget_canister_info(&self) -> Paragraph {
        let mut lines = vec![];
        if self.replica.info.is_none() {
            // return all canisters
            return Paragraph::new(Span::raw("Loading..."))
                .block(self.apply_style(WidgetStyle::CanisterInfo));
        }
        let binding = self.replica.info.clone().unwrap();
        let canister = binding.canisters.get(self.selected_canister_index).clone();
        if canister.is_none() {
            // return all canisters
            return Paragraph::new(Span::raw("No canisters found, try deploying some first."))
                .wrap(Wrap { trim: true })
                .block(self.apply_style(WidgetStyle::CanisterInfo));
        }
        let canister = canister.unwrap();
        if let Some((project_name, canister_name, canister_info)) = self
            .db
            .get_info(&canister.canister_id, &self.selected_network)
        {
            lines.push(Header.build("DFX project: ", &project_name));
            lines.push(Item.build("Canister name: ", &canister_name));
            if let Some(dependencies) = canister_info.dependencies {
                lines.push(Item.build("Dependencies: ", &format!("{:?}", dependencies)));
            }
            if let Some(source) = canister_info.source {
                lines.push(Item.build("Source: ", &format!("{:?}", source)));
            }
            if let Some(frontend) = canister_info.frontend {
                lines.push(Item.build("Frontend: ", &format!("{:?}", frontend)));
            }
            lines.push(Last.build("Type: ", &canister_info.canister_type));
            lines.push(Empty.build("", ""));
        }
        lines.push(Header.build("Canister ID: ", canister.canister_id.as_str()));
        if canister
            .exports
            .exported_query_functions
            .contains(&"http_request".to_string())
        {
            let canister_url = format!(
                "http://{}.localhost:{}",
                canister.canister_id, self.replica.webserver_port
            );
            lines.push(Item.build("HTTP Endpoint: ", &canister_url));
        }
        lines.push(Item.build("Memory Allocation: ", &canister.memory_allocation.clone()));
        lines.push(Item.build("Last Execution Round: ", &canister.last_execution_round));
        lines.push(Item.build("Controllers: ", &canister.controllers));
        lines.push(Item.build("Certified Data Length: ", &canister.certified_data_length));
        lines.push(Item.build(
            "Canister History Memory Usage: ",
            &canister.canister_history_memory_usage,
        ));
        lines.push(Item.build("Execution State: ", &canister.execution_state));
        lines.push(Item.build(
            "Last Full Execution Round: ",
            &canister.last_full_execution_round,
        ));
        lines.push(Item.build("Compute Allocation: ", &canister.compute_allocation));
        lines.push(Item.build("Freeze Threshold: ", &canister.freeze_threshold));
        lines.push(Item.build("Memory Usage: ", &canister.memory_usage));
        lines.push(Item.build("Accumulated Priority: ", &canister.accumulated_priority));
        lines.push(Item.build("Cycles Balance: ", &canister.cycles_balance.to_string()));
        if !canister.exports.exported_query_functions.is_empty() {
            lines.push(Item.build(
                "Exported Query functions: ",
                &canister.exports.exported_query_functions.join(", "),
            ));
        }
        if !canister.exports.exported_update_functions.is_empty() {
            lines.push(Item.build(
                "Exported Update functions: ",
                &canister.exports.exported_update_functions.join(", "),
            ));
        }
        if !canister.exports.exported_system_functions.is_empty() {
            lines.push(Item.build(
                "Exported System functions: ",
                &canister.exports.exported_system_functions.join(", "),
            ));
        }
        lines.push(Item.build(
            "Exports heartbeat: ",
            &canister.exports.exports_heartbeat.to_string(),
        ));
        lines.push(Last.build(
            "Exports global timer: ",
            &canister.exports.exports_global_timer.to_string(),
        ));

        let text = Text::from(lines);
        Paragraph::new(text)
            .wrap(Wrap { trim: true })
            .block(self.apply_style(WidgetStyle::CanisterInfo))
    }
}
