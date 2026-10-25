# Initial architecture boundaries

## Company state and projections

PostgreSQL is the authoritative store for Offices, their parent/satellite relationships, employees, organizational positions, workflows, and other durable company records. The Office Service will own application use cases and access durable state through repository ports.

Apache AGE is reserved for graph relationships and context queries. Graph data is derived or synchronized from authoritative records; AGE must not become a competing authority for employee or company state.

The 3D office, avatar views, and other clients render projections of company state. They do not create the source of truth.

## Office organization and satellite offices

A company may contain a primary Office and one or more connected satellite Offices. A satellite is a first-class Office with its own stable identity and an explicit parent Office relationship. The relationship represents the company structure; it does not imply a separate identity provider or a second source of truth.

Each satellite Office may be headed by a Sub-CEO or VP. This is a persistent organizational position assigned to an Employee/Actor and scoped to that satellite. The role may be held by a human or AI Employee according to company policy. The Office model should support leadership delegation and reporting between the parent and satellite while preserving the satellite's explicit scope.

Leadership position and ownership/authorization are separate concepts: being a Sub-CEO/VP does not automatically grant Owner status or unrestricted access. Office Service must enforce any delegated permissions, and clients should display the parent/satellite structure as a projection of that authoritative state.

## Identity and execution

IAM is external and owns authentication and credential lifecycle. The domain receives verified actor identity at the API boundary; it must not implement password handling or become an identity provider.

An Actor/Employee has persistent identity and organizational position. Agent Runtime is dynamic execution: it can start, stop, or be replaced without changing the employee identity.

The CEO PET is represented as an AI Employee with Position=CEO. PET/avatar presentation is not the employee record itself. Sub-CEO/VP satellite leadership is likewise represented as an organizational position, distinct from the avatar and runtime.

## Initial code boundary

- `avrig-domain` holds infrastructure-independent employee concepts.
- `avrig-office-core` holds Office use cases and a repository port.
- `avrig-office-api` is a thin HTTP adapter and currently exposes only health.
- PostgreSQL, AGE, IAM, event bus, runtime, and avatar integrations are not implemented yet.
