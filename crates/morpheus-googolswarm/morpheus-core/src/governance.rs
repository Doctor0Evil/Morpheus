use crate::capabilities::{CapabilityState, ReversalConditions};
use crate::error::MorpheusError;
use crate::rights::RightsLedgerEntry;

#[derive(Debug, Clone)]
pub struct GovernanceContext {
    pub rights_entry: RightsLedgerEntry,
    pub capability_state: CapabilityState,
}

impl GovernanceContext {
    pub fn request_downgrade(
        &self,
        reversal: &ReversalConditions,
    ) -> Result<CapabilityState, MorpheusError> {
        if !self.rights_entry.is_reversal_allowed() {
            return Err(MorpheusError::ReversalForbidden(
                "neuromorphic reversal is globally disallowed for this subject".to_string(),
            ));
        }
        if !reversal.permits_downgrade() {
            return Err(MorpheusError::ReversalForbidden(
                "reversal conditions not satisfied by sovereignty kernel".to_string(),
            ));
        }
        let mut next = self.capability_state.clone();
        next.can_self_modify = false;
        Ok(next)
    }
}
