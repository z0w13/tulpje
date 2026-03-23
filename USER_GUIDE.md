# Tulpje User Guide

## Invite The Bot
Tulpje can be invited to your server using the following link: [Add To Discord](https://discord.com/oauth2/authorize?client_id=1220754275530051605)

## Permissions
What permissions the bot requests and why

* **View Channels:** required to do anything with channels
* **Manage Channels:** needed to set up fronter category and notification channel
* **Create/Manage Expressions:** needed to create emojis
* **Send Messages:** bot responses and front change notifications
* **Read Message History:** used to track emoji usage in messages

## Commands

### Emojis

* `/emoji clone` • copy the specified emojis to this server
* `/emoji stats` • shows usage stats of this server's emojis (in this server)
* `/emoji maintenance` • manual cleanup of deleted emojis

### PluralKit
To make use of any of the [PluralKit](https://pluralkit.me/) modules you have to
set it up first using `/pk setup`

* `/pk setup` • configures the bot with your PluralKit system

#### Fronter Category
Set up a category in the current server that displays the fronters for the
system specified in `/pk setup`

* `/pk fronters setup` • configure the fronters category
* `/pk fronters update` • manually trigger an update of the fronter list

#### Front Notifications
Allows you to follow systems and have notifications sent in a channel in the current
server whenever a system's fronters change

* `/pk notify setup` • configure the notification channel
* `/pk notify list` • list systems you're currently following
* `/pk notify add` • add a system to be notified of
* `/pk notify remove` • remove a system from the notification list

#### System Member Roles
Creates a role for each system member in this server, and assigns them to the user
that ran `/pk setup` to allow mentioning individual system members

> **⚠️ Warning ⚠️**<br>
> Currently there's no way to delete the created roles
> so if you end up disliking them it might take some effort to remove them

* `/pk roles update` • synchronises the member roles to your system member list.\
  You can optionally specify a token to also use private members.

### Other Commands

* `/stats` • show bot statistics
* `/info processes` • detailed bot process stats
* `/info shards` • detailed bot shard stats
