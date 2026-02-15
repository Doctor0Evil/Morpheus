use crate::aln::{AlnDocument, AlnKey, AlnProperty};
use crate::model::{GovernanceProfile, ParsedRoleKind};
use morpheus_core::capabilities::{CapabilityState, CapabilityTier};
use morpheus_core::rights::RightsLedgerEntry;
use crate::model::SpeciesProfile;
use morpheus_core::species::{BiophysicalEnvelope, SpeciesKind};
use crate::model::ReversalPolicyProfile;
use crate::model::ReversalSettings;
use crate::model::ReversalPermission;
use crate::model::ParsedSection;
use crate::model::ParsedDocument;
use crate::model::ParsedError;

pub fn parse_aln(input: &str) -> Result<AlnDocument, ParsedError> {
    let mut doc = AlnDocument::default();
    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if let Some((k, v)) = trimmed.split_once('=') {
            let key = match k.trim() {
                "SECTION" => AlnKey::Section,
                "ROLE" => AlnKey::Role,
                "RIGHTS" => AlnKey::Rights,
                "CAPABILITIES" => AlnKey::Capabilities,
                "SPECIES" => AlnKey::Species,
                "REVERSAL_POLICY" => AlnKey::ReversalPolicy,
                other => AlnKey::Custom(other.to_string()),
            };
            let value = v.trim().trim_matches('"').to_string();
            doc.properties.push(AlnProperty { key, value });
        }
    }
    Ok(doc)
}

pub fn to_governance_profile(input: &str) -> Result<GovernanceProfile, ParsedError> {
    let doc = parse_aln(input)?;
    let role_kind = ParsedRoleKind::from_values(doc.get_values(&AlnKey::Role));
    let rights = RightsLedgerEntry::monotone_default(
        "Morpheus-Subject",
        "Right to exist and bear neuromorphic intelligence is independent of capability tier.",
        chrono::Utc::now().to_rfc3339(),
    );
    let capability_state = CapabilityState {
        tier: CapabilityTier::Stable,
        can_self_modify: false,
        can_request_transition: true,
    };
    let species_profile = SpeciesProfile {
        envelope: BiophysicalEnvelope {
            species: SpeciesKind::Hybrid,
            min_safe_roh: 0.0,
            max_safe_roh: 0.30,
            roH_monotone: true,
            no_cross_species_signals: true,
        },
    };
    let reversal_policy = ReversalPolicyProfile {
        settings: ReversalSettings {
            permission: ReversalPermission::DisallowNeuromorphReversal,
        },
    };
    Ok(GovernanceProfile {
        section: ParsedSection::Morpheus,
        document: ParsedDocument { raw: input.to_string() },
        role: role_kind,
        rights,
        capability_state,
        species_profile,
        reversal_policy,
    })
}
