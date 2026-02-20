//! Pluggable policy profiles: governance rules as first-class artifacts
//!
//! Encodes neurorights, biomechanical constraints, and jurisdiction-specific
//! rules as JSON/ALN policy schemas that can be swapped at runtime.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A single neurorights constraint
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NeurorightsConstraint {
    /// Constraint name (e.g., "noDreamplexModule", "noSubconsciousTargeting")
    pub name: String,
    /// Human description
    pub description: String,
    /// Whether this constraint is currently enforced
    pub enforced: bool,
}

/// Biomechanical integration policy
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BiomechPolicy {
    /// Role classification (observer, advisor, bounded-auto, forbidden)
    pub module_scope: String,
    /// Risk class (low, medium, high, critical)
    pub risk_class: String,
    /// Max effect size (0.0–1.0)
    pub max_effect_size: f64,
    /// Max duty cycle (0.0–1.0)
    pub max_duty_cycle: f64,
    /// Max session length (minutes)
    pub max_session_minutes: u32,
    /// Deny if BCI* exceeds this threshold
    pub bci_ceiling: f64,
}

/// A complete pluggable policy profile
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PolicyProfile {
    /// Profile name (e.g., "EU_neurorights", "Chile_amendment", "Phoenix_medical")
    pub name: String,
    /// Version of the profile
    pub version: String,
    /// Active neurorights constraints
    pub neurorights_constraints: Vec<NeurorightsConstraint>,
    /// Biomechanical policies
    pub biomech_policy: BiomechPolicy,
    /// Corridor polytope references (jurisdiction-specific safe regions)
    pub corridor_polytopes: HashMap<String, Vec<f64>>,
    /// Minimum rights floor (non-derogable)
    pub minimum_rights: Vec<String>,
    /// Authority/source of this profile (e.g., "EU_AI_Act", "Chilean_Neurorights_Amendment")
    pub authority: String,
    /// Effective date (ISO 8601)
    pub effective_date: String,
    /// Optional notes
    pub notes: Option<String>,
}

impl PolicyProfile {
    /// Create a new policy profile
    pub fn new(name: String, version: String, authority: String) -> Self {
        Self {
            name,
            version,
            neurorights_constraints: Vec::new(),
            biomech_policy: BiomechPolicy {
                module_scope: "observer".to_string(),
                risk_class: "medium".to_string(),
                max_effect_size: 0.5,
                max_duty_cycle: 0.5,
                max_session_minutes: 60,
                bci_ceiling: 0.25,
            },
            corridor_polytopes: HashMap::new(),
            minimum_rights: vec![
                "right_to_consent".to_string(),
                "right_to_abort".to_string(),
                "right_to_identity".to_string(),
                "right_to_privacy".to_string(),
            ],
            authority,
            effective_date: chrono::Utc::now().to_rfc3339(),
            notes: None,
        }
    }

    /// Add a neurorights constraint
    pub fn add_neurorights_constraint(&mut self, constraint: NeurorightsConstraint) {
        self.neurorights_constraints.push(constraint);
    }

    /// Validate the profile structure
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Policy profile name cannot be empty".to_string());
        }
        if self.authority.is_empty() {
            return Err("Authority must be specified".to_string());
        }
        if self.biomech_policy.bci_ceiling < 0.0 || self.biomech_policy.bci_ceiling > 1.0 {
            return Err("BCI ceiling must be in [0.0, 1.0]".to_string());
        }
        if self.minimum_rights.is_empty() {
            return Err("Minimum rights cannot be empty".to_string());
        }
        Ok(())
    }

    /// Check if a constraint is enforced
    pub fn is_constraint_enforced(&self, constraint_name: &str) -> bool {
        self.neurorights_constraints
            .iter()
            .find(|c| c.name == constraint_name)
            .map(|c| c.enforced)
            .unwrap_or(false)
    }
}

/// Predefined policy profiles for common jurisdictions
impl PolicyProfile {
    /// EU neurorights policy profile (GDPR-aligned)
    pub fn eu_neurorights() -> Self {
        let mut profile = PolicyProfile::new(
            "EU_neurorights".to_string(),
            "1.0".to_string(),
            "EU_AI_Act".to_string(),
        );
        profile.add_neurorights_constraint(NeurorightsConstraint {
            name: "noSubconsciousTargeting".to_string(),
            description: "Prohibit targeting subconscious neural processes".to_string(),
            enforced: true,
        });
        profile.add_neurorights_constraint(NeurorightsConstraint {
            name: "noInnerStateGovernance".to_string(),
            description: "Prohibit using inner-state biomarkers for governance".to_string(),
            enforced: true,
        });
        profile.biomech_policy.bci_ceiling = 0.20;
        profile
    }

    /// Chile neurorights amendment profile
    pub fn chile_neurorights() -> Self {
        let mut profile = PolicyProfile::new(
            "Chile_neurorights".to_string(),
            "1.0".to_string(),
            "Chilean_Constitutional_Amendment".to_string(),
        );
        profile.add_neurorights_constraint(NeurorightsConstraint {
            name: "mentalPrivacy".to_string(),
            description: "Protect mental privacy and freedom of thought".to_string(),
            enforced: true,
        });
        profile.add_neurorights_constraint(NeurorightsConstraint {
            name: "psych_integrity".to_string(),
            description: "Protect psychological integrity".to_string(),
            enforced: true,
        });
        profile.biomech_policy.bci_ceiling = 0.25;
        profile
    }

    /// Phoenix medical corridor profile
    pub fn phoenix_medical() -> Self {
        let mut profile = PolicyProfile::new(
            "Phoenix_medical".to_string(),
            "1.0".to_string(),
            "Phoenix_Medical_Authority".to_string(),
        );
        profile.biomech_policy.module_scope = "bounded-auto".to_string();
        profile.biomech_policy.risk_class = "high".to_string();
        profile.biomech_policy.max_session_minutes = 120;
        profile
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_policy_profile_creation() {
        let profile = PolicyProfile::new("test".to_string(), "1.0".to_string(), "test_auth".to_string());
        assert!(profile.validate().is_ok());
    }

    #[test]
    fn test_eu_profile() {
        let profile = PolicyProfile::eu_neurorights();
        assert_eq!(profile.biomech_policy.bci_ceiling, 0.20);
        assert!(profile.is_constraint_enforced("noSubconsciousTargeting"));
    }

    #[test]
    fn test_policy_constraint() {
        let mut profile = PolicyProfile::new("test".to_string(), "1.0".to_string(), "test".to_string());
        let constraint = NeurorightsConstraint {
            name: "test_constraint".to_string(),
            description: "A test constraint".to_string(),
            enforced: true,
        };
        profile.add_neurorights_constraint(constraint);
        assert!(profile.is_constraint_enforced("test_constraint"));
    }
}
