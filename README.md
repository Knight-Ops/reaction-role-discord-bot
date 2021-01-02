# reaction-role-discord-bot

Super simple, reaction-based, discord bot for assigning roles to members of a server

## Usage
To use, build the Dockerfile

`docker build -t APPNAME .`

Run the container with `DISCORD_TOKEN` envvar set

`docker run -e DISCORD_TOKEN=SUPER_SECRET_TOKEN_HERE --rm -d APPNAME`