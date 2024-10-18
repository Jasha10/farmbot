CREATE TABLE managed_computers (name TEXT NOT NULL PRIMARY KEY);

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

CREATE VIEW aug_package_managers AS
SELECT
  pm.name
FROM
  package_managers pm
  JOIN tools t ON pm.name = t.name
  /* aug_package_managers(name) */;

CREATE VIEW aug_local_git_repo_clones AS
SELECT
  lgrc.*
FROM
  local_git_repo_clones lgrc
  JOIN managed_computers mc ON lgrc.computer = mc.name
  /* aug_local_git_repo_clones(computer,local_path,remote_url) */;

CREATE VIEW aug_tool_websites AS
SELECT
  tw.*
FROM
  tool_websites tw
  JOIN tools t ON t.name = tw.tool
  /* aug_tool_websites(tool,url) */;

CREATE VIEW aug_tool_versions AS
SELECT
  tv.*
FROM
  tool_versions tv
  JOIN tools t ON t.name = tv.tool
  /* aug_tool_versions(tool,version) */;

CREATE TABLE IF NOT EXISTS "installed_commands" (
  computer TEXT not null REFERENCES managed_computers,
  command TEXT not null REFERENCES tools (name),
  package_manager TEXT not null REFERENCES tools (name),
  installed_how TEXT not null,
  upgrade_how TEXT not null,
  current_version TEXT not null,
  primary key (computer, command, package_manager)
);

CREATE VIEW aug_installed_commands AS
SELECT
  ic.*
FROM
  installed_commands ic
  JOIN main.managed_computers mc on ic.computer = mc.name
  /* aug_installed_commands(computer,command,package_manager,installed_how,upgrade_how,current_version) */;
