+++
title = "Users and roles"
description = "What viewers, editors and admins can do, and how to manage users."
weight = 4
+++

## Roles {#roles}

| Role | Can |
|---|---|
| **Viewer** | See services, logs, stats, maintenance, status pages and alert history |
| **Editor** | Also create, edit, pause and delete monitors, and manage maintenance windows and status pages |
| **Admin** | Also manage users and notification channels |

The first account registered is an admin.

## Manage users {#manage-users}

Admins add users through the API for now. Get `$TOKEN` by signing in, as shown
in [First run](@/docs/getting-started/first-run.md#add-users), and replace the
example password with a unique one.

```sh
# Create a user
curl -H "Authorization: Bearer $TOKEN" -H 'Content-Type: application/json' \
  -d '{"username":"alice","password":"REPLACE-WITH-A-UNIQUE-PASSWORD","role":"viewer"}' \
  http://localhost:3000/api/users

# Change a user's role, or deactivate them (user id 2 here)
curl -X PUT -H "Authorization: Bearer $TOKEN" -H 'Content-Type: application/json' \
  -d '{"role":"editor","active":true}' \
  http://localhost:3000/api/users/2
```

Role changes and deactivation apply right away: every request is checked
against the user's current role, so a deactivated user is refused on their next
request, even with an unexpired sign-in. Admins can't change
their own role or status.

## Hidden secrets

Secrets in monitor settings are hidden from the API: database passwords and
HTTP header values show as `********`, and push tokens do too for viewers.
