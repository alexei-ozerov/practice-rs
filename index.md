# Practice Log Microservice
An application created for logging practice progress for a specific user, written in Rust, using PostgreSQL DB to save data, and Docker to package the service.

## Design & Goals
The application will consist of three parts:
1. WASM Front-End built with Yew
2. Rust Server using Hyper
3. Postgresql Database

A user may track their practice data (be it an instrument, a composing practice, etc.), and then view their data over time, tracking trends, progress, and external factors that may contribute to their feelings of productivity or unfocus.

## Installation & Deployment
### Local Development Setup
#### Dependencies
1. Docker: https://www.docker.com/
2. Docker-Compose: https://docs.docker.com/compose/install/
3. Diesel: https://diesel.rs/

#### Instructions
1. Edit values `POSTGRES_USER` and `POSTGRES_PASSWORD` in in `db/docker-compose.yaml`
2. Run the docker-compose file in `db/` using `docker-compose up -d`
3. Navigate to `ms/logging-ms/` and run `diesel setup && diesel migration`
4. Navigate to `practice-rs/` and run `bash setup-local.bash <DB USER> <DB PASS> <DB ADDR (localhost should work)> <PORT (5433 from the docker-compose file)> <DB Name>`
5. You should be ready to start working with the code, as the DB backend would be set up! Navigate to `ms/logging-ms` and run `cargo run` to bring up the code.
6. Test with `curl 0.0.0.0:3000/health`, if response is successful, you are good to go.
7. Run `docker-compose down` in `db/` before deploying full Dockerized stack.

### Deploy Dockerized DB & MS
1. Navigate to `practice-rs/`
2. Run `bash deploy-docker-stack.sh`
3. Verify both db and ms containers are up and running.
4. Test with `curl localhost:3001/health`
5. Shut down stack with `docker-compose down` in `practice-rs/`

### Kubernetes Deployment
NOTE: The deployment files are currently in-development, and not ready to be used. The instructions to stand up a K8s Local Cluster are valid, however, the deployment files will need to be configured before they can be used. Please build the images locally, push to the local repository that will come up with the cluster, and deploy using your own means for now. This message will be removed once the deployment .yaml files are tested and working.

#### Dependencies
1. KinD: https://kind.sigs.k8s.io/docs/user/quick-start/

#### Instructions
1. Navigate to `practice-rs`
2. Run `bash manage-kind-cluster.sh create`
3. Build and push images to local registry on localhost:5000
4. Deploy to Kubernetes cluster as desired.
