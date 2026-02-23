data class HitGovernanceObject(
    val version: String = "hit-governance-object.v1",
    val licenseId: String,
    val systemId: String,
    val humanOversightRequired: Boolean,
    val abortControlRequired: Boolean,
    val labelAiSegments: Boolean,
    val emitProvenanceMetadata: Boolean,
    val forbidCovertInference: Boolean,
    val forbidCoerciveChannels: Boolean,
    val requireConsentForPersonalData: Boolean,
    val requireFpicForIndigenousData: Boolean
)
