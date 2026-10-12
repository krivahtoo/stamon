+++
title = "First run"
description = "Create the admin account and add more users."
weight = 2
+++

## Create the admin account {#first-run}

Open Stamon and register. The first account becomes the **admin**; after that,
registration is closed.

## Add users

Admins add more users through the API for now (a dashboard page is planned).
Sign in to get a token, then create the user with a role of `viewer`, `editor`
or `admin`:

```sh
TOKEN=$(curl -s -H 'Content-Type: application/json' \
  -d '{"username":"admin","password":"your-password"}' \
  http://localhost:3000/api/login | jq -r .token)

curl -H "Authorization: Bearer $TOKEN" -H 'Content-Type: application/json' \
  -d '{"username":"alice","password":"a-strong-password","role":"editor"}' \
  http://localhost:3000/api/users
```

See [Users and roles](@/docs/features/users-and-roles.md) for what each role can
do, and how to change or deactivate a user.
