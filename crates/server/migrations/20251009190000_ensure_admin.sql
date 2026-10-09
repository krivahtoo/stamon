-- Before roles were enforced, the first account was saved as a viewer but
-- signed in as an admin. Promote the oldest active account on installs
-- left without an active admin, so someone can manage users again.
UPDATE Users
SET role = 'admin'
WHERE id = (SELECT MIN(id) FROM Users WHERE active = 1)
  AND NOT EXISTS (SELECT 1 FROM Users WHERE role = 'admin' AND active = 1);
