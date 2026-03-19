# Tulpje

Unnecessarily complicated rewrite of a multi-purpose discord bot

## Public Instance
Tulpje can be invited to your server using the following link: [Add To Discord](https://discord.com/oauth2/authorize?client_id=1220754275530051605)

### PluralKit Fronters Setup
*in the server you want to enable it in*

1. `/mod enable module:pluralkit`
2. `/pk setup system_id:<your system id>`
3. `/pk fronters setup name:<fronter category name>`
4. Tadaaa

## Components

### Gateway

Receives [Gateway Events](https://discord.com/developers/docs/events/gateway-events) from discord and publishes them onto an AMQP queue.

Also handles storing shard statistics.

### Handler

The main "bot" component of Tulpje, this is where all the commands, event handlers, etc. live.

Works by connecting to an AMQP queue and listening for for Discord [Gateway Events](https://discord.com/developers/docs/events/gateway-events).

### Manager

Intended to be the component that manages (re)sharding, currently just returns
the recommended shard count from Discord.

### Framework

The bot framework used by `tulpje-handler` to handle events, commands, etc.

### Shared

Things shared between different parts of the bot.
