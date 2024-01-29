use anyhow::Result;
use scraper::{Element, ElementRef};
use scraper::{Html, Selector};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Exports {
    pub exported_query_functions: Vec<String>,
    pub exported_update_functions: Vec<String>,
    pub exported_system_functions: Vec<String>,
    pub exports_heartbeat: bool,
    pub exports_global_timer: bool,
}

#[test]
fn test_exported_functions_convertion() {
    use pretty_assertions::assert_eq;
    let s = r#"ExportedFunctions { exported_functions: {Update("add_address"), Update("add_controller"), Update("authorize"), Update("deauthorize"), Update("remove_address"), Update("remove_controller"), Update("set_name"), Update("set_short_name"), Update("wallet_call"), Update("wallet_call128"), Update("wallet_create_canister"), Update("wallet_create_canister128"), Update("wallet_create_wallet"), Update("wallet_create_wallet128"), Update("wallet_receive"), Update("wallet_send"), Update("wallet_send128"), Update("wallet_store_wallet_wasm"), Query("get_chart"), Query("get_controllers"), Query("get_custodians"), Query("get_events"), Query("get_events128"), Query("get_managed_canister_events"), Query("get_managed_canister_events128"), Query("http_request"), Query("list_addresses"), Query("list_managed_canisters"), Query("name"), Query("wallet_api_version"), Query("wallet_balance"), Query("wallet_balance128"), System(CanisterInit), System(CanisterPreUpgrade), System(CanisterPostUpgrade)}, exports_heartbeat: false, exports_global_timer: false }"#;
    let expected = Exports {
        exported_query_functions: vec![
            "get_chart".to_string(),
            "get_controllers".to_string(),
            "get_custodians".to_string(),
            "get_events".to_string(),
            "get_events128".to_string(),
            "get_managed_canister_events".to_string(),
            "get_managed_canister_events128".to_string(),
            "http_request".to_string(),
            "list_addresses".to_string(),
            "list_managed_canisters".to_string(),
            "name".to_string(),
            "wallet_api_version".to_string(),
            "wallet_balance".to_string(),
            "wallet_balance128".to_string(),
        ],
        exported_update_functions: vec![
            "add_address".to_string(),
            "add_controller".to_string(),
            "authorize".to_string(),
            "deauthorize".to_string(),
            "remove_address".to_string(),
            "remove_controller".to_string(),
            "set_name".to_string(),
            "set_short_name".to_string(),
            "wallet_call".to_string(),
            "wallet_call128".to_string(),
            "wallet_create_canister".to_string(),
            "wallet_create_canister128".to_string(),
            "wallet_create_wallet".to_string(),
            "wallet_create_wallet128".to_string(),
            "wallet_receive".to_string(),
            "wallet_send".to_string(),
            "wallet_send128".to_string(),
            "wallet_store_wallet_wasm".to_string(),
        ],
        exported_system_functions: vec![
            "CanisterInit".to_string(),
            "CanisterPreUpgrade".to_string(),
            "CanisterPostUpgrade".to_string(),
        ],
        exports_heartbeat: false,
        exports_global_timer: false,
    };
    let exports = Exports::from_str(s.to_string()).unwrap();
    assert_eq!(
        exports.exported_query_functions,
        vec![
            "get_chart",
            "get_controllers",
            "get_custodians",
            "get_events",
            "get_events128",
            "get_managed_canister_events",
            "get_managed_canister_events128",
            "http_request",
            "list_addresses",
            "list_managed_canisters",
            "name",
            "wallet_api_version",
            "wallet_balance",
            "wallet_balance128"
        ]
    );
    assert_eq!(
        exports.exported_update_functions,
        vec![
            "add_address",
            "add_controller",
            "authorize",
            "deauthorize",
            "remove_address",
            "remove_controller",
            "set_name",
            "set_short_name",
            "wallet_call",
            "wallet_call128",
            "wallet_create_canister",
            "wallet_create_canister128",
            "wallet_create_wallet",
            "wallet_create_wallet128",
            "wallet_receive",
            "wallet_send",
            "wallet_send128",
            "wallet_store_wallet_wasm"
        ]
    );
    assert_eq!(
        exports.exported_system_functions,
        vec!["CanisterInit", "CanisterPreUpgrade", "CanisterPostUpgrade"]
    );
    assert_eq!(expected, exports);
}

impl Exports {
    fn from_str(s: String) -> Option<Exports> {
        let mut exports = Exports::default();
        let mut tmp = vec![];
        let s = s.replace("exported_functions: {", "");
        if let Some(functions_start) = s.find('{') {
            if let Some(functions_end) = s.find('}') {
                let exported_functions_section = &s[functions_start + 1..functions_end];
                tmp = exported_functions_section
                    .split(',')
                    .map(|s| s.trim().trim_matches('"').to_string())
                    .collect();
            }
        }
        exports.exported_query_functions =
            Exports::get_specific_functions(tmp.clone(), "Query".to_string());
        exports.exported_update_functions =
            Exports::get_specific_functions(tmp.clone(), "Update".to_string());
        exports.exported_system_functions =
            Exports::get_specific_functions(tmp.clone(), "System".to_string());

        let exports_heartbeat_keyword = "exports_heartbeat:";
        let exports_global_timer_keyword = "exports_global_timer:";
        if let Some(heartbeat_start) = s.find(exports_heartbeat_keyword) {
            let heartbeat_value_start = heartbeat_start + exports_heartbeat_keyword.len();
            if let Some(substring) = s.get(heartbeat_value_start..) {
                if let Some(heartbeat_end) = substring.find(',') {
                    let heartbeat_str = &substring[..heartbeat_end].trim();
                    exports.exports_heartbeat = if heartbeat_str == &"true" {
                        true
                    } else {
                        false
                    };
                }
            }
        }
        if let Some(global_timer_start) = s.find(exports_global_timer_keyword) {
            let global_timer_value_start = global_timer_start + exports_global_timer_keyword.len();
            if let Some(substring) = s.get(global_timer_value_start..) {
                let global_timer_str = substring.replace("}", "");
                exports.exports_global_timer = if global_timer_str.trim() == "true" {
                    true
                } else if global_timer_str.trim() == "false" {
                    false
                } else {
                    return None;
                };
            }
        }

        Some(exports)
    }

