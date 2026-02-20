use crate::types::guards::{GuardDecision, SafetyGuard};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MicrospaceState {
    pub microspace_id: String,
    pub occupant_organism: String,
    pub volume_mm3: f64,
    pub current_swarm_volume_mm3: f64,
    pub ecosystem_role: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SwarmActivityProposal {
    pub target_microspace_id: String,
    pub proposed_energy_draw_mw: f64,
    pub proposed_duration_secs: u64,
    pub activity_type: String,
}

pub struct MicrospaceIntegrityGuard {
    /// Max density for each organism type (evidence-backed, %)
    pub density_ceilings: std::collections::HashMap<String, f64>,
    /// Max activity power for each ecosystem role (mW)
    pub activity_power_limits: std::collections::HashMap<String, f64>,
    /// Max occupancy time for each organism (secs)
    pub occupancy_limits: std::collections::HashMap<String, u64>,
}

impl MicrospaceIntegrityGuard {
    pub fn new() -> Self {
        let mut density_ceilings = std::collections::HashMap::new();
        density_ceilings.insert("soil_rhizosphere".to_string(), 0.5);
        density_ceilings.insert("insect_thorax".to_string(), 0.1);
        density_ceilings.insert("coral_zooxanthella".to_string(), 0.05);
        density_ceilings.insert("neural_tissue".to_string(), 0.01);

        let mut activity_power_limits = std::collections::HashMap::new();
        activity_power_limits.insert("nutrient_cycling".to_string(), 10.0);
        activity_power_limits.insert("flight_metabolic".to_string(), 1.0);
        activity_power_limits.insert("photosynthesis".to_string(), 0.5);

        let mut occupancy_limits = std::collections::HashMap::new();
        occupancy_limits.insert("soil_rhizosphere".to_string(), 3600); // 60 min
        occupancy_limits.insert("insect_thorax".to_string(), 1800); // 30 min
        occupancy_limits.insert("coral_zooxanthella".to_string(), 14400); // 4 hrs
        occupancy_limits.insert("neural_tissue".to_string(), 7200); // 2 hrs

        Self {
            density_ceilings,
            activity_power_limits,
            occupancy_limits,
        }
    }

    pub fn evaluate_density(&self, state: &MicrospaceState) -> GuardDecision {
        let ceiling = self
            .density_ceilings
            .get(&state.occupant_organism)
            .copied()
            .unwrap_or(0.1);

        let current_density_pct = (state.current_swarm_volume_mm3 / state.volume_mm3) * 100.0;

        if current_density_pct > ceiling {
            GuardDecision::Forbid(format!(
                "Microspace {} density {:.2}% exceeds ceiling {:.2}%",
                state.microspace_id, current_density_pct, ceiling
            ))
        } else if current_density_pct > ceiling * 0.8 {
            GuardDecision::DegradePrecision(format!(
                "Microspace {} density approaching ceiling {:.2}%/{:.2}%",
                state.microspace_id, current_density_pct, ceiling
            ))
        } else {
            GuardDecision::AllowFull
        }
    }

    pub fn evaluate_activity(
        &self,
        state: &MicrospaceState,
        proposal: &SwarmActivityProposal,
    ) -> GuardDecision {
        let limit = self
            .activity_power_limits
            .get(&state.ecosystem_role)
            .copied()
            .unwrap_or(1.0);

        if proposal.proposed_energy_draw_mw > limit {
            GuardDecision::Forbid(format!(
                "Activity {} draws {:.2} mW exceeds limit {:.2} mW for {}",
                proposal.activity_type,
                proposal.proposed_energy_draw_mw,
                limit,
                state.ecosystem_role
            ))
        } else if proposal.proposed_energy_draw_mw > limit * 0.7 {
            GuardDecision::PauseAndRest(format!(
                "Activity power approaching limit; consider reducing duty cycle"
            ))
        } else {
            GuardDecision::AllowFull
        }
    }

    pub fn evaluate_duration(
        &self,
        state: &MicrospaceState,
        proposal: &SwarmActivityProposal,
    ) -> GuardDecision {
        let limit = self
            .occupancy_limits
            .get(&state.occupant_organism)
            .copied()
            .unwrap_or(3600);

        if proposal.proposed_duration_secs > limit {
            GuardDecision::Forbid(format!(
                "Proposed occupancy {} secs exceeds limit {} secs for {}",
                proposal.proposed_duration_secs, limit, state.occupant_organism
            ))
        } else if proposal.proposed_duration_secs > (limit as f64 * 0.8) as u64 {
            GuardDecision::PauseAndRest(format!(
                "Occupancy near limit; consider retreat soon"
            ))
        } else {
            GuardDecision::AllowFull
        }
    }

    pub fn evaluate_swarm_proposal(
        &self,
        state: &MicrospaceState,
        proposal: &SwarmActivityProposal,
    ) -> GuardDecision {
        // Check all three gates in sequence
        let density_check = self.evaluate_density(state);
        if matches!(density_check, GuardDecision::Forbid(_)) {
            return density_check;
        }

        let activity_check = self.evaluate_activity(state, proposal);
        if matches!(activity_check, GuardDecision::Forbid(_)) {
            return activity_check;
        }

        let duration_check = self.evaluate_duration(state, proposal);
        if matches!(duration_check, GuardDecision::Forbid(_)) {
            return duration_check;
        }

        // If any returned DegradePrecision or PauseAndRest, return that (caution)
        if matches!(density_check, GuardDecision::DegradePrecision(_)) {
            return density_check;
        }
        if matches!(activity_check, GuardDecision::PauseAndRest(_)) {
            return activity_check;
        }
        if matches!(duration_check, GuardDecision::PauseAndRest(_)) {
            return duration_check;
        }

        GuardDecision::AllowFull
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_soil_rhizosphere_density() {
        let guard = MicrospaceIntegrityGuard::new();
        let state = MicrospaceState {
            microspace_id: "soil_001".to_string(),
            occupant_organism: "soil_rhizosphere".to_string(),
            volume_mm3: 1000.0,
            current_swarm_volume_mm3: 10.0, // 1% density
            ecosystem_role: "nutrient_cycling".to_string(),
        };
        let result = guard.evaluate_density(&state);
        assert!(matches!(result, GuardDecision::AllowFull));
    }

    #[test]
    fn test_density_exceeded() {
        let guard = MicrospaceIntegrityGuard::new();
        let state = MicrospaceState {
            microspace_id: "soil_001".to_string(),
            occupant_organism: "soil_rhizosphere".to_string(),
            volume_mm3: 1000.0,
            current_swarm_volume_mm3: 100.0, // 10% density (exceeds 0.5% ceiling)
            ecosystem_role: "nutrient_cycling".to_string(),
        };
        let result = guard.evaluate_density(&state);
        assert!(matches!(result, GuardDecision::Forbid(_)));
    }

    #[test]
    fn test_activity_power_limit() {
        let guard = MicrospaceIntegrityGuard::new();
        let state = MicrospaceState {
            microspace_id: "bee_001".to_string(),
            occupant_organism: "insect_thorax".to_string(),
            volume_mm3: 50.0,
            current_swarm_volume_mm3: 0.05, // 0.1% density, OK
            ecosystem_role: "flight_metabolic".to_string(),
        };
        let proposal = SwarmActivityProposal {
            target_microspace_id: "bee_001".to_string(),
            proposed_energy_draw_mw: 5.0, // exceeds 1 mW limit
            proposed_duration_secs: 600,
            activity_type: "pollination_assist".to_string(),
        };
        let result = guard.evaluate_activity(&state, &proposal);
        assert!(matches!(result, GuardDecision::Forbid(_)));
    }
}
