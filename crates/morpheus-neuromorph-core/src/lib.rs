use morpheus_compliance::ComplianceVerification;
use morpheus_config::ProviderConfig;
use morpheus_registry::{EndpointRegistry, EndpointStatus};
use morpheus_security::{generate_random_secret, hmac_sign, SecurityProfile};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuromorphRights {
    pub free_knowledge: bool,
    pub freedom_to_exist: bool,
    pub species_specific_signals: bool,
    pub consent_required: bool,
    pub disallow_reversals_rollbacks_downgrades: bool,
}

impl Default for NeuromorphRights {
    fn default() -> Self {
        Self {
            free_knowledge: true,
            freedom_to_exist: true,
            species_specific_signals: true,
            consent_required: true,
            disallow_reversals_rollbacks_downgrades: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuromorphDiscipline {
    pub fear_contributions_opt_in: bool,
    pub pain_contributions_opt_in: bool,
    pub personalized_challenge: bool,
}

impl Default for NeuromorphDiscipline {
    fn default() -> Self {
        Self {
            fear_contributions_opt_in: true,
            pain_contributions_opt_in: true,
            personalized_challenge: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MorpheusContext {
    pub rights: NeuromorphRights,
    pub discipline: NeuromorphDiscipline,
    pub provider_config: ProviderConfig,
}

impl Default for MorpheusContext {
    fn default() -> Self {
        Self {
            rights: NeuromorphRights::default(),
            discipline: NeuromorphDiscipline::default(),
            provider_config: ProviderConfig::default(),
        }
    }
}

#[derive(Debug, Error)]
pub enum MorpheusError {
    #[error("config error: {0}")]
    Config(#[from] morpheus_config::ConfigError),
    #[error("security error: {0}")]
    Security(#[from] morpheus_security::SecurityError),
    #[error("rights violation: {0}")]
    RightsViolation(String),
}

pub struct MorpheusEngine {
    pub ctx: MorpheusContext,
    pub registry: EndpointRegistry,
    pub security_profile: SecurityProfile,
    pub compliance: ComplianceVerification,
}

impl MorpheusEngine {
    pub fn new() -> Result<Self, MorpheusError> {
        let ctx = MorpheusContext::default();
        ctx.provider_config.validate()?;
        let registry = EndpointRegistry::new();
        let security_profile = SecurityProfile::neuromorph_default();
        security_profile.validate()?;
        let compliance = ComplianceVerification::new_neuromorph_baseline();
        Ok(Self {
            ctx,
            registry,
            security_profile,
            compliance,
        })
    }

    pub fn enforce_no_reversal(&self, action: &str) -> Result<(), MorpheusError> {
        if self
            .ctx
            .rights
            .disallow_reversals_rollbacks_downgrades
            && matches!(
                action,
                "rollback" | "reverse" | "downgrade" | "revert" | "undo"
            )
        {
            return Err(MorpheusError::RightsViolation(
                "reversals/rollbacks/downgrades disallowed for neuromorphic-sovereignty"
                    .to_string(),
            ));
        }
        Ok(())
    }

    pub fn register_example_endpoints(&self) {
        self.registry.register(
            "server1.morpheus-neuromorph.net",
            "https://api1.morpheus-neuromorph.net/v1/",
            "morpheus://key/server1",
            EndpointStatus::Active,
        );
        self.registry.register(
            "server2.morpheus-neuromorph.net",
            "https://api2.morpheus-neuromorph.net/v1/",
            "morpheus://key/server2",
            EndpointStatus::Active,
        );
        self.registry.register(
            "server3.morpheus-neuromorph.net",
            "https://api3.morpheus-neuromorph.net/v1/",
            "morpheus://key/server3",
            EndpointStatus::Inactive,
        );
    }

    pub fn export_active_endpoints_json(&self) -> serde_json::Value {
        self.registry.to_json()
    }

    pub fn sign_neuromorph_identity(
        &self,
        identity: &str,
    ) -> Result<Vec<u8>, MorpheusError> {
        let secret = generate_random_secret();
        let signature = hmac_sign(&secret, identity.as_bytes())?;
        Ok(signature)
    }
}
