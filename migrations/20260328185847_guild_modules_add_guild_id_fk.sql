ALTER TABLE
  guild_modules
ADD CONSTRAINT
  guild_modules_guild_id_fkey
FOREIGN KEY
  (guild_id)
REFERENCES
  guilds(guild_id)
ON DELETE CASCADE;
