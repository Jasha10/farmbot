CREATE TABLE IF NOT EXISTS "computers" (name TEXT NOT NULL PRIMARY KEY);

CREATE TABLE local_git_repo_clones (
  computer TEXT NOT NULL REFERENCES "computers",
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

CREATE TABLE IF NOT EXISTS "managed_packages" (
  computer TEXT not null references computers,
  tool TEXT not null references tools,
  package_manager TEXT not null references package_managers,
  installed_how TEXT not null,
  upgrade_how TEXT not null,
  version TEXT not null,
  notes TEXT,
  primary key (computer, tool, package_manager),
  foreign key (tool, version) references tool_versions
);

CREATE TABLE manually_installed_tools (
  computer TEXT not null references computers,
  tool TEXT not null references tools,
  version TEXT not null,
  notes TEXT,
  primary key (computer, tool),
  foreign key (tool, version) references tool_versions
);
