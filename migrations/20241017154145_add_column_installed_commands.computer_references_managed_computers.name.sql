ALTER TABLE installed_commands
ADD COLUMN computer TEXT REFERENCES managed_computers (name);
