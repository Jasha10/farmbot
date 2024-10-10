# farmbot
If GNU stow is a "symlink farm manager," this tool will be a multi-purpose farming robot. A symlink farmer bot and as well as a package manager manager bot (a robot that manages package managers).

See [INSTALL.md](INSTALL.md) and [USAGE.md](USAGE.md) to get started.

## Vision & Goals:
I spend lots of time updating dependencies -- both locally installed tools and dependency pins on software projects.
Keeping these up to date improves developer quality of life.
I also spend time fiddling with symlinks a la GNU stow / tuckr / yadam -- moving files into my local "dotfiles" repository, creating symlinks to those dotfiles on the local machine, and making sure those symlinks stay up to date.

## Goals:
I want to create a monolithic automation framework that will run on my local machine. It will be data driven (probably backed by postgres or sqlite), written primarily in rust, and will run in the background to detect when actions should be taken. I hope to create command line tools and a GUI for viewing and modifiying the bot's behavior.

Below are some use-cases, though I hope to make the framework extensible enough to handle other use cases too:

### Managing symlink farms
- Keep track of where my local dotfiles have been stow'd. Detect when any of my local symlink farms become "out of sync." Offer to take action when:
  - a file in any of my stow packages is moved or deleted
  - a symlink in the target directory has been moved or deleted
Create a framework for taking follow-up actions after stowing a package, e.g. `stow my-package` followed by `cd target-dir && stow -t .git stow-t.git` to stow a `.git/info/exclude` file.
Do this well by manipulating in-memory tree structures representing files, simulating what the tree "should look like" (to detect if the farm is out of sync), and simulating what the tree will look like after an action such as `stow` is taken.

### Managing Package Managers
- keep track of software installed on the system via various package managers
- possibly change the state of the system to match a specification

Open question: Is this vision trying to solve some of the same things as those infra-as-code / config management / automation projects like OpenTofu / Ansible / Puppet / Saltstack / ...? I've never used them.

#### Managing software versions
- Automate bumping versions installed software
  - use first-class tools like `cargo`, `brew`, `apt`, `pipx`, etc. to install and manage software locally
  - Maintain a knowledge graph:
    - What software versions are currently installed, which versions are available, and which versions
  - Log all actions taken in the database.
- Periodically query github, pypi, crates.io, homebrew website, etc to determine if new versions are available.
- Use a database to create a knowledge graph regarding:
  - available software versions
  - currently installed software versions
  - pinned versions in tracked software projects
  - tools / scripts / commands used for updating software
- For bumping version pins in software projects, be able to create a new local branch, bump the version pin, and create a github draft PR from the new branch
- Handle multi-step processes gracefully, e.g. "upgrade the software version and then restart the associated service."
- Keep track of dependencies not handled by package managers, e.g. "Installing this rust project with cargo requires cmake".
  - possibly install cmake via another package manager as a prerequisite
  - keep track of where the sentence "Installing this rust project with cargo requires cmake" appears in the rust project's README. Periodically query that README.md to see if the sentence (and its surrounding context) still appear in the README.

##### Tracking Installed Software
- Query package managers (`cargo`, `apt`, etc) to see what software is currently installed, the version and source, etc.
- Be able to distinguish software installed from a local source vs from an external repo (e.g. `cargo install --path . vs `cargo install <hosted-package>`)

##### Managing Installed Software
- Take action (with possible user confirmation) to install, upgrade, or uninstall software.

### Inspiration
- GNU Stow
- RaphGL/tuckr
- yadam
- knowledge graphs

## ROADMAP:
- [ ] weigh tradeoffs postgres vs sqlite
- [ ] first useful behavior: create PR branch / worktree for a given purpose in a local worktree
  - [ ] create tables:
    - [ ] in the "git" namespace/schema: "local_repos", "local_branches", "local_worktrees". Populate with some local data.
    - [ ] create tables "deps" and "dep_versions". Populate with data on some deps / dep versions. 
    - [ ] create a table "upgrade_deps" in the "issues" namespace/schema. Populate with data on a dep I'd like to upgrade.
  - [ ] propose a worktree name based on the name of the local_repo and the dep to be upgraded
  - [ ] execute the git command to create the worktree
- [ ] Integration with github
  - [ ] Set up github REST api polling. We can't use webhooks for orgs/repos we don't own.
  - [ ] Alternative to polling individual repos, we can poll the authorized user's notifications?
