# Initial architecture boundaries

## Company state and projections

PostgreSQL is the authoritative store for employees, organizational positions, workflows, and other durable company records. The Office Service will own application use cases and access durable state through repository ports.

Apache AGE is reserved for graph relationships and context queries. Graph data is derived or synchronized from authoritative records; AGE must not become a competing authority for employee or company state.

The 3D office, avatar views, and other clients render projections of company state. They do not create the source of truth.

## Identity and execution

IAM is external and owns authentication and credential lifecycle. The domain receives verified actor identity at the API boundary; it must not implement password handling or become an identity provider.

An Actor/Employee has persistent identity and organizational position. Agent Runtime is dynamic execution: it can start, stop, or be replaced without changing the employee identity.

The CEO PET is represented as an AI Employee with Position=CEO. PET/avatar presentation is not the employee record itself.

## Initial code boundary

- `avrig-domain` holds infrastructure-independent employee concepts.
- `avrig-office-core` holds Office use cases and a repository port.
- `avrig-office-api` is a thin HTTP adapter and currently exposes only health.
- PostgreSQL, AGE, IAM, event bus, runtime, and avatar integrations are not implemented yet.
