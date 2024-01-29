use crate::{dfx_commands::*, dfx_project::ProjectDatabase, parse_replica_dashboard::ReplicaInfo};
use serde::Deserialize;
use std::{sync::mpsc, time::Duration};

#[derive(Debug, Default)]
pub struct AppState {
    pub collected_logs: Vec<String>,
    pub db: ProjectDatabase,
    pub identities: Vec<String>,
    pub identity_selection_menu_active: bool,
    // pub logfile_selection_menu_active: Option<PathBuf>,
    pub logs_pane: LogsPane,
    // pub logs_source: LogsSource,
    pub network_selection_menu_active: bool,
    pub networks: Vec<String>,
    pub path_to_dfx: String,
    pub refresh_interval: Duration,
    pub replica: Replica,
    pub replica_logs_reciver: Option<mpsc::Receiver<String>>,
    pub selected_canister_index: usize,
    // pub selected_fs_index: usize,
    pub selected_identity: String,
    pub selected_identity_cycles: Option<String>,
    pub selected_identity_icp: Option<String>,
    pub selected_identity_index: usize,
    pub selected_identity_principal: Option<String>,
    pub selected_network: String,
    pub selected_network_index: usize,
}

#[derive(Default, PartialEq, Debug)]
pub enum LogsSource {
    #[default]
    Replica,
    // File(Option<PathBuf>),
}

#[derive(Default, PartialEq, Debug)]
pub enum LogsPane {
    #[default]
    ReplicaLogs,
    CanisterLogs,
}

#[derive(Debug, PartialEq, Default, Clone)]
pub enum LocalReplicaState {
    Running,
    #[default]
    NotRunning,
}

impl std::fmt::Display for LocalReplicaState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LocalReplicaState::Running => write!(f, "Running"),
            LocalReplicaState::NotRunning => write!(f, "Not running"),
        }
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct Replica {
    pub replica_revision_url: String,
    pub replica_url: String,
    pub ping: Option<PingOutput>,
    pub webserver_url: String,
    pub state: LocalReplicaState,
    pub info: Option<ReplicaInfo>,
    pub webserver_port: String,
    pub replica_port: String,
}

#[derive(Deserialize, Debug, PartialEq, Default, Clone)]
pub struct PingOutput {
    pub ic_api_version: String,
    pub replica_health_status: String,
    pub root_key: Vec<u8>,
    pub certified_height: Option<u64>,
    pub impl_hash: Option<String>,
}

impl AppState {
    pub fn new(path_to_dfx: &str, db: ProjectDatabase) -> Self {
        let mut s = Self::default();
        s.selected_identity = DfxCommands::IdentityWhoami.run(&s, path_to_dfx);
        s.selected_network = "local".to_string();
        s.refresh_interval = Duration::from_millis(1500);
        s.networks = vec!["local".to_string(), "ic".to_string()];
        s.path_to_dfx = path_to_dfx.to_string();
        s.db = db;
        if DfxCommands::Ping.run(&s, path_to_dfx).contains("Error") {
            s.replica.state = LocalReplicaState::NotRunning;
        } else {
            s.replica.state = LocalReplicaState::Running;
        }
        s
    }

    pub fn refresh(&mut self, path_to_dfx: &str) {
        if self.replica_logs_reciver.is_some() {
            while let Ok(log) = self.replica_logs_reciver.as_ref().unwrap().try_recv() {
                self.collected_logs.push(log);
            }
        }
        self.selected_identity_principal =
            Some(DfxCommands::IdentityGetPrincipal.run(self, path_to_dfx));
        self.selected_identity_icp = Some(DfxCommands::LedgerBalance.run(self, path_to_dfx));
        self.selected_identity_cycles = Some(DfxCommands::CyclesBalance.run(self, path_to_dfx));
        if self.identity_selection_menu_active {
            self.identities = DfxCommands::IdentityList
                .run(self, &self.path_to_dfx)
                .split("\n")
                .map(|s| s.to_string())
                .collect();
        }

        let ping = &DfxCommands::Ping.run(self, path_to_dfx);
        if ping.contains("Error")
            && self.selected_network == "local"
            && self.replica_logs_reciver.is_none()
        {
            self.replica.state = LocalReplicaState::NotRunning;
        } else {
            self.replica.state = LocalReplicaState::Running;
        }
        self.replica.ping = serde_json::from_str(ping).ok();
        match (self.selected_network.clone(), self.replica.state.clone()) {
            (network, LocalReplicaState::Running) if network == "local" => {
                self.replica.webserver_port = DfxCommands::InfoWebserverPort
                    .run(self, path_to_dfx)
                    .trim()
                    .to_string();
                self.replica.replica_port = DfxCommands::InfoReplicaPort
                    .run(self, path_to_dfx)
                    .trim()
                    .to_string();
                self.replica.webserver_url =
                    format!("http://localhost:{}", self.replica.webserver_port);
                self.replica.replica_revision_url = format!(
                    "https://dashboard.internetcomputer.org/release/{}",
                    DfxCommands::InfoReplicaRev
                        .run(self, path_to_dfx)
                        .trim()
                        .to_string()
                );
                self.replica.replica_url =
                    format!("http://localhost:{}/_/dashboard", self.replica.replica_port);
                if let Ok(data) = reqwest::blocking::get(self.replica.replica_url.as_str()) {
                    let dashboard_html = data.text().unwrap();
                    self.replica.info =
                        ReplicaInfo::parse_from_html_dashboard(&dashboard_html).ok();
                }
            }

            (network, LocalReplicaState::NotRunning) if network == "local" => {
                self.replica.replica_revision_url = "N/A".to_string();
                self.replica.replica_url = "N/A".to_string();
                self.replica.webserver_url = "N/A".to_string();
                self.replica.info = None;
            }
            (network, _) if network == "ic" => {
                self.replica.replica_revision_url =
                    "https://dashboard.internetcomputer.org/releases".to_string();
                self.replica.replica_url = "http://ic0.app".to_string();
                self.replica.webserver_url = "N/A".to_string();
                self.replica.info = None;
            }
            _ => {}
        }
        self.db.refresh().unwrap();
        self.db.save().unwrap();
    }
}
