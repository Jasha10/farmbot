# This justfile contains some commands I use in development
install_farmbot:
  cargo install --path .

install_stowsave:
  cargo install --path crates/stowsave

ps:
  just devops/bump
  just devops/push
