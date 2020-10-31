<div align="center">
<img src="assets/wagon.svg" alt="Wagon" width="200" height="180" />

# Wagon

[![AGPL License](https://img.shields.io/badge/license-AGPL-blue.svg)](http://www.gnu.org/licenses/agpl-3.0)
[![Build Status](https://travis-ci.com/realaravinth/wagon.svg?branch=master)](https://travis-ci.com/realaravinth/wagon)
[![codecov](https://codecov.io/gh/realaravinth/wagon/branch/master/graph/badge.svg)](https://codecov.io/gh/realaravinth/wagon)

</div>

**Wagon** is a membership management platform. Users can 
advertise their groups for people to join them.

### Motivation
Currently, we are focussing on building a platform to collect members'
emails but we plan on expanding to provide a [free
software](https://www.gnu.org/philosophy/free-sw.html) alternative to
platforms like [eventbrite](https://www.eventbrite.com/).

### How to build:

- Install Cargo using [rustup](https://rustup.rs/) with:

```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- Clone the repository with:

```
$ git clone https://github.com/realaravinth/wagon
```

- Build with Cargo:

```
$ cd wagon && cargo build
```

### Usage:

#### Configuration:

Wagon expects the following environment variables to be set:

| Name                 | Value                 | Explanation                                                              |
| -------------------- | --------------------- | ------------------------------------------------------------------------ |
| `WAGON_SMTP_API_KEY` | API key               | API key of your SMTP provider\*                                          |
| `DATABASE_URL`       | Postgres database url | database url in `postgres://username:password@host/database_name` format |
| `WAGON_PG_POOL_SIZE` | integer               | postgres database connection pool size                                   |
| `WAGON_RD_URL`       | Redis URL             | Redis database URL                                                       |

*we rely [SMTP2Go's](https://www.smtp2go.com/) HTTP API to send emails.
This will change when the author comes into some money and sets up their
own email server.


### Credits:
Logo  made by <a href="https://smashicons.com/"
title="Smashicons">Smashicons</a> from <a
href="https://www.flaticon.com/" title="Flaticon">
www.flaticon.com</a>. Do check them out!
