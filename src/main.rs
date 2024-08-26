#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let swarm_status = docker_api::get_swarm_status().await;
    println!("{:?}", swarm_status);
}

mod docker_api {
    use bollard::Docker;

    #[derive(Debug)]
    pub enum SwarmStatus {
        Active,
        Inactive,
    }

    pub async fn get_swarm_status() -> SwarmStatus {
        let docker_client = Docker::connect_with_local_defaults();

        match docker_client {
            Ok(client) => {
                let version = client.info().await.unwrap().swarm;
                println!("{:?}", version);
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        }

        SwarmStatus::Active
    }
}

#[cfg(test)]
mod tests {
    use crate::docker_api;

    #[tokio::test]
    async fn should_get_swarm_status() {
        let swarm_status = docker_api::get_swarm_status().await;
        println!("{:?}", swarm_status);
    }
}
