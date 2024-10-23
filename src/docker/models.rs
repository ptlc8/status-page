use super::state::HasState;
use serde::Serialize;

#[derive(Serialize)]
pub struct InstanceInfo {
    pub number: u32,
    pub status: String,
    pub state: String,
}

impl HasState for InstanceInfo {
    fn get_state(&self) -> &String {
        &self.state
    }
}

#[derive(Serialize)]
pub struct ServiceInfo {
    pub name: String,
    pub instances: Vec<InstanceInfo>,
    pub state: String,
}

impl HasState for ServiceInfo {
    fn get_state(&self) -> &String {
        &self.state
    }
}

#[derive(Serialize)]
pub struct ProjectInfo {
    pub name: String,
    pub services: Vec<ServiceInfo>,
    pub state: String,
}

impl HasState for ProjectInfo {
    fn get_state(&self) -> &String {
        &self.state
    }
}
