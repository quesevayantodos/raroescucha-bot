# Raroescucha Bot

## Setup

Once you have requested a token from the botfather, you must create a `.env` file with the token in the following format
```bash
cat <<EOF > .env
# to run directly from the terminal
export TELOXIDE_TOKEN="loremipsum dolor sit amet consectetur adipiscing elit sed do eiusmod"
export DATABASE_URL="sqlite:///data/message.db"
export RUST_LOG="info"
EOF
```
once you have done that, you can run the bot with the following command

```bash
./run_telegram_bot
```

It is a simple wrapper around `cargo run` that sets the environment variable from the `.env` file, and it also sets the
`RUST_LOG` environment variable to `info` so that you can see the logs of the bot, stored under `log` directory.

## Running the container with podman

```bash
cat <<EOF > .env
# to run as a container
TELOXIDE_TOKEN=loremipsum dolor sit amet consectetur adipiscing elit sed do eiusmod
DATABASE_URL=sqlite:///data/message.db
RUST_LOG=info
EOF
```
