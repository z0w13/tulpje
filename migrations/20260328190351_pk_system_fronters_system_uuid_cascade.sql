BEGIN TRANSACTION;

-- drop the original foreign key constraint
ALTER TABLE
  pk_system_fronters
DROP CONSTRAINT
  pk_system_fronters_system_uuid_fkey;

-- add the new one with ON DELETE CASCADE
ALTER TABLE
  pk_system_fronters
ADD CONSTRAINT
  pk_system_fronters_system_uuid_fkey
FOREIGN KEY
  (system_uuid)
REFERENCES
  pk_systems(uuid)
ON DELETE CASCADE;

-- persist changes
COMMIT TRANSACTION;