    pub fn get_specific_functions(exported_functions: Vec<String>, kind: String) -> Vec<String> {
        exported_functions
            .iter()
            .filter(|s| s.starts_with(&kind))
            .map(|s| {
                if kind == "System" {
                    s.replace(&format!("System("), "").replace(")", "")
                } else {
                    s.replace(&format!("{kind}(\""), "").replace("\")", "")
                }
            })
            .collect()
    }
}

#[derive(Debug, PartialEq, Default, Clone)]
pub struct ReplicaInfo {
    pub canisters: Vec<CanisterInfo>,
    pub http_server_config: String, // Add this field
    pub replica_version: String,
    pub subnet_type: String,
    pub total_compute_allocation: String,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CanisterInfo {
    pub canister_id: String,
    pub status: String,
    pub memory_allocation: String,
    pub last_execution_round: String,
    pub controllers: String,
    pub certified_data_length: String,
    pub canister_history_memory_usage: String,
    pub execution_state: String,
    pub last_full_execution_round: String,
    pub exports: Exports,
    pub compute_allocation: String,
    pub freeze_threshold: String,
    pub memory_usage: String,
    pub accumulated_priority: String,
    pub cycles_balance: String,
}

impl ReplicaInfo {
    pub fn parse_from_html_dashboard(html: &str) -> Result<ReplicaInfo, String> {
        let document = Html::parse_fragment(html);
        // dbg!(&document.tree.nodes());
        let mut replica_dashboard = ReplicaInfo {
            replica_version: "".into(),
            subnet_type: "".into(),
            total_compute_allocation: "".into(),
            http_server_config: "".into(),
            canisters: vec![],
        };

        // Parse the header values
        // Assumes that these are in a specific order
        let binding = Selector::parse("td.debug").unwrap();
        let header_data = document.select(&binding);
        let header_values: Vec<String> = header_data
            .map(|n| n.inner_html().as_str().to_string())
            .collect();
        replica_dashboard.replica_version = header_values.get(0).unwrap_or(&"".to_string()).clone();
        replica_dashboard.subnet_type = header_values.get(1).unwrap_or(&"".to_string()).clone();
        replica_dashboard.total_compute_allocation =
            header_values.get(2).unwrap_or(&"".to_string()).clone();
        let config_selector = Selector::parse("div.debug > pre").unwrap();
        let config_element = document.select(&config_selector).next(); // Take the first match
        replica_dashboard.http_server_config =
            config_element.map_or("".to_string(), |n| n.inner_html().trim().to_string());

        // Parse the Canisters
        let canister_summary_selector = Selector::parse("summary").unwrap();
        for summary in document.select(&canister_summary_selector) {
            let canister_id = summary.inner_html();

            let mut data_map = HashMap::new();
            if let Some(details) = summary.parent_element() {
                let td = Selector::parse("td").unwrap();
                let tds = details
                    .parent_element()
                    .unwrap()
                    .parent_element()
                    .unwrap()
                    .parent_element()
                    .unwrap()
                    .select(&td)
                    .map(|v| v.inner_html())
                    .collect::<Vec<_>>();
                let get_val = |nth: usize| tds.iter().rev().nth(nth).unwrap().trim().to_string();
                data_map.insert("status".to_string(), get_val(2));
                data_map.insert("memory_allocation".to_string(), get_val(1));
                data_map.insert("last_execution_round".to_string(), get_val(0));
                details
                    .select(&Selector::parse("tr").unwrap())
                    .for_each(|tr| {
                        let binding = Selector::parse("td").unwrap();
                        let mut tds = tr.select(&binding);
                        if let (Some(key_elem), Some(value_elem)) = (tds.next(), tds.next()) {
                            let get_val = |elem: ElementRef| {
                                elem.text().collect::<Vec<_>>().join("").trim().to_string()
                            };
                            data_map.insert(get_val(key_elem), get_val(value_elem));
                        }
                    });
            }
            let map_get = |key: &str| data_map.get(key).cloned().unwrap_or_default();
            let canister_info = CanisterInfo {
                canister_id,
                status: map_get("status"),
                memory_allocation: map_get("memory_allocation"),
                last_execution_round: map_get("last_execution_round"),
                controllers: map_get("controllers"),
                certified_data_length: map_get("certified_data length"),
                canister_history_memory_usage: map_get("canister_history_memory_usage"),
                execution_state: "".to_string(), // Field not captured correctly in your example
                exports: Exports::from_str(map_get("exports")).unwrap_or_default(),
                last_full_execution_round: map_get("last_full_execution_round"),
                compute_allocation: map_get("compute_allocation"),
                freeze_threshold: map_get("freeze_threshold (seconds)"),
                memory_usage: map_get("memory_usage"),
                accumulated_priority: map_get("accumulated_priority"),
                cycles_balance: map_get("Cycles balance"),
            };

            replica_dashboard.canisters.push(canister_info);
        }

        Ok(replica_dashboard)
    }
}

#[test]
fn parse_example_html() {
    use pretty_assertions::assert_eq;
    const HTML: &str = r#"
<!DOCTYPE html>
<!-- saved from url=(0034)http://localhost:53161/_/dashboard -->
<html lang="en"><head><meta http-equiv="Content-Type" content="text/html; charset=UTF-8">

    <title>Internet Computer Replica Dashboard</title>
</head>
<body data-new-gr-c-s-check-loaded="14.1152.0" data-gr-ext-installed="" data-gr-ext-disabled="forever">
<h1>Internet Computer Replica Dashboard</h1>

<h2>Subnet Settings &amp; Parameters</h2>
<table>
    <tbody><tr>
        <td>Replica Version</td>
        <td class="debug">0.9.0</td>
    </tr>
    <tr>
        <td>Subnet Type</td>
        <td class="debug">System</td>
    </tr>
    <tr>
        <td>Total Compute Allocation</td>
        <td class="debug">0 %</td>
    </tr>
</tbody></table>
<h2>Http Server Config</h2>
<div class="debug">
    <pre>Config { listen_addr: 127.0.0.1:0, port_file_path: Some("/Users/mnl/Library/Application Support/org.dfinity.dfx/network/local/replica-configuration/replica-1.port"), connection_read_timeout_seconds: 1200, request_timeout_seconds: 300, http_max_concurrent_streams: 256, max_request_size_bytes: 5242880, max_delegation_certificate_size_bytes: 1048576, max_request_receive_seconds: 300, max_read_state_concurrent_requests: 100, max_status_concurrent_requests: 100, max_catch_up_package_concurrent_requests: 100, max_dashboard_concurrent_requests: 100, max_call_concurrent_requests: 50, max_query_concurrent_requests: 400, max_pprof_concurrent_requests: 5 }</pre>
</div>
<h2>Canisters</h2>
<div>Info at height <span class="debug">133</span></div>
<div class="debug">
<table>
    <tbody><tr>
        <th class="text">Canister id</th>
        <th class="text">Status</th>
        <th class="number">Memory allocation</th>
        <th class="number">Last Execution Round</th>
    </tr>
    <tr class="row-separator">
        <td colspan="100%"></td>
    </tr>

    <tr>
        <td class="text">
            <details>
                <summary>bnz7o-iuaaa-aaaaa-qaaaa-cai</summary>
                <div class="verbose">
                    <h3>System state</h3>
                    <table>
                        <tbody><tr><td>controllers</td><td>trg6r-vqw4x-tcu5z-pgm4z-nmas4-ailxn-rjavv-zbzhi-jy2oy-wjrpf-hqe</td></tr>
                        <tr><td>certified_data length</td><td>32 bytes</td></tr>
                        <tr><td>canister_history_memory_usage</td><td>238 bytes</td></tr>
                    </tbody></table>
                    <h3>Execution state</h3>

                    <table>
                        <tbody><tr><td>canister_root</td><td>NOT_USED/canister_states/80000000001000000101</td></tr>
                        <tr><td>wasm_binary size</td><td>1775104 bytes</td></tr>
                        <tr><td>wasm_binary sha256</td><td>c1290ad65e6c9f840928637ed7672b688216a9c1e919eacbacc22af8c904a5e3</td></tr>
                        <tr><td>heap_size</td><td>85 pages</td></tr>
                        <tr><td>stable_memory_size</td><td>0 pages</td></tr>
                        <tr><td>exports</td><td>

                              ExportedFunctions { exported_functions: {Update("add_address"), Update("add_controller"), Update("authorize"), Update("deauthorize"), Update("remove_address"), Update("remove_controller"), Update("set_name"), Update("set_short_name"), Update("wallet_call"), Update("wallet_call128"), Update("wallet_create_canister"), Update("wallet_create_canister128"), Update("wallet_create_wallet"), Update("wallet_create_wallet128"), Update("wallet_receive"), Update("wallet_send"), Update("wallet_send128"), Update("wallet_store_wallet_wasm"), Query("get_chart"), Query("get_controllers"), Query("get_custodians"), Query("get_events"), Query("get_events128"), Query("get_managed_canister_events"), Query("get_managed_canister_events128"), Query("http_request"), Query("list_addresses"), Query("list_managed_canisters"), Query("name"), Query("wallet_api_version"), Query("wallet_balance"), Query("wallet_balance128"), System(CanisterInit), System(CanisterPreUpgrade), System(CanisterPostUpgrade)}, exports_heartbeat: false, exports_global_timer: false }

                        </td></tr>
                    </tbody></table>

                    <h3>Scheduler state</h3>
                    <table>
                        <tbody><tr><td>last_full_execution_round</td><td>104</td></tr>
                        <tr><td>compute_allocation</td><td>0%</td></tr>
                        <tr><td>freeze_threshold (seconds)</td><td>2592000</td></tr>
                        <tr><td>memory_usage</td><td>7345934</td></tr>
                        <tr><td>accumulated_priority</td><td>0 </td></tr>
                        <tr><td>Cycles balance</td><td>93_800_000_000_000</td></tr>
                    </tbody></table>
                </div>
            </details>
        </td>
        <td class="text">Running</td>
        <td class="number">
	    best-effort
        </td>
        <td class="number">104</td>
    </tr>

    <tr>
        <td class="text">
            <details>
                <summary>bkyz2-fmaaa-aaaaa-qaaaq-cai</summary>
                <div class="verbose">
                    <h3>System state</h3>
                    <table>
                        <tbody><tr><td>controllers</td><td>bnz7o-iuaaa-aaaaa-qaaaa-cai trg6r-vqw4x-tcu5z-pgm4z-nmas4-ailxn-rjavv-zbzhi-jy2oy-wjrpf-hqe</td></tr>
                        <tr><td>certified_data length</td><td>0 bytes</td></tr>
                        <tr><td>canister_history_memory_usage</td><td>268 bytes</td></tr>
                    </tbody></table>
                    <h3>Execution state</h3>

                    <table>
                        <tbody><tr><td>canister_root</td><td>NOT_USED/canister_states/80000000001000010101</td></tr>
                        <tr><td>wasm_binary size</td><td>130856 bytes</td></tr>
                        <tr><td>wasm_binary sha256</td><td>78f59d500b791021f591aedec685df18936982b3971538b94d0d1e3190faebaf</td></tr>
                        <tr><td>heap_size</td><td>33 pages</td></tr>
                        <tr><td>stable_memory_size</td><td>0 pages</td></tr>
                        <tr><td>exports</td><td>

                              ExportedFunctions { exported_functions: {Update("__motoko_async_helper"), Update("__motoko_gc_trigger"), Query("__get_candid_interface_tmp_hack"), Query("__motoko_stable_var_info"), Query("greet"), System(CanisterStart), System(CanisterInit), System(CanisterPreUpgrade), System(CanisterPostUpgrade), System(CanisterGlobalTimer)}, exports_heartbeat: false, exports_global_timer: true }

                        </td></tr>
                    </tbody></table>

                    <h3>Scheduler state</h3>
                    <table>
                        <tbody><tr><td>last_full_execution_round</td><td>0</td></tr>
                        <tr><td>compute_allocation</td><td>0%</td></tr>
                        <tr><td>freeze_threshold (seconds)</td><td>2592000</td></tr>
                        <tr><td>memory_usage</td><td>2294162</td></tr>
                        <tr><td>accumulated_priority</td><td>0 </td></tr>
                        <tr><td>Cycles balance</td><td>3_100_000_000_000</td></tr>
                    </tbody></table>
                </div>
            </details>
        </td>
        <td class="text">Running</td>
        <td class="number">
        best-effort
        </td>
        <td class="number">0</td>
    </tr>

    <tr>
        <td class="text">
            <details>
                <summary>bd3sg-teaaa-aaaaa-qaaba-cai</summary>
                <div class="verbose">
                    <h3>System state</h3>
                    <table>
                        <tbody><tr><td>controllers</td><td>bnz7o-iuaaa-aaaaa-qaaaa-cai trg6r-vqw4x-tcu5z-pgm4z-nmas4-ailxn-rjavv-zbzhi-jy2oy-wjrpf-hqe</td></tr>
                        <tr><td>certified_data length</td><td>32 bytes</td></tr>
                        <tr><td>canister_history_memory_usage</td><td>268 bytes</td></tr>
                    </tbody></table>
                    <h3>Execution state</h3>

                    <table>
                        <tbody><tr><td>canister_root</td><td>NOT_USED/canister_states/80000000001000020101</td></tr>
                        <tr><td>wasm_binary size</td><td>362850 bytes</td></tr>
                        <tr><td>wasm_binary sha256</td><td>3c86d912ead6de7133b9f787df4ca9feee07bea8835d3ed594b47ee89e6cb730</td></tr>
                        <tr><td>heap_size</td><td>71 pages</td></tr>
                        <tr><td>stable_memory_size</td><td>0 pages</td></tr>
                        <tr><td>exports</td><td>

                              ExportedFunctions { exported_functions: {Update("authorize"), Update("clear"), Update("commit_batch"), Update("commit_proposed_batch"), Update("compute_evidence"), Update("configure"), Update("create_asset"), Update("create_batch"), Update("create_chunk"), Update("deauthorize"), Update("delete_asset"), Update("delete_batch"), Update("get_configuration"), Update("grant_permission"), Update("list_authorized"), Update("list_permitted"), Update("propose_commit_batch"), Update("revoke_permission"), Update("set_asset_content"), Update("set_asset_properties"), Update("store"), Update("take_ownership"), Update("unset_asset_content"), Update("validate_commit_proposed_batch"), Update("validate_configure"), Update("validate_grant_permission"), Update("validate_revoke_permission"), Update("validate_take_ownership"), Query("api_version"), Query("certified_tree"), Query("get"), Query("get_asset_properties"), Query("get_chunk"), Query("http_request"), Query("http_request_streaming_callback"), Query("list"), Query("retrieve"), System(CanisterInit), System(CanisterPreUpgrade), System(CanisterPostUpgrade)}, exports_heartbeat: false, exports_global_timer: false }

                        </td></tr>
                    </tbody></table>

                    <h3>Scheduler state</h3>
                    <table>
                        <tbody><tr><td>last_full_execution_round</td><td>128</td></tr>
                        <tr><td>compute_allocation</td><td>0%</td></tr>
                        <tr><td>freeze_threshold (seconds)</td><td>2592000</td></tr>
                        <tr><td>memory_usage</td><td>5023323</td></tr>
                        <tr><td>accumulated_priority</td><td>0 </td></tr>
                        <tr><td>Cycles balance</td><td>3_100_000_000_000</td></tr>
                    </tbody></table>
                </div>
            </details>
        </td>
        <td class="text">Running</td>
        <td class="number">
	    best-effort
        </td>
        <td class="number">128</td>
    </tr>

    <tr>
        <td class="text">
            <details>
                <summary>be2us-64aaa-aaaaa-qaabq-cai</summary>
                <div class="verbose">
                    <h3>System state</h3>
                    <table>
                        <tbody><tr><td>controllers</td><td>trg6r-vqw4x-tcu5z-pgm4z-nmas4-ailxn-rjavv-zbzhi-jy2oy-wjrpf-hqe</td></tr>
                        <tr><td>certified_data length</td><td>0 bytes</td></tr>
                        <tr><td>canister_history_memory_usage</td><td>238 bytes</td></tr>
                    </tbody></table>
                    <h3>Execution state</h3>

                    <table>
                        <tbody><tr><td>canister_root</td><td>NOT_USED/canister_states/80000000001000030101</td></tr>
                        <tr><td>wasm_binary size</td><td>1603770 bytes</td></tr>
                        <tr><td>wasm_binary sha256</td><td>b91e3dd381aedb002633352f8ebad03b6eee330b7e30c3d15a5657e6f428d815</td></tr>
                        <tr><td>heap_size</td><td>24 pages</td></tr>
                        <tr><td>stable_memory_size</td><td>0 pages</td></tr>
                        <tr><td>exports</td><td>

                              ExportedFunctions { exported_functions: {Query("binding"), Query("did_to_js"), Query("http_request"), Query("merge_init_args"), Query("subtype")}, exports_heartbeat: false, exports_global_timer: false }

                        </td></tr>
                    </tbody></table>

                    <h3>Scheduler state</h3>
                    <table>
                        <tbody><tr><td>last_full_execution_round</td><td>0</td></tr>
                        <tr><td>compute_allocation</td><td>0%</td></tr>
                        <tr><td>freeze_threshold (seconds)</td><td>2592000</td></tr>
                        <tr><td>memory_usage</td><td>3176904</td></tr>
                        <tr><td>accumulated_priority</td><td>0 </td></tr>
                        <tr><td>Cycles balance</td><td>100_000_000_000_000</td></tr>
                    </tbody></table>
                </div>
            </details>
        </td>
        <td class="text">Running</td>
        <td class="number">
	    best-effort
        </td>
        <td class="number">0</td>
    </tr>

</tbody></table>
</div>

</body></html>
"#;

    let parsed = ReplicaInfo::parse_from_html_dashboard(HTML);
    assert!(parsed.is_ok());
    let expected = ReplicaInfo {
        replica_version: "0.9.0".to_string(),
        subnet_type: "System".to_string(),
        total_compute_allocation: "0 %".to_string(),
        http_server_config: "Config { listen_addr: 127.0.0.1:0, port_file_path: Some(\"/Users/mnl/Library/Application Support/org.dfinity.dfx/network/local/replica-configuration/replica-1.port\"), connection_read_timeout_seconds: 1200, request_timeout_seconds: 300, http_max_concurrent_streams: 256, max_request_size_bytes: 5242880, max_delegation_certificate_size_bytes: 1048576, max_request_receive_seconds: 300, max_read_state_concurrent_requests: 100, max_status_concurrent_requests: 100, max_catch_up_package_concurrent_requests: 100, max_dashboard_concurrent_requests: 100, max_call_concurrent_requests: 50, max_query_concurrent_requests: 400, max_pprof_concurrent_requests: 5 }".into(),
        canisters: vec![CanisterInfo {
            canister_id: "bnz7o-iuaaa-aaaaa-qaaaa-cai".to_string(),
            status: "Running".to_string(),
            memory_allocation: "best-effort".to_string(),
            last_execution_round: "0".to_string(),
            controllers: "trg6r-vqw4x-tcu5z-pgm4z-nmas4-ailxn-rjavv-zbzhi-jy2oy-wjrpf-hqe"
                .to_string(),
            certified_data_length: "32 bytes".to_string(),
            canister_history_memory_usage: "238 bytes".to_string(),
            exports: Exports::default(),
            execution_state: "".to_string(),
            last_full_execution_round: "104".to_string(),
            compute_allocation: "0%".to_string(),
            freeze_threshold: "2592000".to_string(),
            memory_usage: "7345934".to_string(),
            accumulated_priority: "0".to_string(),
            cycles_balance: "93_800_000_000_000".to_string(),
        },

        CanisterInfo {
            canister_id: "bkyz2-fmaaa-aaaaa-qaaaq-cai".to_string(),
            status: "Running".to_string(),
            memory_allocation: "best-effort".to_string(),
            last_execution_round: "0".to_string(),
            controllers: "bnz7o-iuaaa-aaaaa-qaaaa-cai trg6r-vqw4x-tcu5z-pgm4z-nmas4-ailxn-rjavv-zbzhi-jy2oy-wjrpf-hqe".to_string(),
            certified_data_length: "0 bytes".to_string(),
            canister_history_memory_usage: "268 bytes".to_string(),
            execution_state: "".to_string(),
            last_full_execution_round: "0".to_string(),
            exports: Exports::default(),
            compute_allocation: "0%".to_string(),
            freeze_threshold: "2592000".to_string(),
            memory_usage: "2294162".to_string(),
            accumulated_priority: "0".to_string(),
            cycles_balance: "3_100_000_000_000".to_string(),
        },
        CanisterInfo {
            canister_id: "bd3sg-teaaa-aaaaa-qaaba-cai".to_string(),
            status: "Running".to_string(),
            memory_allocation: "best-effort".to_string(),
            last_execution_round: "0".to_string(),
            controllers: "bnz7o-iuaaa-aaaaa-qaaaa-cai trg6r-vqw4x-tcu5z-pgm4z-nmas4-ailxn-rjavv-zbzhi-jy2oy-wjrpf-hqe".to_string(),
            certified_data_length: "32 bytes".to_string(),
            canister_history_memory_usage: "268 bytes".to_string(),
            execution_state: "".to_string(),
            last_full_execution_round: "128".to_string(),
            compute_allocation: "0%".to_string(),
            exports: Exports::default(),
            freeze_threshold: "2592000".to_string(),
            memory_usage: "5023323".to_string(),
            accumulated_priority: "0".to_string(),
            cycles_balance: "3_100_000_000_000".to_string(),
        },
        CanisterInfo {
            canister_id: "be2us-64aaa-aaaaa-qaabq-cai".to_string(),
            status: "Running".to_string(),
            memory_allocation: "best-effort".to_string(),
            last_execution_round: "0".to_string(),
            controllers: "trg6r-vqw4x-tcu5z-pgm4z-nmas4-ailxn-rjavv-zbzhi-jy2oy-wjrpf-hqe".to_string(),
            certified_data_length: "0 bytes".to_string(),
            canister_history_memory_usage: "238 bytes".to_string(),
            execution_state: "".to_string(),
            exports: Exports::default(),
            last_full_execution_round: "0".to_string(),
            compute_allocation: "0%".to_string(),
            freeze_threshold: "2592000".to_string(),
            memory_usage: "3176904".to_string(),
            accumulated_priority: "0".to_string(),
            cycles_balance: "100_000_000_000_000".to_string(),
        }

        ],
    };
    assert_eq!(parsed.unwrap(), expected);
}

#[test]
fn another() {
    use pretty_assertions::assert_eq;
    const HTML: &str = r#"
<!DOCTYPE html>
<!-- saved from url=(0034)http://localhost:56668/_/dashboard -->
<html lang="en"><head><meta http-equiv="Content-Type" content="text/html; charset=UTF-8">

    <title>Internet Computer Replica Dashboard</title>
    <style>
        div {
            margin: 6px;
        }

        h3 {
            margin-block-end: 0;
        }

        .debug {
            background-color: #eef;
            font-family: monospace;
            border: 1px solid #aaf;
        }

        span.debug {
            padding: 4px;
        }

        div.debug {
            display: block;
            padding: 10px;
        }

        td, th {
            padding: 0 10px 2px 0;
            vertical-align: text-top;
        }

        .number {
            text-align: right;
        }

        .text {
            text-align: left;
        }

        .row-separator {
            background-color: #aaf;
            height: 2px;
            padding: 0px;
        }

        .verbose {
            position: absolute;
            font-family: monospace;
            background-color: #ffa;
            border: 1px solid #ff0;
            padding: 4px;
        }
    </style>
</head>
<body data-new-gr-c-s-check-loaded="14.1152.0" data-gr-ext-installed="" data-gr-ext-disabled="forever">
<h1>Internet Computer Replica Dashboard</h1>

<h2>Subnet Settings &amp; Parameters</h2>
<table>
    <tbody><tr>
        <td>Replica Version</td>
        <td class="debug">0.9.0</td>
    </tr>
    <tr>
        <td>Subnet Type</td>
        <td class="debug">Application</td>
    </tr>
    <tr>
        <td>Total Compute Allocation</td>
        <td class="debug">0 %</td>
    </tr>
</tbody></table>
<h2>Http Server Config</h2>
<div class="debug">
    <pre>Config { listen_addr: 127.0.0.1:0, port_file_path: Some("/Users/mnl/Library/Application Support/org.dfinity.dfx/network/local/replica-configuration/replica-1.port"), connection_read_timeout_seconds: 1200, request_timeout_seconds: 300, http_max_concurrent_streams: 256, max_request_size_bytes: 5242880, max_delegation_certificate_size_bytes: 1048576, max_request_receive_seconds: 300, max_read_state_concurrent_requests: 100, max_status_concurrent_requests: 100, max_catch_up_package_concurrent_requests: 100, max_dashboard_concurrent_requests: 100, max_call_concurrent_requests: 50, max_query_concurrent_requests: 400, max_pprof_concurrent_requests: 5 }</pre>
</div>
<h2>Canisters</h2>
<div>Info at height <span class="debug">3662</span></div>
<div class="debug">
<table>
    <tbody><tr>
        <th class="text">Canister id</th>
        <th class="text">Status</th>
        <th class="number">Memory allocation</th>
        <th class="number">Last Execution Round</th>
    </tr>
    <tr class="row-separator">
        <td colspan="100%"></td>
    </tr>

    <tr>
        <td class="text">
            <details>
                <summary>bnz7o-iuaaa-aaaaa-qaaaa-cai</summary>
                <div class="verbose">
                    <h3>System state</h3>
                    <table>
                        <tbody><tr><td>controllers</td><td>trg6r-vqw4x-tcu5z-pgm4z-nmas4-ailxn-rjavv-zbzhi-jy2oy-wjrpf-hqe</td></tr>
                        <tr><td>certified_data length</td><td>32 bytes</td></tr>
                        <tr><td>canister_history_memory_usage</td><td>238 bytes</td></tr>
                    </tbody></table>
                    <h3>Execution state</h3>

                    <table>
                        <tbody><tr><td>canister_root</td><td>NOT_USED/canister_states/80000000001000000101</td></tr>
                        <tr><td>wasm_binary size</td><td>1775104 bytes</td></tr>
                        <tr><td>wasm_binary sha256</td><td>c1290ad65e6c9f840928637ed7672b688216a9c1e919eacbacc22af8c904a5e3</td></tr>
                        <tr><td>heap_size</td><td>85 pages</td></tr>
                        <tr><td>stable_memory_size</td><td>0 pages</td></tr>
                        <tr><td>exports</td><td>


                              ExportedFunctions { exported_functions: {Update("add_address"), Update("add_controller"), Update("authorize"), Update("deauthorize"), Update("remove_address"), Update("remove_controller"), Update("set_name"), Update("set_short_name"), Update("wallet_call"), Update("wallet_call128"), Update("wallet_create_canister"), Update("wallet_create_canister128"), Update("wallet_create_wallet"), Update("wallet_create_wallet128"), Update("wallet_receive"), Update("wallet_send"), Update("wallet_send128"), Update("wallet_store_wallet_wasm"), Query("get_chart"), Query("get_controllers"), Query("get_custodians"), Query("get_events"), Query("get_events128"), Query("get_managed_canister_events"), Query("get_managed_canister_events128"), Query("http_request"), Query("list_addresses"), Query("list_managed_canisters"), Query("name"), Query("wallet_api_version"), Query("wallet_balance"), Query("wallet_balance128"), System(CanisterInit), System(CanisterPreUpgrade), System(CanisterPostUpgrade)}, exports_heartbeat: false, exports_global_timer: false }

                        </td></tr>
                    </tbody></table>

                    <h3>Scheduler state</h3>
                    <table>
                        <tbody><tr><td>last_full_execution_round</td><td>1736</td></tr>
                        <tr><td>compute_allocation</td><td>0%</td></tr>
                        <tr><td>freeze_threshold (seconds)</td><td>2592000</td></tr>
                        <tr><td>memory_usage</td><td>7345934</td></tr>
                        <tr><td>accumulated_priority</td><td>0 </td></tr>
                        <tr><td>Cycles balance</td><td>93_799_340_083_699</td></tr>
                    </tbody></table>
                </div>
            </details>
        </td>
        <td class="text">Running</td>
        <td class="number">
	    best-effort
        </td>
        <td class="number">1736</td>
    </tr>

    <tr>
        <td class="text">
            <details>
                <summary>bkyz2-fmaaa-aaaaa-qaaaq-cai</summary>
                <div class="verbose">
                    <h3>System state</h3>
                    <table>
                        <tbody><tr><td>controllers</td><td>bnz7o-iuaaa-aaaaa-qaaaa-cai trg6r-vqw4x-tcu5z-pgm4z-nmas4-ailxn-rjavv-zbzhi-jy2oy-wjrpf-hqe</td></tr>
                        <tr><td>certified_data length</td><td>0 bytes</td></tr>
                        <tr><td>canister_history_memory_usage</td><td>268 bytes</td></tr>
                    </tbody></table>
                    <h3>Execution state</h3>

                    <table>
                        <tbody><tr><td>canister_root</td><td>NOT_USED/canister_states/80000000001000010101</td></tr>
                        <tr><td>wasm_binary size</td><td>130977 bytes</td></tr>
                        <tr><td>wasm_binary sha256</td><td>2aae403ef7bc3133007567ecb7f7335ebc68d331b28f67e4dde72721f115e5bf</td></tr>
                        <tr><td>heap_size</td><td>33 pages</td></tr>
                        <tr><td>stable_memory_size</td><td>0 pages</td></tr>
                        <tr><td>exports</td><td>


                              ExportedFunctions { exported_functions: {Update("__motoko_async_helper"), Update("__motoko_gc_trigger"), Query("__get_candid_interface_tmp_hack"), Query("__motoko_stable_var_info"), Query("greet"), System(CanisterStart), System(CanisterInit), System(CanisterPreUpgrade), System(CanisterPostUpgrade), System(CanisterGlobalTimer)}, exports_heartbeat: false, exports_global_timer: true }

                        </td></tr>
                    </tbody></table>

                    <h3>Scheduler state</h3>
                    <table>
                        <tbody><tr><td>last_full_execution_round</td><td>0</td></tr>
                        <tr><td>compute_allocation</td><td>0%</td></tr>
                        <tr><td>freeze_threshold (seconds)</td><td>2592000</td></tr>
                        <tr><td>memory_usage</td><td>2294283</td></tr>
                        <tr><td>accumulated_priority</td><td>0 </td></tr>
                        <tr><td>Cycles balance</td><td>3_092_279_190_900</td></tr>
                    </tbody></table>
                </div>
            </details>
        </td>
        <td class="text">Running</td>
        <td class="number">
	    best-effort
        </td>
        <td class="number">0</td>
    </tr>

    <tr>
        <td class="text">
            <details>
                <summary>bd3sg-teaaa-aaaaa-qaaba-cai</summary>
                <div class="verbose">
                    <h3>System state</h3>
                    <table>
                        <tbody><tr><td>controllers</td><td>bnz7o-iuaaa-aaaaa-qaaaa-cai trg6r-vqw4x-tcu5z-pgm4z-nmas4-ailxn-rjavv-zbzhi-jy2oy-wjrpf-hqe</td></tr>
                        <tr><td>certified_data length</td><td>32 bytes</td></tr>
                        <tr><td>canister_history_memory_usage</td><td>268 bytes</td></tr>
                    </tbody></table>
                    <h3>Execution state</h3>

                    <table>
                        <tbody><tr><td>canister_root</td><td>NOT_USED/canister_states/80000000001000020101</td></tr>
                        <tr><td>wasm_binary size</td><td>362850 bytes</td></tr>
                        <tr><td>wasm_binary sha256</td><td>3c86d912ead6de7133b9f787df4ca9feee07bea8835d3ed594b47ee89e6cb730</td></tr>
                        <tr><td>heap_size</td><td>77 pages</td></tr>
                        <tr><td>stable_memory_size</td><td>0 pages</td></tr>
                        <tr><td>exports</td><td>


                              ExportedFunctions { exported_functions: {Update("authorize"), Update("clear"), Update("commit_batch"), Update("commit_proposed_batch"), Update("compute_evidence"), Update("configure"), Update("create_asset"), Update("create_batch"), Update("create_chunk"), Update("deauthorize"), Update("delete_asset"), Update("delete_batch"), Update("get_configuration"), Update("grant_permission"), Update("list_authorized"), Update("list_permitted"), Update("propose_commit_batch"), Update("revoke_permission"), Update("set_asset_content"), Update("set_asset_properties"), Update("store"), Update("take_ownership"), Update("unset_asset_content"), Update("validate_commit_proposed_batch"), Update("validate_configure"), Update("validate_grant_permission"), Update("validate_revoke_permission"), Update("validate_take_ownership"), Query("api_version"), Query("certified_tree"), Query("get"), Query("get_asset_properties"), Query("get_chunk"), Query("http_request"), Query("http_request_streaming_callback"), Query("list"), Query("retrieve"), System(CanisterInit), System(CanisterPreUpgrade), System(CanisterPostUpgrade)}, exports_heartbeat: false, exports_global_timer: false }

                        </td></tr>
                    </tbody></table>

                    <h3>Scheduler state</h3>
                    <table>
                        <tbody><tr><td>last_full_execution_round</td><td>1761</td></tr>
                        <tr><td>compute_allocation</td><td>0%</td></tr>
                        <tr><td>freeze_threshold (seconds)</td><td>2592000</td></tr>
                        <tr><td>memory_usage</td><td>5416539</td></tr>
                        <tr><td>accumulated_priority</td><td>0 </td></tr>
                        <tr><td>Cycles balance</td><td>3_091_813_676_556</td></tr>
                    </tbody></table>
                </div>
            </details>
        </td>
        <td class="text">Running</td>
        <td class="number">
	    best-effort
        </td>
        <td class="number">1761</td>
    </tr>

    <tr>
        <td class="text">
            <details>
                <summary>be2us-64aaa-aaaaa-qaabq-cai</summary>
                <div class="verbose">
                    <h3>System state</h3>
                    <table>
                        <tbody><tr><td>controllers</td><td>trg6r-vqw4x-tcu5z-pgm4z-nmas4-ailxn-rjavv-zbzhi-jy2oy-wjrpf-hqe</td></tr>
                        <tr><td>certified_data length</td><td>0 bytes</td></tr>
                        <tr><td>canister_history_memory_usage</td><td>238 bytes</td></tr>
                    </tbody></table>
                    <h3>Execution state</h3>

                    <table>
                        <tbody><tr><td>canister_root</td><td>NOT_USED/canister_states/80000000001000030101</td></tr>
                        <tr><td>wasm_binary size</td><td>1603770 bytes</td></tr>
                        <tr><td>wasm_binary sha256</td><td>b91e3dd381aedb002633352f8ebad03b6eee330b7e30c3d15a5657e6f428d815</td></tr>
                        <tr><td>heap_size</td><td>24 pages</td></tr>
                        <tr><td>stable_memory_size</td><td>0 pages</td></tr>
                        <tr><td>exports</td><td>


                              ExportedFunctions { exported_functions: {Query("binding"), Query("did_to_js"), Query("http_request"), Query("merge_init_args"), Query("subtype")}, exports_heartbeat: false, exports_global_timer: false }

                        </td></tr>
                    </tbody></table>

                    <h3>Scheduler state</h3>
                    <table>
                        <tbody><tr><td>last_full_execution_round</td><td>0</td></tr>
                        <tr><td>compute_allocation</td><td>0%</td></tr>
                        <tr><td>freeze_threshold (seconds)</td><td>2592000</td></tr>
                        <tr><td>memory_usage</td><td>3176904</td></tr>
                        <tr><td>accumulated_priority</td><td>0 </td></tr>
                        <tr><td>Cycles balance</td><td>99_999_624_176_398</td></tr>
                    </tbody></table>
                </div>
            </details>
        </td>
        <td class="text">Running</td>
        <td class="number">
	    best-effort
        </td>
        <td class="number">0</td>
    </tr>

</tbody></table>
</div>


</body></html>
"#;

    let parsed = ReplicaInfo::parse_from_html_dashboard(HTML);
    assert!(parsed.is_ok());
    let expected = ReplicaInfo {
        replica_version: "0.9.0".to_string(),
        subnet_type: "System".to_string(),
        total_compute_allocation: "0 %".to_string(),
        http_server_config: "Config { listen_addr: 127.0.0.1:0, port_file_path: Some(\"/Users/mnl/Library/Application Support/org.dfinity.dfx/network/local/replica-configuration/replica-1.port\"), connection_read_timeout_seconds: 1200, request_timeout_seconds: 300, http_max_concurrent_streams: 256, max_request_size_bytes: 5242880, max_delegation_certificate_size_bytes: 1048576, max_request_receive_seconds: 300, max_read_state_concurrent_requests: 100, max_status_concurrent_requests: 100, max_catch_up_package_concurrent_requests: 100, max_dashboard_concurrent_requests: 100, max_call_concurrent_requests: 50, max_query_concurrent_requests: 400, max_pprof_concurrent_requests: 5 }".into(),
        canisters: vec![CanisterInfo {
            canister_id: "bnz7o-iuaaa-aaaaa-qaaaa-cai".to_string(),
            status: "Running".to_string(),
            memory_allocation: "best-effort".to_string(),
            last_execution_round: "0".to_string(),
            controllers: "trg6r-vqw4x-tcu5z-pgm4z-nmas4-ailxn-rjavv-zbzhi-jy2oy-wjrpf-hqe"
                .to_string(),
            certified_data_length: "32 bytes".to_string(),
            canister_history_memory_usage: "238 bytes".to_string(),
            execution_state: "".to_string(),
            last_full_execution_round: "104".to_string(),
            exports: Exports::default(),
            compute_allocation: "0%".to_string(),
            freeze_threshold: "2592000".to_string(),
            memory_usage: "7345934".to_string(),
            accumulated_priority: "0".to_string(),
            cycles_balance: "93_800_000_000_000".to_string(),
        },

        CanisterInfo {
            canister_id: "bkyz2-fmaaa-aaaaa-qaaaq-cai".to_string(),
            status: "Running".to_string(),
            memory_allocation: "best-effort".to_string(),
            last_execution_round: "0".to_string(),
            exports: Exports::default(),
            controllers: "bnz7o-iuaaa-aaaaa-qaaaa-cai trg6r-vqw4x-tcu5z-pgm4z-nmas4-ailxn-rjavv-zbzhi-jy2oy-wjrpf-hqe".to_string(),
            certified_data_length: "0 bytes".to_string(),
            canister_history_memory_usage: "268 bytes".to_string(),
            execution_state: "".to_string(),
            last_full_execution_round: "0".to_string(),
            compute_allocation: "0%".to_string(),
            freeze_threshold: "2592000".to_string(),
            memory_usage: "2294162".to_string(),
            accumulated_priority: "0".to_string(),
            cycles_balance: "3_100_000_000_000".to_string(),
        },
        CanisterInfo {
            canister_id: "bd3sg-teaaa-aaaaa-qaaba-cai".to_string(),
            status: "Running".to_string(),
            memory_allocation: "best-effort".to_string(),
            last_execution_round: "0".to_string(),
            controllers: "bnz7o-iuaaa-aaaaa-qaaaa-cai trg6r-vqw4x-tcu5z-pgm4z-nmas4-ailxn-rjavv-zbzhi-jy2oy-wjrpf-hqe".to_string(),
            certified_data_length: "32 bytes".to_string(),
            canister_history_memory_usage: "268 bytes".to_string(),
            execution_state: "".to_string(),
            exports: Exports::default(),
            last_full_execution_round: "128".to_string(),
            compute_allocation: "0%".to_string(),
            freeze_threshold: "2592000".to_string(),
            memory_usage: "5023323".to_string(),
            accumulated_priority: "0".to_string(),
            cycles_balance: "3_100_000_000_000".to_string(),
        },
        CanisterInfo {
            canister_id: "be2us-64aaa-aaaaa-qaabq-cai".to_string(),
            status: "Running".to_string(),
            memory_allocation: "best-effort".to_string(),
            last_execution_round: "0".to_string(),
            controllers: "trg6r-vqw4x-tcu5z-pgm4z-nmas4-ailxn-rjavv-zbzhi-jy2oy-wjrpf-hqe".to_string(),
            certified_data_length: "0 bytes".to_string(),
            exports: Exports::default(),
            canister_history_memory_usage: "238 bytes".to_string(),
            execution_state: "".to_string(),
            last_full_execution_round: "0".to_string(),
            compute_allocation: "0%".to_string(),
            freeze_threshold: "2592000".to_string(),
            memory_usage: "3176904".to_string(),
            accumulated_priority: "0".to_string(),
            cycles_balance: "100_000_000_000_000".to_string(),
        }

        ],
    };
    assert_eq!(parsed.unwrap(), expected);
}
