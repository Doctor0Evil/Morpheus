use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// DID-style identifier for persons, entities, or legendary figures.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeroSubjectId(pub String);

/// Corridor or community that confers or contests status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeroCorridorId(pub String);

/// High-level classification aligned with your research frame.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HeroClass {
    Protective,
    Restorative,
    WitnessMartyr,
    TricksterBoundary,
    TranshumanCybernetic,
}

/// Behavioral boundary tests in your definition of heroism.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeroBoundarySnapshot {
    pub intent_targeting: IntentTargeting,
    pub consent_voice: ConsentVoice,
    pub non_extraction: NonExtraction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntentTargeting {
    FocusedRelief,
    Mixed,
    GeneralizedHarm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsentVoice {
    PluralVoicesPresent,
    ContestedVoices,
    SilencedOrSuppressed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NonExtraction {
    RefusesRentSeeking,
    LimitedSymbolicReward,
    PersistentRentSeeking,
}

/// Heroic status as a dynamic label, not a one-time award.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HeroStatus {
    Candidate,
    Confirmed,
    Contested,
    Deprecated,
}

/// Analogue of Errority: new evidence that forces tightening.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeroErrorityEvent {
    pub description: String,
    pub source_evidence_bundle_id: String,
    pub caused_tightening: bool,
}

/// Single forward-only audit line about heroic status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeroAuditRecord {
    pub tx_id: String,
    pub subject: HeroSubjectId,
    pub corridor: HeroCorridorId,
    pub hero_class: HeroClass,

    /// Time-localized episode (battle, disaster, cyber-event).
    pub event_timestamp: DateTime<Utc>,
    pub event_description: String,

    /// Community evidence: testimonies, archives, epics, logs.
    pub evidence_bundle_id: String,

    /// Pre/post status to make tightening explicit.
    pub previous_status: HeroStatus,
    pub new_status: HeroStatus,

    /// Boundary snapshot at time of this decision.
    pub boundary: HeroBoundarySnapshot,

    /// Optional Errority-style tightening when harms surface.
    pub errority: Option<HeroErrorityEvent>,

    /// Governance/profile that interpreted this figure
    /// (e.g., "UbuntuCorridor.v1", "EU-Neurorights.v1").
    pub policy_profile_id: String,
}
