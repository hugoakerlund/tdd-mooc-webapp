# [TDD MOOC](https://tdd.mooc.fi): Full-stack web app

_This exercise is part of the [TDD MOOC](https://tdd.mooc.fi) at the University of Helsinki, brought to you
by [Esko Luontola](https://twitter.com/EskoLuontola) and [Nitor](https://nitor.com/)._

## Developing

Build and start all database

    docker compose up -d database

Start the database

    cd backend
    cargo run

Start the frontend

    cd frontend
    npm start

The web app will be available at http://localhost:3000

Run backend and database tests

    cd backend
    cargo test -- --test-threads 1

Run frontend tests

    cd frontend
    npm test

Run end-to-end tests

    ./end-to-end-test.sh

Destroy all containers

    docker compose down

