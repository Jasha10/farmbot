# Not using sql migrations

I've decided not to use sql migration files for controlling changes to the database schema.
This is because:
- The schema is changing quickly and I can make those changes more quickly via datagrip.
- In the spirit of having this project be reactive to system state rather than attempting to control
it from the outside as nix does, I'd rather save comprehensive post-db-migration schema dumps to the
git repo than have a bunch of migration files that encode the database state in a way that isn't as
easy to understand.
