<div align="center">
<img src="assets/wagon.svg" alt="Wagon" width="200" height="180" />

# Wagon

[![AGPL License](https://img.shields.io/badge/license-AGPL-blue.svg)](http://www.gnu.org/licenses/agpl-3.0)
[![Build Status](https://travis-ci.com/realaravinth/wagon.svg?branch=master)](https://travis-ci.com/realaravinth/wagon)
[![codecov](https://codecov.io/gh/realaravinth/wagon/branch/master/graph/badge.svg)](https://codecov.io/gh/realaravinth/wagon)

</div>

### STATUS: Active development (fancy word for unusable)

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

Wagon is highly configurable. 
Configuration is applied/merged in the following order:

1. `config/default.toml`
2. `config/$WAGON_MODE.toml`
3. environment variables.

##### Environment variables:

| Name                         | Value                                                       |
| ---------------------------- | ----------------------------------------------------------- |
| `WAGON_MODE`                 | Run mode for choosing configuration(development/production) |
| `WAGON_SMTP_KEY`             | API key                                                     |
| `WAGON_POSTGRES_PASSWORD`    | Postgres password                                           |
| `WAGON_POSTGRES_NAME`        | Postgres database name                                      |
| `WAGON_POSTGRES_PORT`        | Postgres port                                               |
| `WAGON_POSTGRES_HOSTNAME`    | Postgres hostmane                                           |
| `WAGON_POSTGRES_USERNAME`    | Postgres username                                           |
| `WAGON_POSTGRES_POOL`        | Postgres database connection pool size                      |
| `WAGON_PORT` (or) `PORT`\*\* | The port on which you want wagon to listen to               |
| `WAGON_IP`                   | The IP address on which you want wagon to listen to         |

\*we rely [SMTP2Go's](https://www.smtp2go.com/) HTTP API to send emails.
This will change when the author comes into some money and sets up their
own email server.

\*\*Heroku uses `$PORT`, we got you covered :)

### Credits:

Logo made by <a href="https://smashicons.com/"
title="Smashicons">Smashicons</a> from <a
href="https://www.flaticon.com/" title="Flaticon">
www.flaticon.com</a>. Do check them out!
