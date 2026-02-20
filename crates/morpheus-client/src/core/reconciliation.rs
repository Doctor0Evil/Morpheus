//! Reconciliation engine: evaluates evolution proposals against all three pillars
//!
//! Integrates EvolutionAuditRecords, pluggable policies, and monotonicity checks
//! into a unified decision framework.

use crate::types::{
    audit::{EvolutionAuditRecord, EvolutionOutcome},
    corridor::EcoCorridorContext,
    evidence::EvidenceBundle,
    guards::{BciCeilingGuard, EnvelopeGuard, GuardDecision, RoHGuard},
    policy::PolicyProfile,
};
use crate::MorpheusError;
use std::sync::Arc;
use tracing::{debug, error, info, warn};

/// An evolution proposal to be evaluated
pub struct EvolutionProposal {
    /// DID of the proposer
    pub did: String,
    /// Corridor context
    pub corridor_context: EcoCorridorContext,
    /// Evidence bundle
    pub evidence_bundle: EvidenceBundle,
    /// Description of the neuromorphic decision
    pub neuromorphic_decision: String,
    /// Current BCI* value
    pub current_bci: f64,
    /// Proposed BCI* after evolution
    pub proposed_bci: f64,
    /// Current RoH value
    pub current_roh: f64,
    /// Proposed RoH after evolution
    pub proposed_roh: f64,
    /// Current duty cycle
    pub current_duty_cycle: f64,
    /// Proposed duty cycle
    pub proposed_duty_cycle: f64,
    /// Current session length (minutes)
    pub current_session_length: u32,
    /// Proposed session length (minutes)
    pub proposed_session_length: u32,
}

/// The reconciliation engine
pub struct ReconciliationEngine {
    /// Active policy profile
    pub policy_profile: Arc<PolicyProfile>,
    /// BCI ceiling guard
    pub bci_guard: BciCeilingGuard,
    /// RoH guard (instantiated per proposal)
    pub roh_ceiling: f64,
    /// Envelope guard (instantiated per proposal)
    pub envelope_guard_enabled: bool,
}

impl ReconciliationEngine {
    /// Create a new reconciliation engine
    pub fn new(policy_profile: PolicyProfile) -> Result<Self, MorpheusError> {
        policy_profile.validate().map_err(|e| MorpheusError::PolicyError(e))?;

        let bci_ceiling = policy_profile.biomech_policy.bci_ceiling;
        let warn_threshold = (bci_ceiling * 0.85).max(0.0);

        Ok(Self {
            policy_profile: Arc::new(policy_profile),
            bci_guard: BciCeilingGuard::new(bci_ceiling, warn_threshold),
            roh_ceiling: 0.3, // Hard constitutional ceiling
            envelope_guard_enabled: true,
        })
    }

