use uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct DepId {
    id: Uuid,
}

impl DepId {
    pub fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }
}

impl std::fmt::Display for DepId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl From<DepId> for Uuid {
    fn from(dep_id: DepId) -> Self {
        dep_id.id
    }
}

impl From<Uuid> for DepId {
    fn from(id: Uuid) -> Self {
        Self { id }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct RuntimeId {
    id: Uuid,
}

impl RuntimeId {
    pub fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }
}

impl std::fmt::Display for RuntimeId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl From<RuntimeId> for Uuid {
    fn from(runtime_id: RuntimeId) -> Self {
        runtime_id.id
    }
}

impl From<Uuid> for RuntimeId {
    fn from(id: Uuid) -> Self {
        Self { id }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct ResourceId {
    id: Uuid,
}

impl ResourceId {
    pub fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }
}

impl std::fmt::Display for ResourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl From<ResourceId> for Uuid {
    fn from(resource_id: ResourceId) -> Self {
        resource_id.id
    }
}

impl From<Uuid> for ResourceId {
    fn from(id: Uuid) -> Self {
        Self { id }
    }
}
