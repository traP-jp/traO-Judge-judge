use uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct DepId {
    id: Uuid,
}

impl DepId {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
        }
    }
}

impl std::ops::Deref for DepId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

impl std::fmt::Display for DepId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}


#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct RuntimeId {
    id: Uuid,
}

impl RuntimeId {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
        }
    }
}

impl std::ops::Deref for RuntimeId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

impl std::fmt::Display for RuntimeId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}


#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct ResourceId {
    id: Uuid,
}

impl ResourceId {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
        }
    }
}

impl std::ops::Deref for ResourceId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

impl std::fmt::Display for ResourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}