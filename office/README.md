# Office Service

The Office Service owns the authoritative company organization and work state.

A company may organize connected satellite Offices beneath a parent Office. Each Office has a stable identity and an explicit parent relationship where applicable. A satellite may be headed by a Sub-CEO or VP who reports directly to the company CEO. The Sub-CEO/VP position is scoped to that satellite and may be held by a human or AI Employee according to policy.

The CEO may assign the satellite leader responsibility for selected departments and project teams. Each assignment has an explicit scope and can be changed or revoked through governed Office actions. A leadership assignment covers only the named department(s) or project team(s); it does not imply company-wide authority, Owner status, or unrestricted access.

PostgreSQL remains authoritative for Office records, hierarchy, employee positions, reporting lines, and delegated assignments. Apache AGE may provide relationship/context queries as a projection. Desktop, avatar, and 3D office views display this structure through Office APIs; they do not own it.

This is a planned boundary. Persistent Office hierarchy, leadership assignments, delegated authorization, and satellite operations are not implemented yet. See [the architecture notes](../docs/architecture.md) for the initial rules.
