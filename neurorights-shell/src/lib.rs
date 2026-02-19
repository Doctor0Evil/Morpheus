use serde::{Deserialize, Serialize};

/// Environment / embodiment plane for an interaction.
/// This separates software-only, hardware, and organic domains.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum EnvironmentPlane {
    SoftwareOnly,          // chat, XR, web
    BciHciEeg,             // non-invasive BCI
    CyberneticHardware,    // exoskeleton, prosthetic
    OrganicHost,           // physiological sensors only
    ExternalEnvironment,   // air, water, soil, traffic
}

/// Minimal outer-domain request that can be governed.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OuterActionRequest {
    pub plane: EnvironmentPlane,
    /// Purely outer metrics: emissions, energy, traffic deltas, etc.
    pub eco_delta: f32,
    pub physical_risk: f32,
    /// Policy label (e.g., "traffic-signal-update", "hvac-adjust").
    pub policy_label: String,
}

/// Inner-domain hint is strictly host-local and never used for gating.
/// It can be logged for therapy or safety, but not for permissions.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InnerDomainHint {
    /// Optional coarse stress index 0..1 (host-local only).
    pub stress: Option<f32>,
    /// Optional focus/overload index 0..1 (host-local only).
    pub cognitive_load: Option<f32>,
}

/// Reasons a neuroright would be violated by a proposed use.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NeurorightViolation {
    /// Attempt to use inner-domain data for access control or scoring.
    InnerDomainGatingForbidden,
    /// Attempt to export raw or re-identifiable neural data off-host.
    NeuralExportForbidden,
    /// Attempt to coerce augmentation for essential services.
    CoerciveUptake,
}

/// Policy profile describing how a system must behave.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NeurorightsPolicy {
    pub allow_neural_export: bool,
    pub allow_inner_for_safety_only: bool,
    pub essential_service: bool,
}

impl Default for NeurorightsPolicy {
    fn default() -> Self {
        Self {
            allow_neural_export: false,
            allow_inner_for_safety_only: true,
            essential_service: false,
        }
    }
}

/// Core guard that enforces the neurorights constraints.
/// It must be invoked before any access-control or scoring decision.
pub struct NeurorightsShell {
    pub policy: NeurorightsPolicy,
}

impl NeurorightsShell {
    pub fn new(policy: NeurorightsPolicy) -> Self {
        Self { policy }
    }

    /// Validate that an outer action decision does NOT depend on inner-domain hints
    /// for rights, access, or Karma. Inner hints may only be used for host-local
    /// safety (e.g., throttling a session) and never for external permissions.
    pub fn authorize_outer_action(
        &self,
        _inner: Option<&InnerDomainHint>,
        outer: &OuterActionRequest,
    ) -> Result<(), NeurorightViolation> {
        // This implementation intentionally ignores inner-domain hints entirely.
        // All governance decisions must be driven by outer-domain metrics only.
        let _ = outer; // suppress unused warning

        Ok(())
    }

    /// Enforce export rules for neural/biogenic data.
    pub fn check_neural_export(
        &self,
        attempting_export: bool,
        is_reidentifiable: bool,
    ) -> Result<(), NeurorightViolation> {
        if attempting_export && (!self.policy.allow_neural_export || is_reidentifiable) {
            return Err(NeurorightViolation::NeuralExportForbidden);
        }
        Ok(())
    }

    /// Enforce the "no coercive uptake" rule.
    pub fn check_augmentation_condition(
        &self,
        requires_augmentation: bool,
    ) -> Result<(), NeurorightViolation> {
        if self.policy.essential_service && requires_augmentation {
            return Err(NeurorightViolation::CoerciveUptake);
        }
        Ok(())
    }
}
