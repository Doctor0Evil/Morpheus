/// Non-actuating record of a single evolution decision.
/// Compatible with existing ALN.evo JSONL layout.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionAuditRecord {
    pub tx_id: String,
    pub subject_did: String,
    pub host_id: String,

    pub corridor_id: String,
    pub peco_id: String,
    pub pbee_id: String,
    pub ptree_id: String,
    pub pservice_id: Option<String>,

    pub pre_bio: BioStateSnapshot,
    pub post_bio: BioStateSnapshot,
    pub pre_eco: EcoStateSnapshot,
    pub post_eco: EcoStateSnapshot,

    pub proposal: EvolutionProposalRef,
    pub policy_profile_id: String,

    pub consent_vc: ConsentRef,
    pub evidence_bundle_id: String,

    pub verdict: EvolutionVerdict,

    /// Explicit ethical marker: why this decision is heroic / protective
    /// in corridor terms. Non-actuating, for documentation and audit only.
    pub refusal_semantics: Option<RefusalSemantics>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvolutionVerdict {
    AllowFullAction,
    DegradePrecision,
    PauseAndRest,
    DenyEvolution,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefusalSemantics {
    /// Corridor in whose name the refusal happened (e.g. "Eco", "RoH", "FPIC").
    pub axis: RefusalAxis,
    /// Machine-checkable reason, aligned with your invariants.
    pub reason: RefusalReason,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RefusalAxis {
    Biophysical,
    Ecological,
    Neurorights,
    Karma,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RefusalReason {
    RoHWouldIncrease,
    BciWouldExceedCeiling,
    EcoAdmissibleWouldFail,
    BeeTreeServiceWouldFail,
    FpicMissingOrRevoked,
    NeurorightsProfileViolation,
}
