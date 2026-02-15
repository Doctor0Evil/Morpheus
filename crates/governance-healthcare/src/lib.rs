#![forbid(unsafe_code)]

use std::time::SystemTime;

/// Risk tiers for healthcare AI / neuromorphic systems.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClinicalRiskTier {
    Low,
    Medium,
    High,
    Critical,
}

/// Where and how the model is used in care delivery.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClinicalUseCase {
    Triage,
    DiagnosticSupport,
    TreatmentRecommendation,
    Monitoring,
    Administrative,
    ResearchOnly,
}

/// How human oversight is wired into the workflow.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HitlPattern {
    /// Human must always review before action (recommend-only).
    HumanReviewRequired,
    /// Human can override; system can auto-act within guardrails.
    HumanOverrideCapable,
    /// Auto-execution; allowed only for Low risk, non-clinical.
    AutonomousWithinLimits,
}

/// Basic consent and FPIC / IDS flags for this deployment.
#[derive(Clone, Debug)]
pub struct ConsentProfile {
    /// True if individual patient consent / notice is required.
    pub requires_individual_consent: bool,
    /// True if Indigenous / community data is involved.
    pub involves_indigenous_or_community_data: bool,
    /// True if FPIC has been recorded for affected communities.
    pub fpic_granted: bool,
}

/// Retention profile for audit logs (in years).
#[derive(Clone, Debug)]
pub struct LoggingProfile {
    /// Minimum retention period for high‑stakes logs.
    pub min_retention_years: u8,
    /// Whether tamper‑evident / append‑only storage is required.
    pub tamper_evident_required: bool,
    /// Whether full decision traces must be stored
    /// (inputs, model version, overrides, timestamps).
    pub full_decision_trace_required: bool,
}

/// Minimal provenance requirements for training / tuning data.
#[derive(Clone, Debug)]
pub struct DatasetProvenancePolicy {
    /// True if every dataset must declare source and license.
    pub require_source_and_license: bool,
    /// True if consent basis and jurisdiction tags are mandatory.
    pub require_consent_and_jurisdiction_tags: bool,
    /// True if biosignal / neuromorphic data must be explicitly labeled.
    pub require_biosignal_labelling: bool,
}

/// Governance policy object for a single healthcare model / stack.
/// This is what CI can validate before deploy.
#[derive(Clone, Debug)]
pub struct HealthcareGovernancePolicy {
    pub model_id: String,
    pub owner: String,
    pub clinical_use_case: ClinicalUseCase,
    pub risk_tier: ClinicalRiskTier,
    pub hitl_pattern: HitlPattern,
    pub consent_profile: ConsentProfile,
    pub logging: LoggingProfile,
    pub dataset_provenance: DatasetProvenancePolicy,
    /// True if biosignals / neuromorphic channels are used.
    pub uses_biosignals: bool,
    /// True if the stack processes Indigenous / community‑linked data.
    pub touches_indigenous_data: bool,
    /// Timestamp when this policy snapshot was created.
    pub created_at: SystemTime,
}

/// Validation result for CI / orchestration.
#[derive(Clone, Debug)]
pub struct PolicyValidationResult {
    pub ok: bool,
    pub errors: Vec<String>,
}

impl PolicyValidationResult {
    pub fn is_ok(&self) -> bool {
        self.ok
    }
}

/// Core validator: apply HIT‑style but license‑agnostic checks
/// for healthcare deployments.
///
/// Typical CI usage: fail the pipeline if !result.is_ok().
pub fn validate_healthcare_policy(policy: &HealthcareGovernancePolicy) -> PolicyValidationResult {
    let mut errors = Vec::new();

    // 1. Basic identifiers.
    if policy.model_id.trim().is_empty() {
        errors.push("model_id must not be empty".to_string());
    }
    if policy.owner.trim().is_empty() {
        errors.push("owner must not be empty".to_string());
    }

    // 2. HITL constraints by risk tier.
    match policy.risk_tier {
        ClinicalRiskTier::High | ClinicalRiskTier::Critical => {
            match policy.hitl_pattern {
                HitlPattern::HumanReviewRequired | HitlPattern::HumanOverrideCapable => {}
                HitlPattern::AutonomousWithinLimits => {
                    errors.push(
                        "AutonomousWithinLimits is forbidden for High/Critical clinical risk"
                            .to_string(),
                    );
                }
            }
        }
        ClinicalRiskTier::Medium => {
            // Medium risk can be HumanReviewRequired or HumanOverrideCapable.
            if let HitlPattern::AutonomousWithinLimits = policy.hitl_pattern {
                errors.push(
                    "AutonomousWithinLimits is not allowed for Medium clinical risk".to_string(),
                );
            }
        }
        ClinicalRiskTier::Low => {
            // Low risk: all patterns are allowed, but we still log.
        }
    }

    // 3. Consent profile vs risk tier.
    if matches!(
        policy.risk_tier,
        ClinicalRiskTier::Medium | ClinicalRiskTier::High | ClinicalRiskTier::Critical
    ) {
        if !policy.consent_profile.requires_individual_consent {
            errors.push(
                "Medium/High/Critical risk deployments must require individual consent/notice"
                    .to_string(),
            );
        }
    }

    // 4. Indigenous Data Sovereignty / FPIC constraints.
    if policy.touches_indigenous_data || policy.consent_profile.involves_indigenous_or_community_data
    {
        if !policy.consent_profile.fpic_granted {
            errors.push(
                "FPIC must be granted before deploying models that touch Indigenous/community data"
                    .to_string(),
            );
        }
    }

    // 5. Logging constraints by risk tier.
    match policy.risk_tier {
        ClinicalRiskTier::High | ClinicalRiskTier::Critical => {
            if policy.logging.min_retention_years < 7 {
                errors.push(
                    "High/Critical risk deployments must retain logs for at least 7 years"
                        .to_string(),
                );
            }
            if !policy.logging.tamper_evident_required {
                errors.push(
                    "High/Critical risk deployments must use tamper‑evident log storage"
                        .to_string(),
                );
            }
            if !policy.logging.full_decision_trace_required {
                errors.push(
                    "High/Critical risk deployments must store full decision traces".to_string(),
                );
            }
        }
        ClinicalRiskTier::Medium => {
            if policy.logging.min_retention_years < 5 {
                errors.push(
                    "Medium risk deployments should retain logs for at least 5 years".to_string(),
                );
            }
        }
        ClinicalRiskTier::Low => {
            // Low: keep as configured; CI can enforce local defaults.
        }
    }

    // 6. Dataset provenance requirements when biosignals are used.
    if policy.uses_biosignals {
        if !policy.dataset_provenance.require_biosignal_labelling {
            errors.push(
                "uses_biosignals=true requires biosignal labelling in dataset provenance"
                    .to_string(),
            );
        }
    }

    // 7. General dataset provenance invariants.
    if !policy.dataset_provenance.require_source_and_license {
        errors.push("Training datasets must declare source and license".to_string());
    }
    if !policy
        .dataset_provenance
        .require_consent_and_jurisdiction_tags
    {
        errors.push("Training datasets must include consent and jurisdiction tags".to_string());
    }

    PolicyValidationResult {
        ok: errors.is_empty(),
        errors,
    }
}