    /// Evaluate a complete evolution proposal
    pub fn evaluate_evolution(
        &self,
        proposal: &EvolutionProposal,
    ) -> Result<(EvolutionOutcome, EvolutionAuditRecord), MorpheusError> {
        info!(
            "Evaluating evolution proposal for DID: {}",
            proposal.did
        );

        // Step 1: Validate corridor context
        proposal
            .corridor_context
            .validate()
            .map_err(|e| MorpheusError::CorridorViolation(e))?;

        // Step 2: Validate evidence bundle
        proposal
            .evidence_bundle
            .validate()
            .map_err(|e| MorpheusError::EvidenceInvalid(e))?;

        // Step 3: Run BCI ceiling guard
        debug!(
            "Running BCI guard: current={}, proposed={}",
            proposal.current_bci, proposal.proposed_bci
        );
        let bci_decision = self.bci_guard.evaluate(proposal.proposed_bci);
        if matches!(bci_decision, GuardDecision::Forbid(_)) {
            return Err(MorpheusError::GuardRejection(format!(
                "BCI guard rejected: {:?}",
                bci_decision
            )));
        }

        // Step 4: Run RoH monotonicity guard
        debug!(
            "Running RoH guard: current={}, proposed={}",
            proposal.current_roh, proposal.proposed_roh
        );
        let roh_guard = RoHGuard::new(self.roh_ceiling, proposal.current_roh);
        let roh_decision = roh_guard.evaluate(proposal.proposed_roh);
        if matches!(roh_decision, GuardDecision::Forbid(_)) {
            return Err(MorpheusError::MonotonicityViolation(format!(
                "RoH guard rejected: {:?}",
                roh_decision
            )));
        }

        // Step 5: Run envelope-tightening guard
        if self.envelope_guard_enabled {
            debug!("Running envelope guard");
            let envelope_guard = EnvelopeGuard::new(proposal.current_duty_cycle, proposal.current_session_length);
            let envelope_decision = envelope_guard.evaluate(proposal.proposed_duty_cycle, proposal.proposed_session_length);
            if matches!(envelope_decision, GuardDecision::Forbid(_)) {
                return Err(MorpheusError::GuardRejection(format!(
                    "Envelope guard rejected: {:?}",
                    envelope_decision
                )));
            }
        }

        // Step 6: Check policy profile neurorights constraints
        for constraint in &self.policy_profile.neurorights_constraints {
            if constraint.enforced && constraint.name.contains("Forbidden") {
                return Err(MorpheusError::PolicyError(format!(
                    "Policy constraint violated: {}",
                    constraint.name
                )));
            }
        }

        // Step 7: Create audit record
        let mut audit_record = EvolutionAuditRecord::new(
            proposal.did.clone(),
            proposal.corridor_context.clone(),
            proposal.evidence_bundle.clone(),
            self.policy_profile.name.clone(),
            proposal.neuromorphic_decision.clone(),
        );

        audit_record.set_outcome(
            EvolutionOutcome::Allowed,
            proposal.current_bci,
            Some(proposal.proposed_bci),
            proposal.current_roh,
            Some(proposal.proposed_roh),
        );

        // Verify monotonicity
        if !audit_record.respects_monotonicity() {
            return Err(MorpheusError::MonotonicityViolation(
                "Audit record violates monotonicity constraint".to_string(),
            ));
        }

        info!("Evolution proposal APPROVED");
        Ok((EvolutionOutcome::Allowed, audit_record))
    }

    /// Update the active policy profile
    pub fn set_policy_profile(&mut self, profile: PolicyProfile) -> Result<(), MorpheusError> {
        profile.validate().map_err(|e| MorpheusError::PolicyError(e))?;
        self.policy_profile = Arc::new(profile);
        self.bci_guard = BciCeilingGuard::new(
            self.policy_profile.biomech_policy.bci_ceiling,
            self.policy_profile.biomech_policy.bci_ceiling * 0.85,
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::corridor::EcoCorridorContext;
    use crate::types::evidence::EvidenceBundle;
    use crate::types::policy::PolicyProfile;

    #[test]
    fn test_reconciliation_engine_creation() {
        let profile = PolicyProfile::new("test".to_string(), "1.0".to_string(), "test".to_string());
        let engine = ReconciliationEngine::new(profile);
        assert!(engine.is_ok());
    }

    #[test]
    fn test_evolution_proposal_evaluation() {
        let profile = PolicyProfile::new("test".to_string(), "1.0".to_string(), "test".to_string());
        let mut engine = ReconciliationEngine::new(profile).unwrap();
        
        let mut corridor = EcoCorridorContext::new("test".to_string(), "Test".to_string());
        corridor.jurisdictions.push("US/Arizona".to_string());
        
        let evidence = EvidenceBundle::new("ev1".to_string(), 0.9, 0.1);
        
        let proposal = EvolutionProposal {
            did: "did:bostrom:test".to_string(),
            corridor_context: corridor,
            evidence_bundle: evidence,
            neuromorphic_decision: "test".to_string(),
            current_bci: 0.1,
            proposed_bci: 0.15,
            current_roh: 0.1,
            proposed_roh: 0.12,
            current_duty_cycle: 0.5,
            proposed_duty_cycle: 0.4,
            current_session_length: 60,
            proposed_session_length: 45,
        };

        let result = engine.evaluate_evolution(&proposal);
        assert!(result.is_ok());
    }
}
