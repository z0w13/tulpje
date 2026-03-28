-- begin the transaction
BEGIN TRANSACTION;

-- add the column
ALTER TABLE pk_guilds ADD COLUMN system_uuid UUID;

-- populate it with data from pk_guilds
UPDATE
  pk_guilds
SET
  system_uuid = pk_systems.uuid
FROM
  pk_systems
WHERE
  pk_guilds.system_id = pk_systems.id;

-- make column non null
ALTER TABLE pk_guilds ALTER COLUMN system_uuid SET NOT NULL;

-- add foreign key constraint
ALTER TABLE
  pk_guilds
ADD CONSTRAINT
  pk_guilds_system_uuid_fkey
FOREIGN KEY
  (system_uuid)
REFERENCES
  pk_systems(uuid)
ON DELETE CASCADE;

-- drop old column
ALTER TABLE pk_guilds DROP COLUMN system_id;

-- commit the changes
COMMIT TRANSACTION;
