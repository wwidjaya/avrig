# Avatar Development Plan

**Status:** Proposed first implementation plan  
**Scope:** Build the PET avatar system first. The desktop Office interface, agent behavior, and 3D virtual office are later integrations.

## Goal

Create a recognizable, accessible PET avatar that can be selected from the 22 proposed options, rendered consistently, and communicate connection and activity state. Avatar identity is presentation metadata; it is not the Employee/Actor identity, authorization, or Agent Runtime.

## Product rules

- The collection uses a cohesive, polished stylized 3D chibi art direction with large expressive eyes, simple features, and a professional office-casual presentation.
- Each avatar has a distinct silhouette, a bright saturated palette unique to that avatar, and materials suited to its concept.
- Preserve each avatar's identity across states.
- Use a minimal T-shirt and jeans as the base outfit, adapted to each character form; when connected, add a clearly visible casual jacket and the shared small chest badge.
- The connected jacket and badge communicate connection only, not authority, role, activity, or trust. A compact tablet may appear during message/alert presentation, regardless of connection state.
- Connection and activity are separate states. For example, a connected PET may be idle, thinking, or working.
- Every state has a text/accessibility equivalent and must not depend on color or animation alone.
- Support reduced motion and a static presentation.
- Record asset authorship, provenance, license, and version before distribution.
- All 22 concepts should be available in the first catalogue release. Bulb is the initial default; users may change it later. Individual model sheets and assets remain subject to visual review.

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

1. Review the 22 concepts in [AVATAR_OPTIONS.md](AVATAR_OPTIONS.md) and the approved direction in [DESIGN_BRIEF.md](DESIGN_BRIEF.md).
2. Use Bulb as the default reference model; users may choose another avatar later.
3. Create a full-body Bulb design sheet and low-fidelity form sheets for the other 19 concepts.
4. Confirm distinct silhouettes and bright individual palettes within the cohesive playful chibi style.
5. Review the adapted T-shirt-and-jeans base outfit, connected casual jacket, shared chest badge, and status readability.

**Gate:** Bulb’s visual reference direction is approved. Remaining: approve a form sheet for the other 21 concepts, showing distinct silhouettes within one cohesive collection.

### Phase 2 — Define the avatar contract

Define stable avatar IDs and versioned metadata, plus independent state fields:

- Connection: not_connected, connecting, connected, connection_lost, unknown
- Activity: idle, listening, thinking, working, waiting_for_user, message_alert, error
- Motion preference: full, reduced, static

Specify safe fallback behavior: unknown or stale connection state must not show the connected jacket. Keep state transitions deterministic and testable independently of rendering.

**Gate:** State mappings, accessible labels, outfit rules, and fallback behavior are documented.

### Phase 3 — Produce the full first-release catalogue

Create source and optimized runtime assets for Bulb first, then produce all 21 other concepts in batches using the approved collection style:

- Compact icon and larger presentation view.
- Full-body character-appropriate base appearance.
- Matching connected version with a form-appropriate outer layer and shared chest badge.
- A small initial set of poses for idle, listening, thinking, and working, using the shared motion language.
- Metadata for dimensions, supported states, version, source, author, and license.

Create the Bulb as a real rigged 3D asset from the approved model sheet, then review that asset in the native [Bevy model viewer](viewer/README.md). The viewer loads a GLB, plays its first exported animation as idle, and supports orbit, zoom, reset, and optional auto-rotation. It intentionally renders no procedural character substitute. The older [web interaction mockup](prototype/index.html) demonstrates state controls only; its character rendering is not concept-accurate. Reuse animation rigs where forms allow, with tailored compatible rigs where distinct silhouettes require them. Keep avatar IDs and semantic states independent from rig details.

**Gate:** All 22 avatars have reviewed base and connected variants, required initial poses, readable status cues, and complete version/provenance/license metadata before the first catalogue release.

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

### Phase 6 — Maintain and grow the catalogue

After all 22 initial options ship:

1. Gather feedback on the collection and shared style.
2. Refine existing options through versioned updates without changing their stable IDs.
3. Add new options only through the same approved production pipeline.
4. Maintain stable IDs and record when options are available, retired, or replaced.
5. Review artwork representation, accessibility, provenance, and licenses for every release.

**Gate:** Every released avatar has consistent base/connected variants, status accessibility, version metadata, and verified usage rights.

## Initial acceptance criteria

- All 22 approved avatars can be selected and rendered in all required first-release states; Bulb is selected by default and can be changed later.
- Disconnected and connected appearance differ by a form-appropriate outer layer and the shared chest badge; these cues communicate connection only.
- Activity does not change connection clothing.
- Unknown or stale connectivity never appears connected.
- State has a text/accessibility equivalent and remains understandable without color.
- Reduced-motion and static modes work.
- Missing or invalid artwork falls back safely.
- Avatar IDs remain separate from Employee IDs, roles, permissions, and Agent Runtime.
- The first avatar milestone does not require Office API or the 3D office, but the initial catalogue release includes all 22 avatar assets.

## Decisions to make before production

- Final model sheets, material treatment, palette values, and target size range for each concept.
- Final per-avatar connection-badge icon treatment and state variants.
- Animation format and asset tooling.
- Supported desktop platforms and performance budgets.
- Asset provenance and contribution review process.
