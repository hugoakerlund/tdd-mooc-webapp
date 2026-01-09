# [TDD MOOC](https://tdd.mooc.fi): Full-stack web app

This is a project template for writing a full-stack web app. Here is just some basic Docker and Nginx configuration. The
rest is up to you. Just develop everything using TDD.

You should use the [walking skeleton](https://tdd.mooc.fi/5-advanced#walking-skeleton) approach to start developing the
application. Even this project template, empty though it may be, was done that way.

Focus on writing tests on every level of the stack:

- use unit tests to cover as much of the code as is possible to unit test
- also unit test the user interface components ([visual testing](https://tdd.mooc.fi/3-challenges#visual-testing) is
  optional)
  - tests for the UI components should not depend on the API
- use focused integration tests for the database and API layers
  - tests for the API (request routing and validation) should not depend on the database
  - tests for the database should not depend on the API
- write only one end-to-end test which requires a fully deployed application (e.g. Docker containers running locally) to
  make sure that things are wired together correctly (start with this -
  see [walking skeleton](https://tdd.mooc.fi/5-advanced#walking-skeleton))

---

_This exercise is part of the [TDD MOOC](https://tdd.mooc.fi) at the University of Helsinki, brought to you
by [Esko Luontola](https://twitter.com/EskoLuontola) and [Nitor](https://nitor.com/)._

## Prerequisites

You'll need [Docker](https://www.docker.com/) and whatever other tools you decide to use.

## Developing

Start the database

    docker compose up -d db

Build and start all containers

    docker compose up -d --build

Destroy all containers and data

    docker compose down

Run end-to-end tests

    ./end-to-end-test.sh
