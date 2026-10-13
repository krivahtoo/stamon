+++
title = "Adding a check type"
description = "Add a new kind of monitor."
weight = 3
+++

A check type is a config struct and a function in `crates/checks`, plus a form
in the frontend. The server and database need no changes.

{% <devlab.steps> %}
1. **Write the check.** Add a module in `crates/checks/src/` with a config struct
   (`Serialize`, `Deserialize`), a `validate()` method, and an
   `async fn check(...) -> CheckOutcome`. Look at `tcp.rs` for a small example.

2. **Register it** in `crates/checks/src/lib.rs`:
   - add the module and re-export it;
   - add a variant to `CheckConfig`. Its kebab-case name becomes the `type`
     value in JSON;
   - add arms to `validate` and `check`;
   - if the config holds secrets, handle them in `redact` and
     `keep_secrets_from`;
   - if the service reports in rather than being checked, update `is_passive`.

3. **Check the target.** The dashboard shows a service's target from the
   config's `url`, `host` or `container` field. If yours uses another key, add a
   migration that updates the generated `target` column.

4. **Add the form.** In
   `frontend/src/routes/(dashboard)/services/new/+page.svelte`, add the type to
   `serviceTypes`, its fields to the form, and its config to `buildConfig()`.
   Check `services/[id]/+page.svelte` shows it correctly.

5. **Test and document it.** Add unit tests next to the check, and a page in
   `landing/content/docs/monitors/`, linked from the
   [check types table](@/docs/monitors/_index.md#check-types).
{% </devlab.steps> %}
