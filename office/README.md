# Office Service

The Office Service owns the authoritative company organization and work state.

A company may organize connected satellite Offices beneath a parent Office. Each Office has a stable identity and an explicit parent relationship where applicable. A satellite may be headed by a Sub-CEO or VP whose Employee/Actor position is scoped to that Office. Leadership can be held by a human or AI Employee according to policy.

Sub-CEO/VP is an organizational leadership role, not automatic Owner status or unrestricted authorization. The service must model delegated scope and enforce permissions independently from position titles.

PostgreSQL remains authoritative for Office records, hierarchy, employee positions, and delegated assignments. Apache AGE may provide relationship/context queries as a projection. Desktop, avatar, and 3D office views display this structure through Office APIs; they do not own it.

This is a planned boundary. Persistent Office hierarchy, leadership assignments, delegated authorization, and satellite operations are not implemented yet. See [the architecture notes](../docs/architecture.md) for the initial rules.
