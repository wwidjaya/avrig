//! Core company concepts. Infrastructure and runtime concerns live outside this crate.

use std::fmt;

/// Stable identifier for an employee or other company actor.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ActorId(String);

impl ActorId {
    pub fn new(value: impl Into<String>) -> Result<Self, DomainError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(DomainError::EmptyIdentifier);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ActorId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// A position is an organizational role, separate from execution runtime.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Position(String);

impl Position {
    pub fn new(value: impl Into<String>) -> Result<Self, DomainError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(DomainError::EmptyPosition);
        }
        Ok(Self(value))
    }

    pub fn title(&self) -> &str {
        &self.0
    }
}

/// Persistent company actor. Whether an agent runtime is currently running is
/// intentionally not represented here.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Employee {
    id: ActorId,
    display_name: String,
    position: Position,
    is_ai: bool,
}

impl Employee {
    pub fn new(
        id: ActorId,
        display_name: impl Into<String>,
        position: Position,
        is_ai: bool,
    ) -> Result<Self, DomainError> {
        let display_name = display_name.into();
        if display_name.trim().is_empty() {
            return Err(DomainError::EmptyDisplayName);
        }
        Ok(Self { id, display_name, position, is_ai })
    }

    pub fn id(&self) -> &ActorId { &self.id }
    pub fn display_name(&self) -> &str { &self.display_name }
    pub fn position(&self) -> &Position { &self.position }
    pub fn is_ai(&self) -> bool { self.is_ai }
}

/// Canonical CEO PET representation: an AI employee with the CEO position.
pub fn ceo_pet(id: ActorId, display_name: impl Into<String>) -> Result<Employee, DomainError> {
    Employee::new(id, display_name, Position::new("CEO")?, true)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DomainError {
    EmptyIdentifier,
    EmptyPosition,
    EmptyDisplayName,
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier => f.write_str("identifier must not be empty"),
            Self::EmptyPosition => f.write_str("position must not be empty"),
            Self::EmptyDisplayName => f.write_str("display name must not be empty"),
        }
    }
}

impl std::error::Error for DomainError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ceo_pet_is_an_ai_employee_with_ceo_position() {
        let employee = ceo_pet(ActorId::new("employee-1").unwrap(), "Ava").unwrap();
        assert!(employee.is_ai());
        assert_eq!(employee.position().title(), "CEO");
    }

    #[test]
    fn actor_identity_is_stable_and_runtime_is_not_part_of_employee() {
        let id = ActorId::new("employee-1").unwrap();
        let employee = Employee::new(id.clone(), "Ava", Position::new("CEO").unwrap(), true).unwrap();
        assert_eq!(employee.id(), &id);
    }
}
