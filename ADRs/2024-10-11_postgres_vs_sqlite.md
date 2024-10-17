Tradeoffs of using postgresql vs sqlite as a backing data store

Pros of postgresql:
- higher performance
- more features
- better support for if we want to go cloud-based or share a DB between multiple-machines in the future 

Cons of postgresql:
- super heavy weight
- less in-memory support?
- probably more complicated to use & test against

What would be the cost of switching between databases?
Would it be difficult to make the application data-store agnostic? Probably yes -- it would add development cycles, complicate the code, and possibly reduce the leverage we
can get out of `sqlx`.

Flavors of data to be stored in databases:
- use-case specific (only applicable on some machines, e.g. my work config, and possibly contains confidential data that shouldn't be migrated!)
  - would any of this be single-machine specific? Or possibly portable between work computers?
- cross-machine

This opens up questions of syncrhonization of settings:
- my employer probably wouldn't like me using syncthing or my personal dropbox account on the work compy.
  It's easier to ask for forgiveness than for permission, but I'm inclined to design
  from the start for using git as a first-class mechanism for syncing.
- Will the data be so big that we can't dump it all to text files & check into version control?
- Is an sql database even the right choice for a data store? Maybe just .csv or .json files isntead? Or persist as sql files?
  - postgres provides [`pg_dump`](https://www.postgresql.org/docs/current/backup-dump.html) to dump to sql
  - sqlite provides [`.dump`](https://www.sqlite.org/cli.html#dump) to dump to sql
How would we know which parts of the database are OK to check into version control?
Should we maintain two separate databases, one for confidential work stuff and one for my personal stuff
that can be shared via git?
- Merging those two data sources at runtime sounds like a pain.
- Maintaining `no_vcs` or `no_dump` or `confidential` columns in the DB also sounds annoying.

## Revising this on 2024-10-17 after using sqlite a little bit
Cons of sqlite:
- support for e.g. dates is not first class
- you have to opt in to foreign key constraints with a pragma
