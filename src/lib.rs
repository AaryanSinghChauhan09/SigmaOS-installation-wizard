// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-installation-wizard: installation wizard component model.
//!
//! Real implementation: wizards, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Wizard {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstallationWizardError {
    EmptyWizardName,
    DuplicateWizard(String),
    EmptyEventSubject,
}

impl fmt::Display for InstallationWizardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyWizardName => write!(f, "wizard name must not be empty"),
            Self::DuplicateWizard(id) => write!(f, "duplicate wizard {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for InstallationWizardError {}

pub struct InstallationWizard {
    pub name: String,
    wizards: BTreeMap<String, Wizard>,
    events: BTreeMap<String, Event>,
}

impl InstallationWizard {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            wizards: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_wizard(&mut self, item: Wizard) -> Result<(), InstallationWizardError> {
        if item.name.trim().is_empty() {
            return Err(InstallationWizardError::EmptyWizardName);
        }
        let id = item.id.clone();
        if self.wizards.contains_key(&id) {
            return Err(InstallationWizardError::DuplicateWizard(id.clone()));
        }
        self.wizards.insert(id, item);
        Ok(())
    }

    pub fn wizard_count(&self) -> usize {
        self.wizards.len()
    }

    pub fn get_wizard(&self, id: &str) -> Option<&Wizard> {
        self.wizards.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), InstallationWizardError> {
        if event.subject.trim().is_empty() {
            return Err(InstallationWizardError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_wizard(id: &str, name: &str) -> Wizard {
        Wizard {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = InstallationWizard::new("InstallationWizard");
        assert_eq!(m.name, "InstallationWizard");
        assert_eq!(m.wizard_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_wizard() {
        let mut m = InstallationWizard::new("InstallationWizard");
        m.add_wizard(sample_wizard("a1", "Alpha")).unwrap();
        assert_eq!(m.wizard_count(), 1);
        assert!(m.get_wizard("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = InstallationWizard::new("InstallationWizard");
        assert!(m.add_wizard(sample_wizard("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = InstallationWizard::new("InstallationWizard");
        m.add_wizard(sample_wizard("x", "One")).unwrap();
        assert!(m.add_wizard(sample_wizard("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = InstallationWizard::new("InstallationWizard");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
