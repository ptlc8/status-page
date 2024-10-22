use serde::Serialize;
use shiplift::{ContainerListOptions, Docker};
use std::collections::HashMap;

#[derive(Serialize)]
pub struct ServiceInfo {
    pub name: Option<String>,
    pub container_name: String,
    pub up: bool,
    pub status: String,
}
#[derive(Serialize)]
pub struct ProjectInfo {
    pub name: String,
    pub services: Vec<ServiceInfo>,
    pub up: bool,
}

pub async fn get_projects_status() -> Vec<ProjectInfo> {
    let docker = Docker::unix("/var/run/docker.sock");
    let containers = docker.containers().list(&ContainerListOptions::builder().all().build()).await.unwrap();

    let mut services_by_project: HashMap<String, Vec<ServiceInfo>> = HashMap::new();

    for container in containers {
        if let Some(project_name) = container.labels.get("com.docker.compose.project") {
            let service_info = ServiceInfo {
                name: container.labels.get("com.docker.compose.service").cloned(),
                container_name: container.names[0].clone(),
                up: container.state == "running",
                status: container.status.clone(),
            };
            services_by_project
                .entry(project_name.clone())
                .or_insert(Vec::new())
                .push(service_info);
        }
    }

    let mut projects: Vec<ProjectInfo> = Vec::new();

    for (project_name, services) in services_by_project {
        let up = services.iter().all(|service| service.up);

        projects.push(ProjectInfo {
            name: project_name,
            services,
            up,
        });
    }

    projects
}
