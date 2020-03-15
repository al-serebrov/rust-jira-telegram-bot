# JIRA Telegram Bot

## What it does?
The bot is created to send the updates from JIRA board to the Telegram. For now, the update is triggered manually - you need to send a message with any text to bot, and it replies with the list of tickets that matches the query from configuration file (see configuration section).

## Installation

To install the bot you need to clone this repository and build the rust code, so make sure that you have both git and rust (with cargo) installed.

```bash
git clone https://github.com/al-serebrov/rust-jira-telegram-bot
cd rust-jira-telegram-bot
cargo build
```

## Configuration

The bot is configured via environmental variables, there is a sample of configuration file `config.env.sample` which you can use to store your variables.

To configure the bot you need have set the following environmental variables:

### JIRA-related variables
```
JIRA_USER
JIRA_PASS
JIRA_HOST
JIRA_QUERY
```

`JIRA_HOST` is a URL of your JIRA instance.

`JIRA_USER` and `JIRA_PASS` variables should contain your credentials to the JIRA, depending on JIRA configuration, you may need to generate an API Token and use it instead the password, please refer to the official [Atlassian documentation](https://confluence.atlassian.com/cloud/api-tokens-938839638.html).

`JIRA_QUERY` variable should contain a query written in JIRA Query Language (JQL), please see [the official documentation for JQL](https://www.atlassian.com/blog/jira-software/jql-the-most-flexible-way-to-search-jira-14).

### Telegram bot related variables

Please add your telegram bot token to `TELEGRAM_BOT_TOKEN` environmental variable. How to create new telegram bot - refer to the [official Telegram documentation](https://core.telegram.org/bots).

## Usage

Set all needed environmental variables or use an `.env` file as a source of them and run the bot:
```bash
source config.env
cargo run
```

## Important note

For now the bot doesn't have any security level, so if you run it, any Telegram user may find it and see the list of issues from you JIRA board (for the configured query of course). I'm going to implement some security to the bot, so it sends messages only to configured group of Telegram users.

Consider the functionality of the bot as a demo.

## Dependencies

The bot depends on two crates:
- [telegram-bot](https://github.com/telegram-rs/telegram-bot)
- [goji](https://github.com/softprops/goji)
