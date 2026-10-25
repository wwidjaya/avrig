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
- **Edge clients** — interfaces through which people interact with and oversee the company.
- **Avatars and 3D office** — a visual way to represent actors and company activity.

These are parts of one system with separate responsibilities. The 3D office is a view of the company, not where company state is stored or decided.

## Architectural rules

- **PostgreSQL is authoritative** for durable company, employee, and workflow state.
- **Apache AGE provides graph relationships and context.** Graph data complements PostgreSQL and does not become a competing source of truth.
- **IAM is external.** Authentication and credential lifecycle belong to an identity provider, not the domain model.
- **Actor/Employee identity persists.** Agent Runtime is dynamic execution state and may start, stop, or be replaced without changing the employee.
- **The CEO PET is an AI Employee with Position=CEO.** PET/avatar presentation is not the identity record.
- **The 3D office is a projection.** It displays company state and activity; it is not the source of truth.

## Implementation plan

1. **Establish the foundation:** create a clean Rust workspace, infrastructure-independent domain types, and the Office Service boundary.
2. **Make company state durable:** define PostgreSQL schemas and migrations, then implement repositories and Office use cases against them.
3. **Add the service contract:** grow the HTTP API around Office use cases and integrate external IAM at the boundary.
4. **Support agent work:** introduce actor lifecycle, dynamic Agent Runtime, policy, tools, memory, and event-bus capabilities as concrete use cases need them.
5. **Add graph context:** use Apache AGE for relationship and context queries while keeping authoritative records in PostgreSQL.
6. **Build interaction surfaces:** add edge clients and avatar/3D views that consume Office APIs and project company state.
7. **Validate end-to-end:** add integration and system tests around durable state, permissions, task execution, and user-visible workflows.

The sequence is intentional: establish the source of truth and service boundary before adding execution and presentation layers. The workspace areas below mark the intended direction; a reserved directory does not imply that its subsystem is already implemented.

## Current status

The initial implementation is the first foundation milestone:

- `crates/domain` contains infrastructure-independent employee concepts.
- `crates/office-core` defines the Office use-case boundary and repository port.
- `crates/office-api` provides a minimal HTTP health endpoint.
- Planned crates and top-level areas are reserved for later work.

PostgreSQL persistence, migrations, Apache AGE, external IAM, agent execution, and client/3D integrations are **not implemented yet**.

## Planned workspace

Core crates: `domain`, `actor`, `agent-runtime`, `office-core`, `office-api`, `event-bus`, `memory`, `policy`, `tools`, `repository`, `graph`, and `iam`.

Top-level areas: `office/`, `edge/`, `avatar/`, `migrations/`, `docs/`, and `tests/`.

See [docs/architecture.md](docs/architecture.md) for the initial service and data ownership boundaries.
