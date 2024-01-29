use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf};

// Structure matching the "canister_ids.json" file
// ```
// /path/to/project/.dfx/local/canister_ids.json
// {
//   "__Candid_UI": {
//     "local": "be2us-64aaa-aaaaa-qaabq-cai",
//     "ic": "be2us-64aaa-aaaaa-qaabq-cai"
//   },
//   "fff_backend": {
//     "local": "be2us-64aaa-aaaaa-qaabq-cai",
//     "ic": "be2us-64aaa-aaaaa-qaabq-cai"
//   },
//   "fff_frontend": {
//     "local": "be2us-64aaa-aaaaa-qaabq-cai",
//     "ic": "be2us-64aaa-aaaaa-qaabq-cai"
//   }
// }
// ```
type CanisterName = String;
// type ProjectName = String;
type Network = String;
type CanisterId = String;
type CanisterIds = HashMap<CanisterName, HashMap<Network, CanisterId>>;

// Structure for relevant parts of "dfx.json"
// ```
// /path/to/project/dfx.json
// {
//   "canisters": {
//     "fff_backend": {
//       "main": "src/fff_backend/main.mo",
//       "type": "motoko"
//     },
//     "fff_frontend": {
//       "dependencies": [
//         "fff_backend"
//       ],
//       "frontend": {
//         "entrypoint": "src/fff_frontend/src/index.html"
//       },
//       "source": [
//         "src/fff_frontend/assets",
//         "dist/fff_frontend/"
//       ],
//       "type": "assets"
//     }
//   },
//   "defaults": {
//     "build": {
//       "args": "",
//       "packtool": ""
//     }
//   },
//   "output_env_file": ".env",
//   "version": 1
// }
// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DfxProject {
    pub canisters: HashMap<CanisterName, CanisterInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CanisterInfo {
    pub dependencies: Option<Vec<String>>,
    pub frontend: Option<HashMap<String, String>>,
    pub source: Option<Vec<String>>,
    #[serde(rename = "type")]
    pub canister_type: String,
    pub main: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct ProjectDatabase {
    pub projects: HashMap<PathBuf, DfxProjectData>,
    pub db_path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DfxProjectData {
    pub canisters: HashMap<CanisterName, CanisterInfo>,
    pub canister_ids: CanisterIds,
}

impl ProjectDatabase {
    pub fn init(path: &PathBuf) -> Result<()> {
        if !path.exists() {
            std::fs::create_dir_all(&path.parent().unwrap())?;
        }
        Ok(())
    }
    // Function to load the database from a JSON file
    pub fn load(path: &PathBuf) -> Result<ProjectDatabase> {
        if path.exists() {
            let content =
                fs::read_to_string(path).with_context(|| format!("Failed to read {:?}", path))?;
            let db = serde_json::from_str(&content)
                .with_context(|| "Failed to parse project database JSON")?;
            Ok(db)
        } else {
            let db = ProjectDatabase {
                db_path: path.clone(),
                ..Default::default()
            };
            db.save()?;
            Ok(db)
        }
    }

    // Function to save the database to a JSON file
    pub fn save(&self) -> Result<()> {
        let serialized = serde_json::to_string_pretty(self)
            .with_context(|| "Failed to serialize project database")?;
        fs::write(&self.db_path, serialized)
            .with_context(|| format!("Failed to write project database to {:?}", self.db_path))?;
        Ok(())
    }

    pub fn add_project(&mut self, project_path: PathBuf, project_data: DfxProjectData) {
        self.projects.insert(project_path, project_data);
    }

    pub fn refresh(&mut self) -> Result<()> {
        for (project_path, project_data) in self.projects.iter_mut() {
            let mut dfx_project_path = project_path.clone();
            dfx_project_path.push("dfx.json");
            let dfx_json_content = fs::read_to_string(&dfx_project_path)
                .with_context(|| format!("Failed to read {:?}", dfx_project_path))?;
            let dfx_project: DfxProject = serde_json::from_str(&dfx_json_content)
                .with_context(|| "Failed to parse dfx.json")?;
            project_data.canisters = dfx_project.canisters;
            let mut canister_ids_path = project_path.clone();
            canister_ids_path.push(".dfx/local/canister_ids.json");
            if canister_ids_path.exists() {
                let canister_ids_content = fs::read_to_string(&canister_ids_path)
                    .with_context(|| format!("Failed to read {:?}", canister_ids_path))?;
                let canister_ids: CanisterIds = serde_json::from_str(&canister_ids_content)
                    .with_context(|| "Failed to parse canister_ids.json")?;
                project_data.canister_ids = canister_ids;
            } else {
                anyhow::bail!("Canister IDs file not found ({:?}).", canister_ids_path);
            }
        }
        Ok(())
    }

    pub fn register_dfx_project(path_to_dfx_project: PathBuf, db_path: PathBuf) -> Result<()> {
        let mut dfx_project_path = path_to_dfx_project.clone();
        dfx_project_path.push("dfx.json");
        if !dfx_project_path.exists() {
            anyhow::bail!("The path you provided is not a dfx project (dfx.json not found).");
        }

        let dfx_json_content = fs::read_to_string(&dfx_project_path)
            .with_context(|| format!("Failed to read {:?}", dfx_project_path))?;
        let dfx_project: DfxProject =
            serde_json::from_str(&dfx_json_content).with_context(|| "Failed to parse dfx.json")?;

        let mut canister_ids_path = path_to_dfx_project.clone();
        canister_ids_path.push(".dfx/local/canister_ids.json");

        let canister_ids: CanisterIds = if canister_ids_path.exists() {
            let canister_ids_content = fs::read_to_string(&canister_ids_path)
                .with_context(|| format!("Failed to read {:?}", canister_ids_path))?;
            serde_json::from_str(&canister_ids_content)
                .with_context(|| "Failed to parse canister_ids.json")?
        } else {
            anyhow::bail!("Canister IDs file not found ({:?}).", canister_ids_path);
        };

        let mut project_db = ProjectDatabase::load(&db_path)?;
        let project_data = DfxProjectData {
            canisters: dfx_project.canisters,
            canister_ids,
        };

        project_db.add_project(path_to_dfx_project, project_data);
        project_db.save()?;
        println!(
            "Project '{}' has been added to the database.",
            dfx_project_path.display()
        );
        Ok(())
    }
    pub fn get_info(
        &self,
        canister_id: &str,
        network: &str,
    ) -> Option<(String, CanisterName, CanisterInfo)> {
        // dbg!(&canister_id, &network);
        for (project_path, project) in self.projects.iter() {
            for (canister_name, ids) in project.canister_ids.iter() {
                if ids.get(network).unwrap_or(&"".to_string()) == canister_id {
                    if let Some(canister_info) = project.canisters.get(canister_name) {
                        return Some((
                            project_path.to_str().unwrap().to_string(),
                            canister_name.clone(),
                            canister_info.clone(),
                        ));
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_getting_info() {
        let db_data = r#"{
  "projects": {
    "/Users/mnl/org/pkms/fff": {
      "canisters": {
        "fff_frontend": {
          "dependencies": [
            "fff_backend"
          ],
          "frontend": {
            "entrypoint": "src/fff_frontend/src/index.html"
          },
          "source": [
            "src/fff_frontend/assets",
            "dist/fff_frontend/"
          ],
          "type": "assets"
        },
        "fff_backend": {
          "main": "src/fff_backend/main.mo",
          "type": "motoko"
        }
      },
      "canister_ids": {
        "__Candid_UI": {
          "local": "be2us-64aaa-aaaaa-qaabq-cai"
        },
        "fff_backend": {
          "local": "bkyz2-fmaaa-aaaaa-qaaaq-cai"
        },
        "fff_frontend": {
          "local": "bd3sg-teaaa-aaaaa-qaaba-cai"
        }
      }
    }
  },
  "db_path": "/Users/mnl/.cache/dfinity/versions/0.16.0/extensions/top/dfx_projects_database.json"
}
"#;
        let db: super::ProjectDatabase = serde_json::from_str(db_data).unwrap();
        let info = db.get_info("bkyz2-fmaaa-aaaaa-qaaaq-cai", "local").unwrap();
        assert_eq!(info.0, "/Users/mnl/org/pkms/fff");
        assert_eq!(info.1, "fff_backend");
        assert_eq!(info.2.canister_type, "motoko");
    }
}
