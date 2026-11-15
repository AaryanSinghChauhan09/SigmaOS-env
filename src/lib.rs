//! SigmaOS-env: environment variables model.

use std::collections::BTreeMap;
use std::fmt;

/// Primary domain entity for environment variables.
pub struct EnvVar {
    pub id: String,
    pub name: String,
    pub key: String,
    pub enabled: bool,
}

/// Secondary domain entity for environment variables.
pub struct EnvScope {
    pub id: String,
    pub scope_name: String,
    pub description: String,
}

/// Errors for the environment variables domain.
#[derive(Debug, PartialEq)]
pub enum EnvError {
    EmptyName,
    Duplicate(String),
    NotFound(String),
}

impl fmt::Display for EnvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "name cannot be empty"),
            Self::Duplicate(id) => write!(f, "duplicate id: {id}"),
            Self::NotFound(id) => write!(f, "not found: {id}"),
        }
    }
}

impl std::error::Error for EnvError {}

/// Registry and manager for environment variables.
pub struct EnvManager {
    pub name: String,
    items: BTreeMap<String, EnvVar>,
    configs: BTreeMap<String, EnvScope>,
}

impl EnvManager {
    /// Create a new manager instance.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            items: BTreeMap::new(),
            configs: BTreeMap::new(),
        }
    }

    /// Add a primary item with fail-fast validation.
    pub fn add_item(&mut self, item: EnvVar) -> Result<(), EnvError> {
        if item.name.is_empty() {
            return Err(EnvError::EmptyName);
        }
        if self.items.contains_key(&item.id) {
            return Err(EnvError::Duplicate(item.id.clone()));
        }
        self.items.insert(item.id.clone(), item);
        Ok(())
    }

    /// Retrieve a primary item by id.
    pub fn get_item(&self, id: &str) -> Option<&EnvVar> {
        self.items.get(id)
    }

    /// Count of primary items.
    pub fn count(&self) -> usize {
        self.items.len()
    }

    /// Add a secondary configuration entry.
    pub fn add_config(&mut self, cfg: EnvScope) {
        self.configs.insert(cfg.id.clone(), cfg);
    }

    /// Count of configuration entries.
    pub fn config_count(&self) -> usize {
        self.configs.len()
    }

    /// Enable or disable an item.
    pub fn set_enabled(&mut self, id: &str, enabled: bool) -> Result<(), EnvError> {
        self.items
            .get_mut(id)
            .map(|item| item.enabled = enabled)
            .ok_or_else(|| EnvError::NotFound(id.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_EnvVar(id: &str, name: &str) -> EnvVar {
        EnvVar {
            id: id.to_string(),
            name: name.to_string(),
            key: "env-default".to_string(),
            enabled: true,
        }
    }

    fn make_EnvScope(id: &str, val: &str) -> EnvScope {
        EnvScope {
            id: id.to_string(),
            scope_name: val.to_string(),
            description: format!("{} config", id),
        }
    }

    #[test]
    fn test_create_aaa() {
        let mgr = EnvManager::new("test");
        assert_eq!(mgr.name, "test");
        assert_eq!(mgr.count(), 0);
        assert_eq!(mgr.config_count(), 0);
    }

    #[test]
    fn test_add_item_aaa() {
        let mut mgr = EnvManager::new("test");
        let item = make_EnvVar("item1", "First Item");
        assert!(mgr.add_item(item).is_ok());
        assert_eq!(mgr.count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut mgr = EnvManager::new("test");
        let item = make_EnvVar("item1", "");
        assert_eq!(mgr.add_item(item), Err(EnvError::EmptyName));
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut mgr = EnvManager::new("test");
        let item1 = make_EnvVar("item1", "First");
        let item2 = make_EnvVar("item1", "Duplicate");
        mgr.add_item(item1).unwrap();
        assert_eq!(mgr.add_item(item2), Err(EnvError::Duplicate("item1".to_string())));
    }

    #[test]
    fn test_config_addition_aaa() {
        let mut mgr = EnvManager::new("test");
        let cfg = make_EnvScope("cfg1", "value1");
        mgr.add_config(cfg);
        assert_eq!(mgr.config_count(), 1);
    }

    #[test]
    fn test_get_item_aaa() {
        let mut mgr = EnvManager::new("test");
        let item = make_EnvVar("item1", "First Item");
        mgr.add_item(item).unwrap();
        assert!(mgr.get_item("item1").is_some());
        assert!(mgr.get_item("none").is_none());
    }

    #[test]
    fn test_set_enabled_aaa() {
        let mut mgr = EnvManager::new("test");
        let item = make_EnvVar("item1", "First Item");
        mgr.add_item(item).unwrap();
        assert!(mgr.set_enabled("item1", false).is_ok());
        assert!(!mgr.get_item("item1").unwrap().enabled);
        assert_eq!(mgr.set_enabled("none", true), Err(EnvError::NotFound("none".to_string())));
    }
}
