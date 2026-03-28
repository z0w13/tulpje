ALTER TABLE
  pk_guilds
ADD CONSTRAINT
  pk_guilds_guild_id_fkey
FOREIGN KEY
  (guild_id)
REFERENCES
  guilds(guild_id)
ON DELETE CASCADE;
