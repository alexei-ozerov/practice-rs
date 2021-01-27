# Practice Log Microservice
An application created for logging practice progress for a specific user, written in Rust, using PostgreSQL DB to save data, and Docker to package the service.

## Installation & Deployment
### Local Development Setup
1. Run the docker-compose file in `db/` using `docker-compose up -d`
2. Navigate to `ms/logging-ms`
3. Run `diesel setup && diesel migration`
4. You should be ready to start working with the code, as the DB backend would be set up! Navigate to `ms/logging-ms` and run `cargo run` to bring up the code.
5. Test with `curl 0.0.0.0:3000/health`, if response is successful, you are good to go.
6. Run `docker-compose down` in `db/` before deploying full Dockerized stack.

### Deploy Dockerized DB & MS
1. Navigate to `practice-rs/`
2. Run `bash deploy-docker-stack.sh`
3. Verify both db and ms containers are up and running.
4. Test with `curl localhost:3001/health`
5. Shut down stack with `docker-compose down` in `practice-rs/`