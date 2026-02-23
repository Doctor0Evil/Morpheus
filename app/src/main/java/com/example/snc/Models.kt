package com.example.snc

import kotlinx.serialization.Serializable

@Serializable
data class DistillRequest(
    val session_id: String,
    val user_id: String?,
    val eco_corridor: EcoCorridorContextJson,
    val history: List<ChatMessageJson>,
    val turn: ChatTurnJson,
    val hgo: HitGovernanceObjectJson,
    val sovereignty_state: SovereigntyStateJson
)

@Serializable
data class DistillResponse(
    val ok: Boolean,
    val data: DistillDataJson? = null,
    val error: String? = null
)

// … plus EcoCorridorContextJson, SovereigntyStateJson, HitGovernanceObjectJson,
// DistillDataJson, OrchestrationResponse etc., matching the Rust JSON.
