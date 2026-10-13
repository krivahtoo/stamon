+++
title = "Adding a notification channel"
description = "Send alerts somewhere new."
weight = 4
+++

A channel is a config struct and a send function in `crates/alerts`, plus a
form in the frontend. The server and database need no changes.

{% <devlab.steps> %}
1. **Write the sender.** Add a module in `crates/alerts/src/` with a config
   struct, a `validate()` method, and
   `send(config, &Alert) -> Result<(), String>`. Use the shared `client()`, which
   sets the timeout, and `ensure_success()` to turn a non-2xx response into an
   error. `Alert` provides `title()`, `details()` and `text()` for the message.

2. **Register it** in `crates/alerts/src/lib.rs`: add the module, a variant to
   `ChannelConfig` (its kebab-case name becomes the `type` value), and arms to
   `validate` and `send`.

3. **Add the form.** In
   `frontend/src/routes/(dashboard)/notifications/channel-form.svelte`, add the
   type to `channelTypes`, its fields, its config in `buildConfig()`, and the
   prefill for editing. Add its label to `typeLabels` in
   `notifications/+page.svelte`.

4. **Test and document it.** Add tests in the module, and the channel to
   [Alerts](@/docs/features/alerts.md) and
   [Alert payloads](@/docs/reference/webhook-payload.md).
{% </devlab.steps> %}
