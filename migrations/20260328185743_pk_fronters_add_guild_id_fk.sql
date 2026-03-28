ALTER TABLE
  pk_fronters
ADD CONSTRAINT
  pk_fronters_guild_id_fkey
FOREIGN KEY
  (guild_id)
REFERENCES
  guilds(guild_id)
ON DELETE CASCADE;
