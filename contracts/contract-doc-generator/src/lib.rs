use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

// ---------------------------------------------------------------------------
// Types & State
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeneratorState {
    Idle,
    Discovery,
    Parsing,
    Generation,
    Complete,
    Failed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractDoc {
    pub name: String,
    pub description: Option<String>,
    pub methods: Vec<MethodDoc>,
    pub types: Vec<TypeDoc>,
    pub events: Vec<EventDoc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodDoc {
    pub name: String,
    pub description: Option<String>,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeDoc {
    pub name: String,
    pub description: Option<String>,
    pub fields: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventDoc {
    pub name: String,
    pub description: Option<String>,
}

// ---------------------------------------------------------------------------
// Generator Engine
// ---------------------------------------------------------------------------

pub struct DocGenerator {
    pub base_path: PathBuf,
    pub output_path: PathBuf,
    pub state: GeneratorState,
}

impl DocGenerator {
    pub fn new(base_path: PathBuf, output_path: PathBuf) -> Self {
        Self {
            base_path,
            output_path,
            state: GeneratorState::Idle,
        }
    }

    /// Primary routine to generate documentation for all contracts
    pub fn run(&mut self) -> Result<(), String> {
        self.state = GeneratorState::Discovery;
        println!("EVENT: Starting contract discovery in {:?}", self.base_path);
        
        let contracts = self.discover_contracts()?;
        
        self.state = GeneratorState::Parsing;
        println!("EVENT: Parsing {} contracts", contracts.len());
        
        let mut docs = Vec::new();
        for contract_path in contracts {
            match self.parse_contract(&contract_path) {
                Ok(doc) => docs.push(doc),
                Err(e) => {
                    println!("WARNING: Failed to parse {:?}: {}", contract_path, e);
                }
            }
        }

        self.state = GeneratorState::Generation;
        println!("EVENT: Finalizing Markdown generation");
        
        self.write_docs(docs)?;

        self.state = GeneratorState::Complete;
        println!("EVENT: Documentation generation successful");
        
        Ok(())
    }

    fn discover_contracts(&self) -> Result<Vec<PathBuf>, String> {
        let mut contracts = Vec::new();
        let entries = fs::read_dir(&self.base_path).map_err(|e| e.to_string())?;

        for entry in entries {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();
            if path.is_dir() {
                let cargo_toml = path.join("Cargo.toml");
                let src_lib = path.join("src").join("lib.rs");
                
                // Exclude shared crates and the generator itself
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if name == "shared" || name == "contract-doc-generator" || name == "deployment-scripts" {
                    continue;
                }

                if cargo_toml.exists() && src_lib.exists() {
                    contracts.push(path);
                }
            }
        }
        Ok(contracts)
    }

    fn parse_contract(&self, path: &Path) -> Result<ContractDoc, String> {
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("Unknown").to_string();
        let lib_path = path.join("src").join("lib.rs");
        let content = fs::read_to_string(lib_path).map_err(|e| e.to_string())?;

        let mut doc = ContractDoc {
            name: name.clone(),
            description: None,
            methods: Vec::new(),
            types: Vec::new(),
            events: Vec::new(),
        };

        let mut lines = content.lines().peekable();
        let mut current_docs = Vec::new();

        while let Some(line) = lines.next() {
            let trimmed = line.trim();
            
            if trimmed.is_empty() {
                continue;
            }

            if trimmed.starts_with("///") {
                current_docs.push(trimmed[3..].trim().to_string());
                continue;
            }

            // Skip decorators but keep current_docs
            if trimmed.starts_with("#[") {
                continue;
            }

            // Extract Contract Header
            if doc.description.is_none() && !current_docs.is_empty() && (trimmed.contains("pub struct") || trimmed.contains("impl")) {
                 doc.description = Some(current_docs.join(" "));
            }

            // Detect Methods
            if trimmed.contains("pub fn") {
                let name_regex = Regex::new(r"fn\s+([a-zA-Z0-9_]+)").unwrap();
                if let Some(cap) = name_regex.captures(trimmed) {
                    doc.methods.push(MethodDoc {
                        name: cap[1].to_string(),
                        description: if current_docs.is_empty() { None } else { Some(current_docs.join(" ")) },
                        signature: trimmed.replace("{", "").trim().to_string(),
                    });
                }
            }

            // Detect Events (marked with #[contractevent])
            // Since we skipped #[ earlier, we need to check if the NEXT line is an event struct
            // Actually, let's look back or handle decorators specifically
            if trimmed.contains("struct") {
                 if !current_docs.is_empty() {
                    let name_regex = Regex::new(r"struct\s+([a-zA-Z0-9_]+)").unwrap();
                    if let Some(cap) = name_regex.captures(trimmed) {
                        let struct_name = cap[1].to_string();
                        if struct_name.contains("Event") || struct_name.contains("Claimed") {
                            doc.events.push(EventDoc {
                                name: struct_name,
                                description: Some(current_docs.join(" ")),
                            });
                        } else {
                            doc.types.push(TypeDoc {
                                name: struct_name,
                                description: Some(current_docs.join(" ")),
                                fields: Vec::new(),
                            });
                        }
                    }
                 }
            }

            current_docs.clear();
        }

        Ok(doc)
    }

    fn write_docs(&self, docs: Vec<ContractDoc>) -> Result<(), String> {
        if !self.output_path.exists() {
            fs::create_dir_all(&self.output_path).map_err(|e| e.to_string())?;
        }

        let mut index_content = String::from("# StellarCade Contracts Reference\n\n");
        index_content.push_str("Automatic generated documentation for Soroban contracts.\n\n");

        for doc in docs {
            let file_name = format!("{}.md", doc.name);
            let file_path = self.output_path.join(&file_name);
            
            let mut content = format!("# {}\n\n", doc.name);
            if let Some(desc) = &doc.description {
                content.push_str(&format!("{}\n\n", desc));
            }

            if !doc.methods.is_empty() {
                content.push_str("## Public Methods\n\n");
                for m in &doc.methods {
                    content.push_str(&format!("### `{}`\n", m.name));
                    if let Some(d) = &m.description {
                        content.push_str(&format!("{}\n\n", d));
                    }
                    content.push_str(&format!("```rust\n{}\n```\n\n", m.signature));
                }
            }

            if !doc.events.is_empty() {
                content.push_str("## Events\n\n");
                for e in &doc.events {
                    content.push_str(&format!("- **{}**: {}\n", e.name, e.description.as_deref().unwrap_or("No description")));
                }
                content.push_str("\n");
            }

            fs::write(file_path, content).map_err(|e| e.to_string())?;
            index_content.push_str(&format!("- [{}]({})\n", doc.name, file_name));
        }

        fs::write(self.output_path.join("README.md"), index_content).map_err(|e| e.to_string())?;
        
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_parsing_logic() {
        let dir = tempdir().unwrap();
        let src_dir = dir.path().join("src");
        fs::create_dir(&src_dir).unwrap();
        let lib_rs = src_dir.join("lib.rs");
        
        let mock_code = r#"
/// The Reward Distribution contract manages lifecycle of rewards.
#[contract]
pub struct RewardContract;

#[contractimpl]
impl RewardContract {
    /// Initializes the contract.
    pub fn init(env: Env, admin: Address) { }

    /// Accrues rewards for a user.
    pub fn accrue(env: Env, user: Address) { }
}

/// Emitted when a reward is claimed.
#[contractevent]
pub struct RewardClaimed {
    pub user: Address,
    pub amount: i128,
}
"#;
        fs::write(lib_rs, mock_code).unwrap();
        fs::write(dir.path().join("Cargo.toml"), "").unwrap();

        let generator = DocGenerator::new(dir.path().to_path_buf(), dir.path().join("docs"));
        let doc = generator.parse_contract(dir.path()).unwrap();

        assert_eq!(doc.name, dir.path().file_name().unwrap().to_str().unwrap());
        assert!(doc.description.unwrap().contains("Reward Distribution"));
        assert_eq!(doc.methods.len(), 2);
        assert_eq!(doc.methods[0].name, "init");
        assert_eq!(doc.events.len(), 1);
        assert_eq!(doc.events[0].name, "RewardClaimed");
    }
}
