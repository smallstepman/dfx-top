use crate::app::widgets::style::ListItem::{Header, Item, Last};
use crate::{app::state::AppState, dfx_commands::DfxCommands};
use ratatui::{prelude::*, text::Span, widgets::*};

use super::style::WidgetStyle;

impl AppState {
    pub fn widget_identity(&self) -> Paragraph {
        let mut lines = vec![];
        lines.push(Header.build("Selected DFX Identity: ", &self.selected_identity));
        let principal = self.selected_identity_principal.as_ref();
        lines.push(Item.build("Principal: ", &principal.unwrap_or(&"N/A".to_string())));
        let icp = self.selected_identity_icp.as_ref();
        lines.push(Item.build("ICP balance: ", &icp.unwrap_or(&"N/A".to_string())));
        let cycles = self.selected_identity_cycles.as_ref();
        lines.push(Last.build("Cycles: ", &cycles.unwrap_or(&"N/A".to_string())));
        let text = Text::from(lines);
        Paragraph::new(text)
            .wrap(Wrap { trim: true })
            .block(self.apply_style(WidgetStyle::Identity))
    }

    pub fn widget_identity_selection_menu(&self) -> Paragraph {
        let mut lines = vec![];
        DfxCommands::IdentityList
            .run(self, &self.path_to_dfx)
            .split("\n")
            .map(|s| s.to_string())
            .enumerate()
            .for_each(|(idx, n)| {
                lines.push(Line::from(vec![if idx == self.selected_identity_index {
                    Span::styled(n.to_string(), self.style_selected())
                } else {
                    Span::styled(n.to_string(), self.style_unselected())
                }]));
            });

        let text = Text::from(lines);
        Paragraph::new(text)
            .wrap(Wrap { trim: true })
            .block(self.apply_style(WidgetStyle::Identity))
    }
}
