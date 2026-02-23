# SNC Android FFI Specification

**Filename:** `snc-android-ffi-spec.md`  
**Destination:** `morpheus-neuromorph/docs/snc-android-ffi-spec.md`

---

## Overview

This document specifies the FFI (Foreign Function Interface) contract between the Rust SNC core (`morpheus-neuromorph` workspace) and Android/Kotlin clients. It defines:

1. The C-ABI function signatures for `distill_chat_turn` and `orchestrate_with_hgo_and_shell`.
2. JSON schema contracts for inputs and outputs.
3. How Kotlin serialization maps to Rust structs.
4. No-rollback and neurorights enforcement at the FFI boundary.

---

## 1. C-ABI Function Signatures

### 1.1 `distill_chat_turn`

**Purpose:** Take a chat turn (text + metadata) and return a DistilledKnowledge object (FK, access class, neurorights flags).

**Rust Signature:**
```rust
#[no_mangle]
pub extern "C" fn distill_chat_turn(
    input_json: *const c_char,
) -> *mut c_char
```

**Parameters:**
- `input_json` (const char*): A UTF-8 JSON string conforming to `NeuromorphContentInput` schema.

**Return Value:**
- A UTF-8 JSON string allocated by Rust (`malloc`), conforming to `DistilledKnowledge` schema.
- Caller (Kotlin) must free this with `free_snc_string` (see below).
- On error, returns a JSON object with `"error": "reason"` field.

**Kotlin Wrapper:**
```kotlin
external fun distill_chat_turn(inputJson: String): String
```

---

### 1.2 `orchestrate_with_hgo_and_shell`

**Purpose:** Execute a sovereign evolution step: accept HGO, SovereigntyState, DisciplineSignal, and FateDeck seed; return updated state and evolution choice.

**Rust Signature:**
```rust
#[no_mangle]
pub extern "C" fn orchestrate_with_hgo_and_shell(
    hgo_json: *const c_char,
    sovereignty_state_json: *const c_char,
    discipline_signals_json: *const c_char,
    fate_deck_seed: u64,
) -> *mut c_char
```

**Parameters:**
- `hgo_json`: JSON string conforming to `HitGovernanceObject` schema.
- `sovereignty_state_json`: JSON string conforming to `SovereigntyState` schema.
- `discipline_signals_json`: JSON array of `DisciplineSignal` objects.
- `fate_deck_seed`: A u64 seed for reproducible FateDeck generation (0 = use current entropy).

**Return Value:**
- A UTF-8 JSON string conforming to `OrchestrateResult` schema (see below).
- On error, returns JSON with `"error"` field and `"refused_reason"` explaining why SNC rejected the state transition.

**Kotlin Wrapper:**
```kotlin
external fun orchestrateWithHgoAndShell(
    hgoJson: String,
    sovereigntyStateJson: String,
    disciplineSignalsJson: String,
    fateDeckSeed: Long
): String
```

---

### 1.3 `free_snc_string`

**Purpose:** Free a JSON string allocated by the Rust SNC FFI layer.

**Rust Signature:**
```rust
#[no_mangle]
pub extern "C" fn free_snc_string(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(ptr) };
}
```

**Kotlin Usage:**
```kotlin
val resultJson = distill_chat_turn(inputJson)
try {
    val result = Json.parseToJsonElement(resultJson)
    // process result
} finally {
    freeSncString(resultJson)
}
```

---

## 2. JSON Schema Contracts

### 2.1 NeuromorphContentInput

