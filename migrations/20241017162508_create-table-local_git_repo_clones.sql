create table local_git_repo_clones (
  computer TEXT not null references managed_computers,
  local_path TEXT not null,
  remote_url TEXT not null,
  primary key (computer, local_path)
);
