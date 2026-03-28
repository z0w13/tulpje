ALTER TABLE
  pk_notify_channels
ADD CONSTRAINT
  pk_notify_channels_guild_id_fkey
FOREIGN KEY
  (guild_id)
REFERENCES
  guilds(guild_id)
ON DELETE CASCADE;
