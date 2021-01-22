# Practice Log Microservice
An application created for logging practice progress for a specific user, written in Rust, using PostgreSQL DB to save data, and Docker to package the service.

## TODO
I have still to do the following:
1. Write complete Dockerfiles
2. Test & Tweak the setup script
3. Rewrite DB table
4. Create structs for deserializing JSON payload data, and writing to db (currently, data is just taken as bytes and converted to a string from the call to the /write route)
