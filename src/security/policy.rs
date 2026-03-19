use anyhow::Result;
use std::collections::HashSet;

use crate::models::{PipelineStep, SecurityPolicy};

pub struct PolicyValidator {
    pub forbidden_commands: HashSet<String>,
    allowed_dirs: Vec<String>,
    allow_sudo: bool,
}

impl PolicyValidator {
    pub fn new(global_policy: &SecurityPolicy) -> Self {
        Self {
            forbidden_commands: global_policy
                .forbidden_cmds
                .iter()
                .cloned()
                .collect(),
            allowed_dirs: global_policy.allowed_dirs.clone(),
            allow_sudo: global_policy.allow_sudo,
        }
    }
    
    pub fn validate(&self, step: &PipelineStep) -> Result<()> {
        self.validate_command(&step.cmd)?;
        self.validate_working_dir(&step.working_dir)?;
        Ok(())
    }
    
    fn validate_command(&self, cmd: &str) -> Result<()> {
        if let Some(first_word) = cmd.split_whitespace().next() {
            if first_word == "sudo" && !self.allow_sudo {
                anyhow::bail!("Sudo is not allowed: {}", first_word);
            }
            if self.forbidden_commands.contains(first_word) {
                anyhow::bail!("Forbidden command: {}", first_word);
            }
        }
        Ok(())
    }
    
    fn validate_working_dir(&self, dir: &Option<String>) -> Result<()> {
        if let Some(dir_path) = dir {
            if !self.allowed_dirs.is_empty() {
                let normalized = if dir_path.starts_with('/') {
                    dir_path.clone()
                } else {
                    format!("./{}", dir_path)
                };
                
                let is_allowed = self.allowed_dirs.iter().any(|allowed| {
                    normalized.starts_with(allowed) || normalized.starts_with("./") && normalized.starts_with(&format!("{allowed}"))
                });
                
                if !is_allowed {
                    anyhow::bail!("Directory not in allowed list: {}", dir_path);
                }
            }
        }
        Ok(())
    }
}

impl Default for PolicyValidator {
    fn default() -> Self {
        Self::new(&SecurityPolicy::default())
    }
}
