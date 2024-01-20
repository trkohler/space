## Set Up

to start the project, create `Secrets.toml` and add there `DATABASE_URL` & `GOOGLE_CLIENT_ID`.

Then, run `cargo shuttle run --port 8080`.

### Infrastructure

Database is postgres instance.
Google Client ID is provisioned by Google Cloud.

### Endpoints

Service is GraphQL API, with following endpoints:

`/graphql` <- on `GET` you have playground. on `POST` handler for queries.