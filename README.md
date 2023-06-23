## Set Up

to start the project, create `Secrets.toml` and add there `DATABASE_URL`.

Then, run `cargo shuttle run`.

### Database

Database is postgres instance, provisioned by Neon.


### Endpoints

Service is GraphQL API, with following endpoints:

`/graphql` <- on `GET` you have playground. on `POST` handler for queries.