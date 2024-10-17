CREATE TABLE managed_computers (name TEXT NOT NULL PRIMARY KEY);

CREATE TABLE IF NOT EXISTS "installed_commands" (
  computer TEXT not null references managed_computers,
  command TEXT not null,
  installed_how TEXT not null,
  upgrade_how TEXT not null,
  current_version TEXT not null, -- this column is now not null
  primary key (computer, command)
);

CREATE TABLE local_git_repo_clones (
  computer TEXT not null references managed_computers,
  local_path TEXT not null,
  remote_url TEXT not null,
  primary key (computer, local_path)
);

CREATE TABLE tools (name TEXT NOT NULL PRIMARY KEY);

CREATE TABLE tool_versions (
  tool TEXT NOT NULL PRIMARY KEY REFERENCES tools (name),
  version TEXT NOT NULL
);
