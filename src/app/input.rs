use super::state::{LocalReplicaState, LogsPane};
use crate::{app::state::AppState, dfx_commands::DfxCommands};
use crossterm::event::{KeyCode, KeyEvent};
use std::time::Duration;

impl AppState {
    #[allow(unused_assignments)]
    pub fn handle_input(&mut self, key_event: KeyEvent, path_to_dfx: &str) {
        match key_event.code {
            KeyCode::Char('+') => {
                if self.refresh_interval < Duration::from_secs(10) {
                    self.refresh_interval = self.refresh_interval + Duration::from_millis(100)
                }
            }
            KeyCode::Char('-') => {
                if self.refresh_interval > Duration::from_millis(1000) {
                    self.refresh_interval = self.refresh_interval - Duration::from_millis(100)
                }
            }
            KeyCode::Char('i') => {
                self.network_selection_menu_active = false;
                self.identity_selection_menu_active = true;
            }
            KeyCode::Char('n') => {
                self.identity_selection_menu_active = false;
                self.network_selection_menu_active = true;
            }
            KeyCode::Esc => {
                self.network_selection_menu_active = false;
                self.identity_selection_menu_active = false;
                // self.logfile_selection_menu_active = None;
            }
            KeyCode::Enter => {
                if self.network_selection_menu_active {
                    self.network_selection_menu_active = false;
                    self.selected_network = self.networks[self.selected_network_index].clone();
                }
                if self.identity_selection_menu_active {
                    self.identity_selection_menu_active = false;
                    self.selected_identity = self.identities[self.selected_identity_index].clone();
                }
            }
            KeyCode::Char('s') => match self.replica.state {
                LocalReplicaState::Running => {
                    DfxCommands::StopReplica.run(&self, path_to_dfx);
                    {
                        let mut rx = self.replica_logs_reciver.take(); // Optionally consume any remaining logs
                        if let Some(rx) = rx {
                            while let Ok(log) = rx.try_recv() {
                                self.collected_logs.push(log);
                            }
                        }
                        rx = None;
                        self.replica_logs_reciver = None;
                    }
                    self.replica.state = LocalReplicaState::NotRunning;
                    self.collected_logs = vec![];
                }
                LocalReplicaState::NotRunning => {
                    self.replica_logs_reciver =
                        Some(DfxCommands::start_replica_stream(self.path_to_dfx.clone()));
                    self.replica.state = LocalReplicaState::Running;
                }
            },
            // KeyCode::Char('f') => {
            //     if self.logs_source == LogsSource::Replica {
            //         // self.collected_logs = vec![];
            //         self.logs_source = LogsSource::File(None);
            //         self.logfile_selection_menu_active = Some(dirs::home_dir().unwrap_or_default());
            //         // self.replica_logs_reciver = None;
            //     } else {
            //         // self.collected_logs = vec![];
            //         self.logs_source = LogsSource::Replica;
            //         // self.replica_logs_reciver =
            //         //     Some(DfxCommands::start_replica_stream(self.path_to_dfx.clone()));
            //     }
            // }
            // KeyCode::Left if self.logfile_selection_menu_active.is_some() => {
            //     // got to parent dir
            //     let mut path = self
            //         .logfile_selection_menu_active
            //         .take()
            //         .unwrap_or_default();
            //     path.pop();
            //     self.logfile_selection_menu_active = Some(path);
            // }
            // KeyCode::Right if self.logfile_selection_menu_active.is_some() => {
            //     // got to child dir
            //     let mut path = self
            //         .logfile_selection_menu_active
            //         .take()
            //         .unwrap_or_default();
            //     let entries = std::fs::read_dir(path.clone());
            //     if let Ok(entries) = entries {
            //         let entries = entries.map(|e| e.unwrap().path()).collect::<Vec<_>>();
            //         let entries_len = entries.len();
            //         if entries_len > 0 {
            //             self.selected_fs_index = (self.selected_fs_index + 1) % entries_len;
            //             path = entries[self.selected_fs_index].clone();
            //         }
            //         self.logfile_selection_menu_active = Some(path);
            //     }
            // }
            KeyCode::Left => match self.logs_pane {
                LogsPane::CanisterLogs => {
                    self.logs_pane = LogsPane::ReplicaLogs;
                }
                LogsPane::ReplicaLogs => {
                    self.logs_pane = LogsPane::CanisterLogs;
                }
            },
            KeyCode::Right => match self.logs_pane {
                LogsPane::CanisterLogs => {
                    self.logs_pane = LogsPane::ReplicaLogs;
                }
                LogsPane::ReplicaLogs => {
                    self.logs_pane = LogsPane::CanisterLogs;
                }
            },
            // KeyCode::Down if self.logfile_selection_menu_active.is_some() => {
            //     let mut path = self
            //         .logfile_selection_menu_active
            //         .take()
            //         .unwrap_or_default();
            //     let entries = std::fs::read_dir(path.clone());
            //     if let Ok(entries) = entries {
            //         let entries = entries.map(|e| e.unwrap().path()).collect::<Vec<_>>();
            //         let entries_len = entries.len();
            //         if entries_len > 0 {
            //             self.selected_fs_index = (self.selected_fs_index + 1) % entries_len;
            //             path = entries[self.selected_fs_index].clone();
            //         }
            //         self.logfile_selection_menu_active = Some(path);
            //     }
            // }
            // KeyCode::Up if self.logfile_selection_menu_active.is_some() => {
            //     let mut path = self
            //         .logfile_selection_menu_active
            //         .take()
            //         .unwrap_or_default();
            //     let entries = std::fs::read_dir(path.clone());
            //     if let Ok(entries) = entries {
            //         let entries = entries.map(|e| e.unwrap().path()).collect::<Vec<_>>();
            //         let entries_len = entries.len();
            //         if entries_len > 0 {
            //             self.selected_fs_index = if self.selected_fs_index > 0 {
            //                 self.selected_fs_index - 1
            //             } else {
            //                 entries_len - 1
            //             };
            //             path = entries[self.selected_fs_index].clone();
            //         }
            //         self.logfile_selection_menu_active = Some(path);
            //     }
            // }
            KeyCode::Down if self.network_selection_menu_active => {
                let networks_len = self.networks.len();
                if networks_len > 0 {
                    self.selected_network_index = (self.selected_network_index + 1) % networks_len;
                }
            }
            KeyCode::Down if self.identity_selection_menu_active => {
                let identities_len = self.identities.len();
                if identities_len > 0 {
                    self.selected_identity_index =
                        (self.selected_identity_index + 1) % identities_len;
                }
            }
            KeyCode::Down
                if !self.network_selection_menu_active && !self.identity_selection_menu_active =>
            {
                let canisters_len = self
                    .replica
                    .info
                    .as_ref()
                    .map_or(0, |info| info.canisters.len());
                if canisters_len > 0 {
                    self.selected_canister_index =
                        (self.selected_canister_index + 1) % canisters_len;
                }
            }
            KeyCode::Up if self.network_selection_menu_active => {
                let networks_len = self.networks.len();
                if networks_len > 0 {
                    self.selected_network_index = if self.selected_network_index > 0 {
                        self.selected_network_index - 1
                    } else {
                        networks_len - 1
                    };
                }
            }
            KeyCode::Up if self.identity_selection_menu_active => {
                let identities_len = self.identities.len();
                if identities_len > 0 {
                    self.selected_identity_index = if self.selected_identity_index > 0 {
                        self.selected_identity_index - 1
                    } else {
                        identities_len - 1
                    };
                }
            }
            KeyCode::Up
                if !self.network_selection_menu_active && !self.identity_selection_menu_active =>
            {
                let canisters_len = self
                    .replica
                    .info
                    .as_ref()
                    .map_or(0, |info| info.canisters.len());
                if canisters_len > 0 {
                    self.selected_canister_index = if self.selected_canister_index > 0 {
                        self.selected_canister_index - 1
                    } else {
                        canisters_len - 1
                    };
                }
            }
            _ => {}
        }
    }
}
