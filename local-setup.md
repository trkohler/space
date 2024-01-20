# Infrastructure

For my own development purposes I use postgres instance, provisioned by Neon. [Neon console](https://console.neon.tech)

Google Client ID is provision by Google Cloud. [Credentials – APIs & Services – Booking Space App – Google Cloud console](https://console.cloud.google.com/apis/credentials)

## Check pending migrations & health of the schema

Make `.env` file in migration folder. It should contain `DATABASE_URL` variable.

```bash
DATABASE_URL=postgres://postgres:postgres@localhost:5432/booking_space
```

Then run in migration folder:

```bash
cargo run -- status
```