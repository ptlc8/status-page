use shiplift::{ContainerListOptions, Docker};
use std::collections::HashMap;
mod models;
mod state;
use models::{InstanceInfo, ProjectInfo, ServiceInfo};
use state::get_state;

pub async fn get_projects_status() -> Vec<ProjectInfo> {
    let docker = Docker::unix("/var/run/docker.sock");
    let containers = docker
        .containers()
        .list(&ContainerListOptions::builder().all().build())
        .await
        .unwrap();

    let mut instances_by_service_by_project: HashMap<String, HashMap<String, Vec<InstanceInfo>>> =
        HashMap::new();

    for container in containers {
        if let Some(service_name) = container.labels.get("com.docker.compose.service") {
            let project_name = container
                .labels
                .get("com.docker.compose.project")
                .cloned()
                .unwrap_or("default".to_string());
            let state = match container.state.as_str() {
                "running" => "up",
                _ => "down",
            }
            .to_string();
            let number = container
                .labels
                .get("com.docker.compose.container-number")
                .and_then(|n| n.parse().ok())
                .unwrap_or(0);

            let instance_info = InstanceInfo {
                number,
                container_name: container.names[0].clone(),
                status: container.status.clone(),
                state,
            };

            instances_by_service_by_project
                .entry(project_name.clone())
                .or_insert(HashMap::new())
                .entry(service_name.clone())
                .or_insert(Vec::new())
                .push(instance_info);
        }
    }

    let mut projects: Vec<ProjectInfo> = Vec::new();

    for (project_name, instances_by_service) in instances_by_service_by_project {
        let mut services: Vec<ServiceInfo> = Vec::new();
        for (service_name, instances) in instances_by_service {
            let state = get_state(&instances);

            let service_info = ServiceInfo {
                name: service_name,
                instances,
                state,
            };

            services.push(service_info);
        }

        let state = get_state(&services);

        projects.push(ProjectInfo {
            name: project_name,
            services,
            state,
        });
    }

    projects
}
