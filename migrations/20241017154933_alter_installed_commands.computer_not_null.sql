-- First, create a new table with the desired structure
CREATE TABLE installed_commands_new (
  command TEXT NOT NULL PRIMARY KEY,
  installed_how TEXT NOT NULL,
  upgrade_how TEXT NOT NULL,
  computer TEXT NOT NULL REFERENCES managed_computers
);

-- Copy data from the old table to the new table
INSERT INTO
  installed_commands_new
SELECT
  *
FROM
  installed_commands;

-- Drop the old table
DROP TABLE installed_commands;

-- Rename the new table to the original name
ALTER TABLE installed_commands_new
RENAME TO installed_commands;
