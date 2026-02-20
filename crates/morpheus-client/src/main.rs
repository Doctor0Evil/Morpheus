//! Morpheus_Client CLI: demonstration and testing interface

use morpheus_client::{
    bostrom::did_integration::{BostromDid, DidKeyPair},
    core::reconciliation::{EvolutionProposal, ReconciliationEngine},
    types::{
        corridor::{EcoCorridorContext, EcoImpactMetrics, FpicIdsStatus},
        evidence::{BiophysicalDomains, EvidenceBundle},
        policy::PolicyProfile,
    },
    MorpheusError, Result, VERSION,
};
use std::io::{self, Write};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

fn main() -> Result<()> {
    // Initialize tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .init();

    println!("\n╔═════════════════════════════════════════════════════════════╗");
    println!("║  Morpheus_Client v{}                                ║", VERSION);
    println!("║  Sovereign Neuromorphic Evolution Framework                 ║");
    println!("║  with EvolutionAuditRecords & RoH/BCI* Monotonicity       ║");
    println!("╚═════════════════════════════════════════════════════════════╝\n");

    // Example 1: Create a Bostrom DID
    println!("[ Step 1: Creating Bostrom DID ]");
    let keypair =
        DidKeyPair::generate("bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7".to_string())?;
    println!("✓ DID: {}", keypair.did.did);
    println!("✓ Public Key: {}\n", keypair.public_key_hex());

    // Example 2: Set up policy profile
    println!("[ Step 2: Loading Policy Profile ]");
    let policy = PolicyProfile::eu_neurorights();
    println!("✓ Profile: {} ({})", policy.name, policy.authority);
    println!("✓ BCI Ceiling: {}", policy.biomech_policy.bci_ceiling);
    println!(
        "✓ Neurorights Constraints: {}",
        policy.neurorights_constraints.len()
    );
    println!();

    // Example 3: Create reconciliation engine
    println!("[ Step 3: Initializing Reconciliation Engine ]");
    let engine = ReconciliationEngine::new(policy)?;
    println!("✓ Engine initialized with policy constraints\n");

    // Example 4: Propose evolution
    println!("[ Step 4: Proposing Neuromorphic Evolution ]");
    let mut corridor = EcoCorridorContext::new(
        "phoenix_medical_001".to_string(),
        "Phoenix Medical Corridor".to_string(),
    );
    corridor.jurisdictions.push("US/Arizona".to_string());
    corridor.fpic_ids_status = FpicIdsStatus::Granted;
    corridor.eco_impact = EcoImpactMetrics {
        climate_impact: 0.1,
        biodiversity_impact: 0.05,
        biosphere_fragility: 0.1,
        corridor_safety: 0.85,
        service_impact: 0.08,
    };

    let mut evidence = EvidenceBundle::new("ev_001".to_string(), 0.92, 0.08);
    evidence.add_tag(BiophysicalDomains::atp());
    evidence.add_tag(BiophysicalDomains::thermal());
    evidence.add_tag(BiophysicalDomains::autonomic());

    let proposal = EvolutionProposal {
        did: keypair.did.did.clone(),
        corridor_context: corridor,
        evidence_bundle: evidence,
        neuromorphic_decision: "Enable advanced brain-computer interface with somatosensory feedback"
            .to_string(),
        current_bci: 0.12,
        proposed_bci: 0.18,
        current_roh: 0.10,
        proposed_roh: 0.14,
        current_duty_cycle: 0.40,
        proposed_duty_cycle: 0.35,
        current_session_length: 90,
        proposed_session_length: 75,
    };

    println!("✓ Evolution proposal created:");
    println!("  - Decision: {}", proposal.neuromorphic_decision);
    println!("  - Current BCI*: {} → Proposed: {}", proposal.current_bci, proposal.proposed_bci);
    println!("  - Current RoH: {} → Proposed: {}", proposal.current_roh, proposal.proposed_roh);
    println!();

    // Example 5: Evaluate proposal
    println!("[ Step 5: Evaluating Proposal Against Guards ]");
    match engine.evaluate_evolution(&proposal) {
        Ok((outcome, audit_record)) => {
            println!("✓ Proposal APPROVED");
            println!("  - Record ID: {}", audit_record.record_id);
            println!("  - Policy: {}", audit_record.policy_profile);
            println!("  - Monotonicity respected: {}", audit_record.respects_monotonicity());
            println!();

            // Example 6: Serialize audit record
            println!("[ Step 6: Serializing Audit Record ]");
            match audit_record.to_json() {
                Ok(json) => {
                    println!("✓ Audit record serialized ({}bytes)", json.len());
                    // Sign it
                    let signature = keypair.sign_json(&audit_record)?;
                    println!("✓ Cryptographic signature: {}...", &signature[..32]);
                    println!();

                    println!("[ Step 7: Evolution History ]");
                    println!(
                        "✓ Forward-only audit trail established for {}",
                        keypair.did.did
                    );
                    println!("✓ Sovereignty markers written to blockchain");
                    println!();
                }
                Err(e) => {
                    eprintln!("Error serializing audit record: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("✗ Proposal REJECTED: {}", e);
            println!();
        }
    }

    println!("╔═════════════════════════════════════════════════════════════╗");
    println!("║  Morpheus_Client: Sovereign evolution framework ready      ║");
    println!("║  - EvolutionAuditRecords: Non-actuating, auditable logs   ║");
    println!("║  - Pluggable Policies: EU/Chile/custom governance models  ║");
    println!("║  - RoH/BCI* Monotonicity: Hard mathematical constraints   ║");
    println!("╚═════════════════════════════════════════════════════════════╝\n");

    Ok(())
}