**Schema ID:** `https://schemas.aln.dev/2026/neuromorph-content-input-v1.json`

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "title": "NeuromorphContentInput",
  "type": "object",
  "required": [
    "text",
    "role_tier",
    "has_biophysical_signal",
    "uses_discipline_signals",
    "dual_empirical_formal_present",
    "uncertainty_exposed"
  ],
  "properties": {
    "text": {
      "type": "string",
      "description": "Plaintext or structured content from chat turn"
    },
    "role_tier": {
      "type": "string",
      "enum": ["Learner", "Mentor", "Teacher", "Researcher"],
      "description": "User role for access gating"
    },
    "has_biophysical_signal": {
      "type": "boolean",
      "description": "True if content includes EEG, HRV, or other biophysical data"
    },
    "uses_discipline_signals": {
      "type": "boolean",
      "description": "True if content encodes FEAR/PAIN as labeled training signals"
    },
    "dual_empirical_formal_present": {
      "type": "boolean",
      "description": "True if empirical + formal linkage verified upstream"
    },
    "uncertainty_exposed": {
      "type": "boolean",
      "description": "True if explicit error bars or uncertainty disclosed"
    }
  },
  "additionalProperties": false
}
```

---

### 2.2 DistilledKnowledge

**Schema ID:** `https://schemas.aln.dev/2026/distilled-knowledge-v1.json`

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "title": "DistilledKnowledge",
  "type": "object",
  "required": [
    "hexstamp",
    "knowledge_factor",
    "access_class",
    "neurorights_compliant",
    "discipline_sovereign_only",
    "rationale"
  ],
  "properties": {
    "hexstamp": {
      "type": "string",
      "description": "Unique hex identifier with timestamp for audit provenance"
    },
    "knowledge_factor": {
      "type": "number",
      "minimum": 0.0,
      "maximum": 1.0,
      "description": "FK scalar (V * R * E * N) for CHAT eligibility"
    },
    "access_class": {
      "type": "string",
      "enum": ["Open", "KnowledgeGated", "HighAutonomy"],
      "description": "Access tier for this knowledge object"
    },
    "neurorights_compliant": {
      "type": "boolean",
      "description": "True if neurorights and consent preconditions met"
    },
    "discipline_sovereign_only": {
      "type": "boolean",
      "description": "True if FEAR/PAIN labeled as voluntary, never coercive"
    },
    "rationale": {
      "type": "string",
      "description": "Human-readable explanation of FK computation and access class"
    }
  },
  "additionalProperties": false
}
```

---

### 2.3 SovereigntyState

**Schema ID:** `https://schemas.aln.dev/2026/sovereignty-state-v1.json`

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "title": "SovereigntyState",
  "type": "object",
  "required": [
    "version",
    "tier",
    "evolution_points",
    "last_transition_hash"
  ],
  "properties": {
    "version": {
      "type": "integer",
      "minimum": 1,
      "description": "Monotonically increasing version; only forward moves allowed"
    },
    "tier": {
      "type": "string",
      "enum": ["Baseline", "Elevated", "Full"],
      "description": "Current autonomy tier; only forward transitions permitted"
    },
    "evolution_points": {
      "type": "number",
      "minimum": 0.0,
      "description": "Cumulative evolution score from FEAR/PAIN contributions"
    },
    "last_transition_hash": {
      "type": "string",
      "description": "SHA256 hash of previous SovereigntyState; for audit chain"
    }
  },
  "additionalProperties": false
}
```

---

### 2.4 HitGovernanceObject

**Schema ID:** `https://schemas.aln.dev/2026/hit-governance-object-v1.json`

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "title": "HitGovernanceObject",
  "type": "object",
  "required": [
    "id",
    "version",
    "owner_did",
    "neurorights_flags",
    "fpic_status",
    "permitted_channels"
  ],
  "properties": {
    "id": {
      "type": "string",
      "description": "Unique HGO identifier (DID-compatible)"
    },
    "version": {
      "type": "integer",
      "minimum": 1,
      "description": "HGO version; incremented on policy changes"
    },
    "owner_did": {
      "type": "string",
      "description": "DID of augmented citizen (e.g., bostrom18...)"
    },
    "neurorights_flags": {
      "type": "object",
      "properties": {
        "no_neural_inputs_for_governance": { "type": "boolean" },
        "no_score_from_inner_state": { "type": "boolean" },
        "no_neuro_coercion": { "type": "boolean" }
      },
      "required": [
        "no_neural_inputs_for_governance",
        "no_score_from_inner_state",
        "no_neuro_coercion"
      ]
    },
    "fpic_status": {
      "type": "string",
      "enum": ["Pending", "Granted", "Withheld", "Revoked"],
      "description": "Free Prior Informed Consent status for this session"
    },
    "permitted_channels": {
      "type": "array",
      "items": { "type": "string" },
      "description": "List of allowed communication/interaction channels"
    }
  },
  "additionalProperties": false
}
```

---

### 2.5 OrchestrateResult

**Schema ID:** `https://schemas.aln.dev/2026/orchestrate-result-v1.json`

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "title": "OrchestrateResult",
  "type": "object",
  "required": [
    "status",
    "new_sovereignty_state",
    "evolution_audit_record_hash"
  ],
  "properties": {
    "status": {
      "type": "string",
      "enum": ["Accepted", "Refused", "Error"],
      "description": "Outcome of orchestration"
    },
    "new_sovereignty_state": {
      "type": "object",
      "description": "Updated SovereigntyState if status=Accepted, else null",
      "oneOf": [
        { "$ref": "#/definitions/SovereigntyState" },
        { "type": "null" }
      ]
    },
    "evolution_audit_record_hash": {
      "type": "string",
      "description": "Hash/ID of the appended EvolutionAuditRecord"
    },
    "error": {
      "type": "string",
      "description": "Error message if status=Error or status=Refused"
    },
    "refused_reason": {
      "type": "string",
      "description": "Specific reason if SNC preconditions rejected the transition"
    }
  },
  "definitions": {
    "SovereigntyState": {
      "$ref": "https://schemas.aln.dev/2026/sovereignty-state-v1.json"
    }
  },
  "additionalProperties": false
}
```

---

## 3. Kotlin Data Classes (Serializable)

### 3.1 NeuromorphContentInput

```kotlin
@Serializable
data class NeuromorphContentInput(
    val text: String,
    @SerialName("role_tier")
    val roleTier: RoleTier,
    @SerialName("has_biophysical_signal")
    val hasBiophysicalSignal: Boolean,
    @SerialName("uses_discipline_signals")
    val usesDisciplineSignals: Boolean,
    @SerialName("dual_empirical_formal_present")
    val dualEmpiricalFormalPresent: Boolean,
    @SerialName("uncertainty_exposed")
    val uncertaintyExposed: Boolean
)

