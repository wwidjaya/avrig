# Avatar Development Plan

**Status:** Proposed first implementation plan  
**Scope:** Build the PET avatar system first. The desktop Office interface, agent behavior, and 3D virtual office are later integrations.

## Goal

Create a recognizable, accessible PET avatar that can be selected from the 20 proposed options, rendered consistently, and communicate connection and activity state. Avatar identity is presentation metadata; it is not the Employee/Actor identity, authorization, or Agent Runtime.

## Product rules

- The same avatar identity, face, body, palette, and motif persist across states.
- When **not connected**, the avatar wears a T-shirt and jeans.
- When **connected**, the same outfit gains an additional jacket. The jacket is only a connection cue.
- Connection and activity are separate states. For example, a connected PET may be idle, thinking, or working.
- Every state has a text/accessibility equivalent and must not depend on color or animation alone.
- Support reduced motion and a static presentation.
- Record asset authorship, provenance, license, and version before distribution.
- The 20 concepts remain proposals until a visual direction and initial avatar are reviewed and selected.

## Planned structure

    avatar/
    ├── README.md
    ├── AVATAR_OPTIONS.md
    ├── DEVELOPMENT_PLAN.md
    ├── definitions/       # Stable IDs and presentation metadata
    ├── assets/
    │   ├── source/        # Editable source files
    │   └── runtime/       # Optimized output assets
    ├── runtime/           # State model and rendering integration
    └── docs/              # Style guide and asset pipeline notes

Keep the first implementation in this repository. Do not split it into a Git submodule while its interface and asset pipeline are still changing.

## Development phases

### Phase 1 — Choose the visual direction

1. Review the 20 options in [AVATAR_OPTIONS.md](AVATAR_OPTIONS.md).
2. Select one candidate for the first working avatar; record the choice as an explicit decision.
3. Define its proportions, face, palette, motif, clothing, and intended tone.
4. Produce low-fidelity sketches for the T-shirt-and-jeans state and the matching jacket state.
5. Review readability at desktop companion sizes and confirm the motif does not obscure connection status.

**Gate:** One avatar direction and a paired outfit design are approved for a prototype. Do not produce all 20 character assets yet.

### Phase 2 — Define the avatar contract

Define stable avatar IDs and versioned metadata, plus independent state fields:

- Connection: not_connected, connecting, connected, connection_lost, unknown
- Activity: idle, listening, thinking, working, waiting_for_user, error
- Motion preference: full, reduced, static

Specify safe fallback behavior: unknown or stale connection state must not show the connected jacket. Keep state transitions deterministic and testable independently of rendering.

**Gate:** State mappings, accessible labels, outfit rules, and fallback behavior are documented.

### Phase 3 — Produce one complete avatar asset set

Create source and optimized runtime assets for the selected avatar:

- Compact icon and larger presentation view.
- T-shirt-and-jeans base appearance.
- Matching connected version with jacket.
- A small initial set of poses for idle, listening, thinking, and working.
- Metadata for dimensions, supported states, version, source, author, and license.

Use a paired 2D illustration/sprite prototype first to validate identity, state readability, and rendering performance. Keep the asset contract capable of supporting a later rigged 3D model without changing avatar IDs or semantic states.

**Gate:** The first avatar renders correctly in connected and disconnected states, remains recognizable at target sizes, and has clear text status.

### Phase 4 — Build the presentation runtime

Implement a small presentation layer that:

1. Accepts a validated avatar ID, connection state, activity state, surface size, and motion preference.
2. Resolves the outfit and pose deterministically.
3. Loads the appropriate versioned asset with a neutral fallback.
4. Provides a localized/accessibility status label from semantic state.
5. Disables or reduces animation when requested or when the surface is inactive.

Keep the runtime independent from agent reasoning and service credentials. Initially, drive it with a local preview/debug state selector rather than integrating Office APIs.

**Gate:** All supported states and transitions can be demonstrated in a preview with reduced-motion, static, stale, and asset-failure cases.

### Phase 5 — Desktop PET integration

After the avatar runtime and first asset set are stable, embed them in the Tauri desktop companion. Implement a transparent floating window, drag/move behavior, and basic user controls. Keep window position and display preferences local to the desktop client.

Office connection status may later drive the avatar’s connection state through a typed client interface. The desktop client must not treat cached status as live. Office panel and task-management features remain a separate later milestone.

**Gate:** The PET is usable as a desktop companion without owning or duplicating Employee identity or company state.

### Phase 6 — Expand the catalogue

After the selected avatar ships successfully:

1. Gather feedback on the first avatar and shared style.
2. Refine the 20 options based on that feedback.
3. Produce additional options in batches using the approved production pipeline.
4. Maintain stable IDs and record when options are available, retired, or replaced.
5. Review artwork representation, accessibility, provenance, and licenses for every release.

**Gate:** Every released avatar has consistent base/connected variants, status accessibility, version metadata, and verified usage rights.

## Initial acceptance criteria

- One approved avatar can be rendered in all prototype states.
- Disconnected and connected appearance differ only by the additional jacket and explicit status cue.
- Activity does not change connection clothing.
- Unknown or stale connectivity never appears connected.
- State has a text/accessibility equivalent and remains understandable without color.
- Reduced-motion and static modes work.
- Missing or invalid artwork falls back safely.
- Avatar IDs remain separate from Employee IDs, roles, permissions, and Agent Runtime.
- The first milestone does not require Office API, 3D office, or all 20 final assets.

## Decisions to make before production

- Which of the 20 candidates is the first prototype.
- Final visual style and target size range.
- Whether the production avatar is 2D, rigged 3D, or a staged move from 2D to 3D.
- Animation format and asset tooling.
- Supported desktop platforms and performance budgets.
- Asset provenance and contribution review process.
