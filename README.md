# avrig-ai

**An autonomous agentic engineering system for system development.**

## Vision

avrig-ai is being developed as an agentic virtual company that can take a system from a stated goal through planning, coordinated engineering work, and delivery. People and AI agents work as persistent company actors with clear roles, while software runtimes execute work on their behalf.

The project will grow incrementally. The first milestone is a dependable Office Service and domain boundary; additional capabilities will be introduced behind explicit interfaces rather than built as a large, tightly coupled platform all at once.

## Product direction

The intended system brings together:

- **Office** — the company control plane: employees, positions, work, coordination, and durable company state.
- **Actors and agent runtime** — persistent people and AI employee identities, with dynamic runtimes that can execute tasks and be restarted or replaced independently.
- **Engineering capabilities** — policy, tools, memory, repositories, graph context, and event-driven coordination added as the core workflow requires them.
- **Desktop companion and Office interface** — a Tauri desktop app that gives the user a persistent PET companion and a practical interface for viewing and controlling Office activity.
- **PET avatar options** — a proposed catalogue of 20 visual identities for review, following shared disconnected/connected outfit and status rules.
- **3D virtual office** — an immersive view of employees and company activity, designed to target the smoothness and liveliness of The Sims and AgentOffice.
- **Other edge clients** — interfaces through which people interact with and oversee the company.

These are parts of one system with separate responsibilities. The desktop PET and 3D office are views and control surfaces; neither owns company state.

## Architectural rules

- **PostgreSQL is authoritative** for durable company, employee, and workflow state.
- **Apache AGE provides graph relationships and context.** Graph data complements PostgreSQL and does not become a competing source of truth.
- **IAM is external.** Authentication and credential lifecycle belong to an identity provider, not the domain model.
- **Actor/Employee identity persists.** Agent Runtime is dynamic execution state and may start, stop, or be replaced without changing the employee.
- **The CEO PET is an AI Employee with Position=CEO.** PET/avatar presentation is not the identity record.
- **The 3D office and desktop companion are projections and clients.** They display and control company workflows through Office APIs; they are not the source of truth.

## Implementation plan

1. **Establish the foundation:** create a clean Rust workspace, infrastructure-independent domain types, and the Office Service boundary.
2. **Make company state durable:** define PostgreSQL schemas and migrations, then implement repositories and Office use cases against them.
3. **Add the service contract:** grow the HTTP API around Office use cases and integrate external IAM at the boundary.
4. **Build the desktop companion and Office UI:** use Tauri for the desktop shell, PET companion, and user-facing Office panels. Keep the UI connected to Office APIs rather than embedding company truth in the client.
5. **Explore PET designs:** review the [20 proposed avatar options](avatar/AVATAR_OPTIONS.md), choose a visual direction, and create a representative connected/disconnected asset pair before producing the full catalogue.
6. **Prototype the immersive 3D office:** evaluate a dedicated 3D engine, starting with a representative scene containing a room, several employees, and basic movement/idle animation. Unity is the current candidate for the Sims/AgentOffice smoothness target; keep the renderer decision open until the prototype is profiled on target hardware.
7. **Support agent work:** introduce actor lifecycle, dynamic Agent Runtime, policy, tools, memory, and event-bus capabilities as concrete use cases require them.
8. **Add graph context:** use Apache AGE for relationship and context queries while keeping authoritative records in PostgreSQL.
9. **Validate end-to-end:** add integration and system tests around durable state, permissions, task execution, desktop workflows, and 3D projections.

The sequence is intentional: establish the source of truth and service boundary, then add usable interfaces and execution capabilities. The 3D office is an explicit visual-quality goal, but its engine and final architecture should be validated with a realistic prototype before full production investment. Reserved directories mark intended direction; they do not imply implementation is complete.

## Current status

The initial implementation is the first foundation milestone:

- `crates/domain` contains infrastructure-independent employee concepts.
- `crates/office-core` defines the Office use-case boundary and repository port.
- `crates/office-api` provides a minimal HTTP health endpoint.
- `avatar/AVATAR_OPTIONS.md` documents 20 proposed PET visual directions; none is selected as the default.
- Planned crates and top-level areas are reserved for later work.

PostgreSQL persistence, migrations, Apache AGE, external IAM, agent execution, Tauri desktop app, PET assets, and the 3D office are **not implemented yet**.

## Planned workspace

Core crates: `domain`, `actor`, `agent-runtime`, `office-core`, `office-api`, `event-bus`, `memory`, `policy`, `tools`, `repository`, `graph`, and `iam`.

Top-level areas: `office/`, `edge/`, `avatar/`, `migrations/`, `docs/`, and `tests/`.

See [docs/architecture.md](docs/architecture.md) for the initial service and data ownership boundaries.