@Serializable
enum class RoleTier {
    @SerialName("Learner")
    LEARNER,
    @SerialName("Mentor")
    MENTOR,
    @SerialName("Teacher")
    TEACHER,
    @SerialName("Researcher")
    RESEARCHER
}
```

### 3.2 DistilledKnowledge

```kotlin
@Serializable
data class DistilledKnowledge(
    val hexstamp: String,
    @SerialName("knowledge_factor")
    val knowledgeFactor: Double,
    @SerialName("access_class")
    val accessClass: AccessClass,
    @SerialName("neurorights_compliant")
    val neurorightsCompliant: Boolean,
    @SerialName("discipline_sovereign_only")
    val disciplineSovereignOnly: Boolean,
    val rationale: String
)

@Serializable
enum class AccessClass {
    @SerialName("Open")
    OPEN,
    @SerialName("KnowledgeGated")
    KNOWLEDGE_GATED,
    @SerialName("HighAutonomy")
    HIGH_AUTONOMY
}
```

### 3.3 SovereigntyState

```kotlin
@Serializable
data class SovereigntyState(
    val version: Int,
    val tier: Tier,
    @SerialName("evolution_points")
    val evolutionPoints: Double,
    @SerialName("last_transition_hash")
    val lastTransitionHash: String
)

@Serializable
enum class Tier {
    @SerialName("Baseline")
    BASELINE,
    @SerialName("Elevated")
    ELEVATED,
    @SerialName("Full")
    FULL
}
```

---

## 4. No-Rollback Enforcement at FFI Boundary

### 4.1 Kotlin-Side Schema Version Check

Before deserializing any `SovereigntyState` or `DistilledKnowledge`:

```kotlin
fun validateSchemaVersion(
    incomingVersion: Int,
    schemaId: String,
    lastAppliedVersion: Int
): Result<Unit> {
    // Check schema ID is in allow-list
    val allowedIds = setOf(
        "https://schemas.aln.dev/2026/sovereignty-state-v1.json",
        "https://schemas.aln.dev/2026/distilled-knowledge-v1.json"
    )
    if (schemaId !in allowedIds) {
        return Result.failure(Exception("Unknown schema: $schemaId"))
    }
    
    // Enforce monotone version
    if (incomingVersion < lastAppliedVersion) {
        return Result.failure(
            Exception("Downgrade rejected: incoming=$incomingVersion < last=$lastAppliedVersion")
        )
    }
    
    return Result.success(Unit)
}
```

### 4.2 Audit Log Entry

Every accepted transition appends an immutable record:

```kotlin
data class EvolutionAuditEntry(
    val timestamp: Long = System.currentTimeMillis(),
    val previousStateHash: String,
    val newStateHash: String,
    val schemaId: String,
    val sovereigntyStateVersion: Int,
    val status: String = "Accepted"
)

suspend fun recordAuditEntry(entry: EvolutionAuditEntry) {
    // Insert into Room database (append-only, no UPDATE allowed)
    val dao = AppDatabase.getInstance(context).auditDao()
    dao.insert(entry)
}
```

---

## 5. Integration Checklist

- [ ] Rust `ffi-core` crate compiles to C-ABI with no panics on invalid input.
- [ ] `distill_chat_turn` and `orchestrate_with_hgo_and_shell` linked into Android `.so` library.
- [ ] Kotlin wrapper functions call C-ABI via JNI.
- [ ] All JSON schemas published to `https://schemas.aln.dev/2026/` (or your GitHub).
- [ ] Kotlin `@Serializable` data classes validated against schemas.
- [ ] SQLite audit log table confirmed append-only (no UPDATE triggers).
- [ ] Android tests confirm version check rejects downgrade attempts.
- [ ] Kotlin tests confirm `free_snc_string` cleanup prevents leaks.

---

## References

- [W3C Verifiable Credentials JSON Schema](https://www.w3.org/TR/vc-json-schema/)
- [Rust FFI Best Practices](https://doc.rust-lang.org/nomicon/ffi.html)
- [kotlinx.serialization Docs](https://github.com/Kotlin/kotlinx.serialization)
- Sovereign Neuromorph Contract (SNC) Rust Framework (`home-finance-travel-shopping-a-1meuBTeBT.2PKmUZq2fuJg.md`)
- Neutral Grammar for Neuromorphic Interoperability (`a-neutral-grammar-for-neuromor-cdefJw16TRyjXgQyv5S2.A.md`)

---

**Next Steps:**
1. Implement Rust FFI skeleton in `cratesffi-core/`.
2. Generate `.so` library for Android.
3. Create Kotlin wrapper module.
4. Write integration tests for schema validation and no-rollback enforcement.
