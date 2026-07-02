use bollard::Docker;

use super::types::ContainerInfo;
use crate::error::AppError;

pub(crate) async fn do_list_containers(
    docker: &Docker,
) -> Result<Vec<ContainerInfo>, AppError> {
    let options = bollard::query_parameters::ListContainersOptions {
        all: false,
        ..Default::default()
    };

    let containers = docker
        .list_containers(Some(options))
        .await
        .map_err(|e| AppError::DockerOperation(e.to_string()))?;

    Ok(containers
        .into_iter()
        .map(|c| ContainerInfo {
            id: c.id.unwrap_or_default(),
            name: c
                .names
                .unwrap_or_default()
                .first()
                .cloned()
                .unwrap_or_default(),
            image: c.image.unwrap_or_default(),
        })
        .collect())
}
