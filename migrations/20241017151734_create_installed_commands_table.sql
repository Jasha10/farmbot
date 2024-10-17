CREATE TABLE installed_commands (
  command TEXT NOT NULL PRIMARY KEY,
  installed_how TEXT NOT NULL,
  upgrade_how TEXT NOT NULL
);
