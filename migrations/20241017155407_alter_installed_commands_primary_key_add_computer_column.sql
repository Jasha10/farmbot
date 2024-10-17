-- Step 1: Create a new table with the desired structure
CREATE TABLE new_installed_commands (
  computer TEXT not null,
  command TEXT not null,
  installed_how TEXT not null,
  upgrade_how TEXT not null,
  PRIMARY KEY (computer, command),
  FOREIGN KEY (computer) REFERENCES managed_computers (name)
);

-- Step 2: Copy data from the old table to the new table
INSERT INTO
  new_installed_commands (computer, command, installed_how, upgrade_how)
SELECT
  computer,
  command,
  installed_how,
  upgrade_how
FROM
  installed_commands;

-- Step 3: Drop the old table
DROP TABLE installed_commands;

-- Step 4: Rename the new table to the original table name
ALTER TABLE new_installed_commands
RENAME TO installed_commands;

-- The table has now been restructured with:
-- - The computer column first
-- - A composite primary key of (computer, command)
-- - The same data as before
