# Practice Log Microservice
An application created for logging practice progress for a specific user, written in Rust, using PostgreSQL DB to save data, and Docker to package the service.

## Installation & Deployment
### Local Development Setup
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