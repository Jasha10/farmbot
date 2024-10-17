CREATE TABLE managed_computers (name TEXT NOT NULL PRIMARY KEY);

CREATE TABLE IF NOT EXISTS "installed_commands" (
  computer TEXT NOT NULL REFERENCES managed_computers,
  command TEXT NOT NULL,
  installed_how TEXT NOT NULL,
  upgrade_how TEXT NOT NULL,
  current_version TEXT NOT NULL,
  PRIMARY KEY (computer, command)
);

CREATE TABLE local_git_repo_clones (
  computer TEXT NOT NULL REFERENCES managed_computers,
  local_path TEXT NOT NULL,
  remote_url TEXT NOT NULL,
  PRIMARY KEY (computer, local_path)
);

CREATE TABLE tools (name TEXT NOT NULL PRIMARY KEY);

CREATE TABLE tool_websites (
  tool TEXT NOT NULL REFERENCES tools (name),
  url TEXT NOT NULL,
  PRIMARY KEY (tool, url)
);

CREATE TABLE IF NOT EXISTS "tool_versions" (
  tool TEXT NOT NULL REFERENCES tools,
  version TEXT NOT NULL,
  PRIMARY KEY (tool, version)
);

CREATE TABLE package_managers (
  name TEXT NOT NULL PRIMARY KEY REFERENCES tools (name) -- every package manager is a tool
);
