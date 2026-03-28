ALTER TABLE
  pk_notify_systems
ADD CONSTRAINT
  pk_notify_systems_guild_id_fkey
FOREIGN KEY
  (guild_id)
REFERENCES
  guilds(guild_id)
ON DELETE CASCADE;
