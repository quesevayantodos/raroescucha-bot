# Raroescucha Bot

## Setup

Once you have requested a token from the botfather, you must create a `.env` file with the token in the following format
```bash
cat <<EOF > .env
export TELOXIDE_TOKEN="your token goes here"
EOF
```
once you have done that, you can run the bot with the following command

```bash
./run_telegram_bot
```

It is a simple wrapper around `cargo run` that sets the environment variable from the `.env` file, and it also sets the
`RUST_LOG` environment variable to `info` so that you can see the logs of the bot, stored under `log` directory.
