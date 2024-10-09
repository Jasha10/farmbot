# farmbot
If GNU stow is a "symlink farm manager," this tool will be a multi-purpose farming robot.

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

### Managing software versions
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

### Inspiration
- GNU Stow
- RaphGL/tuckr
- yadam
- knowledge graphs
