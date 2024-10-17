# This justfile contains some commands I use in development
install_farmbot:
  cargo install --path .

install_stowsave:
  cargo install --path crates/stowsave

ps:
  just devops/push

bp:
  just devops/bump  # bump version, which will trigger gh workflow to release to crates.io
  just devops/push  # push to github

migrate_add *description:
  sqlx migrate add '{{ description }}'

migrate_run:
  sqlx migrate run

migrate_info:
  sqlx migrate info

dump_schema:
  sqlite3 ./farmbot.db .schema > schema.sql
  sql-formatter --fix ./schema.sql

dump_db:
  sqlite3 ./farmbot.db .dump > dump.sql
  sql-formatter --fix ./dump.sql

watch_dump_schema:
  watchexec --on-busy-update queue --debounce 1s -e db --filter ./farmbot.db -- just dump_schema
