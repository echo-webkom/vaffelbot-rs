# Vaffelbot

## How to install

https://discord.com/oauth2/authorize?client_id=1367651749262921868&permissions=277025458240&integration_type=0&scope=bot

## How to run

Make sure that you have copied `.env.example` to `.env` and filled in the missing enviornment variables.

| Variable        | Description                                                           |
| --------------- | --------------------------------------------------------------------- |
| `DISCORD_TOKEN` | The discord token. Used in authenticate requests                      |
| `GUILD_ID`      | The guild id is the id of the server where the bot is supposed to run |
| `REDIS_URL`     | The url to the redis (valkey) database                                |

When this is done you can just run the bot with:

```sh
cargo run
```

## How to test

You can test the application with `cargo test -- --test-threads=1`. Remember to
have docker running.

## Documentation

### Commands

#### `/stekt`

**Must be an oracle to use**

Bakes _n_ amount of waffles, and pings the _n_ next people in the queue.

#### `/stopp`

**Must be an oracle to use**

Stops the queue

#### `/start`

**Must be an oracle to use**

Starts the queue

#### `/ping`

Pings the bot to see if it is alive. Basic healthcheck.

#### `/kø`

Checks the where in the queue the person who ran the command is.

#### `/vaffel`

Adds the user who ran the command to the queue. If the user is already in the queue, it will just print the position.
