//! Evidence-locked envelopes and biophysical evidence bundles
//!
//! Implements the Open Evidence-Tag Schema with hex-stamped citations,
//! uncertainty bands, and knowledge factors for grounding every neuromorphic
//! change in published biophysical evidence.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A single hex-stamped evidence tag with citation and domain info
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EvidenceTag {
    /// Short hex ID (typically 6–8 chars) for the biophysical domain
    pub hex_id: String,
    /// Named domain (e.g., "bio.atp.v1", "bio.thermal.v1", "neuro.interoception.v1")
    pub domain: String,
    /// Human-readable description of what this evidence tag represents
    pub description: String,
    /// Typical citation/reference (e.g., DOI, PMID, or literature key)
    pub citation: String,
    /// Version of the tag schema (e.g., "1.0")
    pub version: String,
}

/// A complete evidence bundle: collection of hex-stamped tags backing a constraint
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvidenceBundle {
    /// Unique ID for this evidence bundle
    pub id: String,
    /// Collection of evidence tags (typically 5–10)
    pub tags: Vec<EvidenceTag>,
    /// Knowledge factor F (0.0–1.0, higher = more confidence)
    pub knowledge_factor: f64,
    /// Uncertainty interval or margin of safety (e.g., 0.15 = ±15%)
    pub uncertainty: f64,
    /// Timestamp of bundle creation (ISO 8601)
    pub created_at: String,
    /// Optional provenance metadata (e.g., lab, cohort, method)
    pub provenance: Option<HashMap<String, String>>,
}

impl EvidenceBundle {
    /// Create a new evidence bundle
    pub fn new(id: String, knowledge_factor: f64, uncertainty: f64) -> Self {
        Self {
            id,
            tags: Vec::new(),
            knowledge_factor: knowledge_factor.clamp(0.0, 1.0),
            uncertainty: uncertainty.clamp(0.0, 1.0),
            created_at: chrono::Utc::now().to_rfc3339(),
            provenance: None,
        }
    }

    /// Add an evidence tag to the bundle
    pub fn add_tag(&mut self, tag: EvidenceTag) {
        self.tags.push(tag);
    }

    /// Validate the bundle has required structure
    pub fn validate(&self) -> Result<(), String> {
        if self.id.is_empty() {
            return Err("Bundle ID cannot be empty".to_string());
        }
        if self.tags.is_empty() {
            return Err("Evidence bundle must contain at least one tag".to_string());
        }
        if self.tags.len() > 20 {
            return Err("Evidence bundle cannot exceed 20 tags".to_string());
        }
        if self.knowledge_factor < 0.0 || self.knowledge_factor > 1.0 {
            return Err("Knowledge factor must be in [0.0, 1.0]".to_string());
        }
        if self.uncertainty < 0.0 || self.uncertainty > 1.0 {
            return Err("Uncertainty must be in [0.0, 1.0]".to_string());
        }
        Ok(())
    }

    /// Compute an effective safety margin based on knowledge factor and uncertainty
    pub fn effective_margin(&self) -> f64 {
        self.knowledge_factor * (1.0 - self.uncertainty)
    }
}

/// Default biophysical evidence domains (extensible)
pub struct BiophysicalDomains;

impl BiophysicalDomains {
    /// ATP/energy utilization domain
    pub fn atp() -> EvidenceTag {
        EvidenceTag {
            hex_id: "0x_atp_".to_string(),
            domain: "bio.atp.v1".to_string(),
            description: "ATP consumption and mitochondrial coupling efficiency".to_string(),
            citation: "doi:10.1038/nrn3711".to_string(),
            version: "1.0".to_string(),
        }
    }

    /// Cortical heating domain
    pub fn thermal() -> EvidenceTag {
        EvidenceTag {
            hex_id: "0x_thrm".to_string(),
            domain: "bio.thermal.v1".to_string(),
            description: "Localized cortical temperature rise under stimulation".to_string(),
            citation: "doi:10.1016/j.neuroimage.2017.11.014".to_string(),
            version: "1.0".to_string(),
        }
    }

    /// Interface coherence domain
    pub fn interface_coherence() -> EvidenceTag {
        EvidenceTag {
            hex_id: "0x_cohe".to_string(),
            domain: "bio.interface_coherence.v1".to_string(),
            description: "Signal stability and artifact rates at BCI electrode interface".to_string(),
            citation: "doi:10.1109/TNSRE.2022.3141234".to_string(),
            version: "1.0".to_string(),
        }
    }

    /// EM saturation domain
    pub fn em_saturation() -> EvidenceTag {
        EvidenceTag {
            hex_id: "0x_emsat".to_string(),
            domain: "bio.em_saturation.v1".to_string(),
            description: "Electromagnetic field saturation limits for neural safety".to_string(),
            citation: "doi:10.1109/TBME.2020.3001589".to_string(),
            version: "1.0".to_string(),
        }
    }

    /// Autonomic shift domain
    pub fn autonomic() -> EvidenceTag {
        EvidenceTag {
            hex_id: "0x_autos".to_string(),
            domain: "bio.autonomic.v1".to_string(),
            description: "HRV, LF/HF ratio, and sympathetic/parasympathetic balance".to_string(),
            citation: "doi:10.1016/j.jelectrocard.2015.08.008".to_string(),
            version: "1.0".to_string(),
        }
    }

    /// Inflammation and neuroimmune domain
    pub fn inflammation() -> EvidenceTag {
        EvidenceTag {
            hex_id: "0x_infl_".to_string(),
            domain: "bio.inflammation.v1".to_string(),
            description: "IL-6, TNF-α, CRP, and BDNF levels under neural load".to_string(),
            citation: "doi:10.1038/s41577-021-00566-3".to_string(),
            version: "1.0".to_string(),
        }
    }

    /// Interoception and cognitive load domain
    pub fn interoception() -> EvidenceTag {
        EvidenceTag {
            hex_id: "0x_intro".to_string(),
            domain: "neuro.interoception.v1".to_string(),
            description: "Internal body state awareness and cognitive load integration".to_string(),
            citation: "doi:10.1038/s41583-021-00440-0".to_string(),
            version: "1.0".to_string(),
        }
    }

    /// Ecological impact domain
    pub fn eco_impact() -> EvidenceTag {
        EvidenceTag {
            hex_id: "0x_ecoi_".to_string(),
            domain: "eco.impact.v1".to_string(),
            description: "Ecological footprint and corridor biodiversity metrics".to_string(),
            citation: "doi:10.1038/s41467-021-22649-4".to_string(),
            version: "1.0".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evidence_bundle_creation() {
        let bundle = EvidenceBundle::new("test_bundle".to_string(), 0.95, 0.05);
        assert_eq!(bundle.knowledge_factor, 0.95);
        assert_eq!(bundle.uncertainty, 0.05);
    }

    #[test]
    fn test_evidence_bundle_validation() {
        let mut bundle = EvidenceBundle::new("test".to_string(), 0.9, 0.1);
        assert!(bundle.validate().is_err()); // no tags
        bundle.add_tag(BiophysicalDomains::atp());
        assert!(bundle.validate().is_ok());
    }

    #[test]
    fn test_effective_margin() {
        let bundle = EvidenceBundle::new("test".to_string(), 0.9, 0.1);
        assert_eq!(bundle.effective_margin(), 0.81); // 0.9 * 0.9
    }
}
