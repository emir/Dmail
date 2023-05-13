# Dmail

This project provides a simple Discord bot named Dmail that fetches new emails from Gmail and sends them to a specified Discord channel.

## Prerequisites

- Rust 1.54.0 or later
- Access to the Gmail API
- A Discord bot token
- The Serenity, yup-oauth2, reqwest, and dotenv crates

## Setup

1. First, clone this repository to your local machine using `git clone https://github.com/emir/Dmail.git`.

2. Install the required Rust crates by running `cargo install`.

3. Set up the Gmail API:
    - Follow the instructions [here](https://developers.google.com/gmail/api/quickstart/go) to set up a project in the Google Cloud Console, enable the Gmail API, and download the `client_secret.json` file.
    - Place the `client_secret.json` file in the root directory of the project.

4. Set up the Discord bot:
    - Follow the instructions [here](https://discordpy.readthedocs.io/en/stable/discord.html) to create a bot on the Discord Developer Portal and get your bot token.
    - Make sure to invite the bot to your Discord server.

5. Set the following environment variables in a `.env` file in the root directory of the project:
    - `SECRET_FILE`: The path to your `client_secret.json` file.
    - `DISCORD_TOKEN`: Your Discord bot token.
    - `CHANNEL_ID`: The ID of the Discord channel where you want the bot to send emails.

6. Finally, run `cargo run` to start the bot.

## Usage

Once started, the bot will automatically fetch new emails from the Gmail account associated with the `client_secret.json` file and send them to the specified Discord channel.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project licensed under MIT.