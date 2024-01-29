use crate::AppState;
use std::io::BufRead;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::{io, thread};

#[allow(unreachable_code, unused_imports, unused_variables, dead_code)]
pub enum DfxCommands {
    CyclesBalance,         // Get the cycle balance of the selected Identity's cycles wallet
    IdentityGetPrincipal, // Shows the textual representation of the Principal associated with the current identity
    IdentityGetWallet, // Gets the canister ID for the wallet associated with your identity on a network
    IdentityList,      // Lists existing identities
    IdentityWhoami,    // Shows the name of the current identity
    InfoNetworksJsonPath, // Show the path to network configuration file
    InfoReplicaPort,   // Show the port of the local replica
    InfoReplicaRev,    // Show the revision of the replica shipped with this dfx binary
    InfoWebserverPort, // Show the port of the webserver
    LedgerAccountId,   // Prints the ledger account identifier corresponding to a principal
    LedgerBalance,     // Prints the account balance of the user
    LedgerShowSubnetTypes, // Show available subnet types in the cycles minting canister
    Ping,              // Ping the IC replica
    StopReplica,       // Stop a local replica
    Version,           // Prints the dfx version
}

impl DfxCommands {
    pub fn run(&self, app_state: &AppState, path_to_dfx: &str) -> String {
        let identity = app_state.selected_identity.clone().trim().to_string();
        let network = app_state.selected_network.clone();
        //dbg!(network.clone());
        match self {
            DfxCommands::CyclesBalance => {
                let output = Command::new(path_to_dfx)
                    .args(["--identity", identity.as_str()])
                    // .args(["cycles", "balance"]) // TODO
                    .args(["wallet", "balance"])
                    // .args(["--network", network.as_str()]) // TODO
                    // .args(["--network", "ic"])
                    .output()
                    .expect("Failed to execute dfx cycles balance");
                String::from_utf8_lossy(&output.stdout).to_string()
            }
            DfxCommands::IdentityGetPrincipal => {
                let output = Command::new(path_to_dfx)
                    .args(["--identity", identity.as_str()])
                    .args(["identity", "get-principal"])
                    .args(["--network", network.as_str()])
                    .output()
                    .expect("Failed to execute dfx identity get-principal");
                String::from_utf8_lossy(&output.stdout).to_string()
            }
            DfxCommands::IdentityGetWallet => {
                let output = Command::new(path_to_dfx)
                    .args(["--identity", identity.as_str()])
                    .args(["identity", "get-wallet"])
                    .args(["--network", network.as_str()])
                    .output()
                    .expect("Failed to execute dfx identity get-wallet");
                String::from_utf8_lossy(&output.stdout).to_string()
            }
            DfxCommands::IdentityList => {
                let output = Command::new(path_to_dfx)
                    .args(["identity", "list"])
                    .output()
                    .expect("Failed to execute dfx identity list");
                String::from_utf8_lossy(&output.stdout).to_string()
            }
            DfxCommands::IdentityWhoami => {
                let output = Command::new(path_to_dfx)
                    .args(["identity", "whoami"])
                    .args(["--network", network.as_str()])
                    .output()
                    .expect("Failed to execute dfx identity whoami");
                String::from_utf8_lossy(&output.stdout).to_string()
            }
            DfxCommands::InfoNetworksJsonPath => {
                let output = Command::new(path_to_dfx)
                    .args(["--identity", identity.as_str()])
                    .args(["info", "networks-json-path"])
                    .args(["--network", network.as_str()])
                    .output()
                    .expect("Failed to execute dfx info networks-json-path");
                String::from_utf8_lossy(&output.stdout).to_string()
            }
            DfxCommands::InfoReplicaPort => {
                let output = Command::new(path_to_dfx)
                    .args(["info", "replica-port"])
                    .output()
                    .expect("Failed to execute dfx info replica-port");
                String::from_utf8_lossy(&output.stdout).to_string()
            }
            DfxCommands::InfoReplicaRev => {
                let output = Command::new(path_to_dfx)
                    .args(["info", "replica-rev"])
                    .output()
                    .expect("Failed to execute dfx info replica-rev");
                String::from_utf8_lossy(&output.stdout).to_string()
            }
            DfxCommands::InfoWebserverPort => {
                let output = Command::new(path_to_dfx)
                    .args(["info", "webserver-port"])
                    .output()
                    .expect("Failed to execute dfx info webserver-port");
                String::from_utf8_lossy(&output.stdout).to_string()
            }
            DfxCommands::LedgerAccountId => {
                let output = Command::new(path_to_dfx)
                    .args(["--identity", identity.as_str()])
                    .args(["ledger", "account-id"])
                    .args(["--network", network.as_str()])
                    .output()
                    .expect("Failed to execute dfx ledger account-id");
                String::from_utf8_lossy(&output.stdout).to_string()
            }
            DfxCommands::LedgerBalance => {
                let output = Command::new(path_to_dfx)
                    .args(["--identity", identity.as_str()])
                    .args(["ledger", "balance"])
                    // .args(["--network", network.as_str()]) // TODO
                    .args(["--network", "ic"])
                    .output()
                    .expect("Failed to execute dfx ledger balance");
                String::from_utf8_lossy(&output.stdout).to_string()
            }
            DfxCommands::LedgerShowSubnetTypes => {
                let output = Command::new(path_to_dfx)
                    .args(["--identity", identity.as_str()])
                    .args(["ledger", "show-subnet-types"])
                    .args(["--network", network.as_str()])
                    .output()
                    .expect("Failed to execute dfx ledger show-subnet-types");
                String::from_utf8_lossy(&output.stdout).to_string()
            }
            DfxCommands::Ping => {
                let output = Command::new(path_to_dfx)
                    .args(["ping", network.as_str()])
                    .output()
                    .expect("Failed to execute dfx ping ic");
                if output.status.success() {
                    String::from_utf8_lossy(&output.stdout)
                        .to_string()
                        .replace("  ", ", ")
                        .replace("\n,", "")
                } else {
                    "Error".to_string()
                }
            }
            DfxCommands::StopReplica => {
                let output = Command::new(path_to_dfx)
                    .args(["stop"])
                    .output()
                    .expect("Failed to execute dfx stop replica");
                format!(
                    "{}{}",
                    String::from_utf8_lossy(&output.stdout).to_string(),
                    String::from_utf8_lossy(&output.stderr).to_string()
                )
            }
            DfxCommands::Version => {
                let output = Command::new(path_to_dfx)
                    .args(["version"])
                    .output()
                    .expect("Failed to execute dfx --version");
                String::from_utf8_lossy(&output.stdout).to_string()
            }
        }
    }
    pub fn start_replica_stream(path_to_dfx: String) -> mpsc::Receiver<String> {
        let (sender, receiver) = mpsc::channel();
        // Spawn a new thread to run the command
        let cloned_path_to_dfx = path_to_dfx.clone();
        thread::spawn(move || {
            let mut child = Command::new(cloned_path_to_dfx)
                .args(&["start"])
                // .stdout(Stdio::piped())
                .stderr(Stdio::piped()) // Capture stderr as well
                .spawn()
                .expect("Failed to start dfx start replica");

            // Combine stdout and stderr into a single stream
            // let stdout = child
            //     .stdout
            //     .take()
            //     .expect("Failed to capture standard output.");
            let stderr = child
                .stderr
                .take()
                .expect("Failed to capture standard error.");
            // let reader = io::BufReader::new(stdout);
            let reader_err = io::BufReader::new(stderr);

            // Use select to read from both stdout and stderr
            // let stdout_lines = reader
            //     .lines()
            //     .map(|line| line.expect("Could not read line from stdout"));
            let stderr_lines = reader_err
                .lines()
                .map(|line| line.expect("Could not read line from stderr"));

            // for line in stdout_lines.chain(stderr_lines) {
            for line in stderr_lines {
                // dbg!(&line);
                if sender.send(line).is_err() {
                    break; // Stop if receiving end is dropped
                }
            }

            // kill the process forcefully
            DfxCommands::StopReplica.run(&AppState::default(), &path_to_dfx);
            // let _ = child.kill();
            // Wait for the child process to finish, if needed
            // let _ = child.wait();
        });

        receiver
    }

    // above function leaks output to my TUI app, which is not what I want, I want the Rx to swallow whole output and hold it until i use it
}

#[test]
fn test_ping_command() {
    use serde_json::{from_str, Value};
    use std::collections::HashMap;
    let app_state = AppState::default();
    let path_to_dfx = "dfx";
    let output = DfxCommands::Ping.run(&app_state, path_to_dfx);
    let json: HashMap<String, Value> = from_str(&output).unwrap();
    assert!(json.contains_key("ic_api_version"));
}
