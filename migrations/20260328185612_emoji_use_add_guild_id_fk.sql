ALTER TABLE
  emoji_uses
ADD CONSTRAINT
  emoji_uses_guild_id_fkey
FOREIGN KEY
  (guild_id)
REFERENCES
  guilds(guild_id)
ON DELETE CASCADE;
