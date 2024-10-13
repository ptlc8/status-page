use shiplift::Docker;
use serde::Serialize;

#[derive(Serialize)]
pub struct ContainerInfo {
    pub name: String,
    pub status: String,
}

pub async fn get_containers_status() -> Vec<ContainerInfo> {
    let docker = Docker::unix("/var/run/docker.sock");
    let containers = docker.containers().list(&Default::default()).await.unwrap();

    let mut container_infos = Vec::new();

    for container in containers {
        let container_info = ContainerInfo {
            name: container.names[0].clone(),
            status: container.status.clone(),
        };
        container_infos.push(container_info);
    }

    container_infos
}