-- First, create a new table with the desired structure
create table installed_commands_new
(
    computer        TEXT not null
        references managed_computers,
    command         TEXT not null,
    installed_how   TEXT not null,
    upgrade_how     TEXT not null,
    current_version TEXT not null,  -- this column is now not null
    primary key (computer, command)
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
