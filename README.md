[![AGPL License](https://img.shields.io/badge/license-AGPL-blue.svg)](http://www.gnu.org/licenses/agpl-3.0)
# Wagon --- an independent mailing list manager

### How to build:

* Install Cargo using [rustup](https://rustup.rs/) with:
```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

* Clone the repository with:
```
$ git clone https://github.com/realaravinth/wagon
```

* Build with Cargo:
``` 
$ cd auth-backend && cargo build
```

### Usage:

#### Configuration:
Wagon expects the following environment variables to be set:

| Name | Value           | Explanation  |
| ------------- |-------------| -----|
| `WAGON_SMTP_API_KEY`| API key | API key of your SMTP provider* |
| `DATABASE_URL`| Postgres database url |   database url in `postgres://username:password@host/database_name` format |
| `WAGON_RD_URL` | Redis URL      |    Redis database URL |


	*we rely on SMTP providers to send subscriber conformation emails
