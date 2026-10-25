//! Office Service application boundary.
//!
//! This crate owns use cases and depends on a repository port. A PostgreSQL
//! adapter will implement the port; AGE and 3D views remain projections.

use avrig_domain::{ActorId, Employee};
use std::fmt;

pub trait EmployeeRepository {
    type Error;

    /// Read the authoritative employee record from PostgreSQL-backed storage.
    fn find_by_id(&self, id: &ActorId) -> Result<Option<Employee>, Self::Error>;
}

#[derive(Debug, Eq, PartialEq)]
pub enum OfficeError<E> {
    Repository(E),
}

impl<E: fmt::Display> fmt::Display for OfficeError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Repository(error) => write!(f, "employee repository: {error}"),
        }
    }
}

impl<E: fmt::Debug + fmt::Display> std::error::Error for OfficeError<E> {}

pub struct OfficeService<R> {
    employees: R,
}

impl<R> OfficeService<R>
where
    R: EmployeeRepository,
{
    pub fn new(employees: R) -> Self {
        Self { employees }
    }

    pub fn employee(&self, id: &ActorId) -> Result<Option<Employee>, OfficeError<R::Error>> {
        self.employees.find_by_id(id).map_err(OfficeError::Repository)
    }
}
