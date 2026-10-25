# avrig

Agentic virtual company engineering system.

This repository starts with a Rust workspace and an Office Service boundary. The initial implementation deliberately keeps persistence and external integrations behind ports so the domain can evolve without assuming that a UI or runtime owns company state.

## Architecture rules

- PostgreSQL is authoritative for durable company and workflow state.
- Apache AGE is a graph projection for relationships and context; it does not replace PostgreSQL.
- Identity and authentication are provided by an external IAM system.
- An Actor/Employee has persistent identity. Agent Runtime is an execution environment that may be created or stopped dynamically.
- The CEO PET is an AI Employee whose position is CEO.
- The 3D office is a projection of company state, never its source of truth.

See [docs/architecture.md](docs/architecture.md) for the initial boundaries.

## Workspace

- `crates/domain`: identity and company concepts, without infrastructure dependencies.
- `crates/office-core`: Office application service and persistence port.
- `crates/office-api`: HTTP adapter for the Office service.
- Remaining agreed crates and top-level areas are reserved for later implementation.
