# Practice Log Microservice
An application created for logging practice progress for a specific user, written in Rust, using PostgreSQL DB to save data, and Docker to package the service.

## Installation & Deployment
### Local Development Setup
1. Run the docker-compose file in `db/` using `cd db && docker-compose up -d`
2. Run `setup-local.bash` using `bash setup-local.bash`
3. You should be ready to start working with the code, as the DB backend would be set up!

### Deploy Dockerized DB & MS
1. Navigate to `practice-rs/`
2. Run `bash deploy-docker-stack.sh`
3. Verify both db and ms containers are up and running.
4. Test with `curl localhost:3001/health`