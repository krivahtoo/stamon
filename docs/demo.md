# Public demo

The demo at <https://demo.stamon.org> runs the released Docker image on
Render's free plan with `DEMO_MODE=true`.

## What demo mode does

- Seeds an empty database with sample services, history, a status page
  (`/status/demo`), a maintenance window and a webhook channel.
- Creates a shared **editor** account, `demo` / `demo1234`. The login page
  fills it in. There is no admin, so visitors can't manage accounts.
- Checks only public addresses. Targets on loopback, private, link-local and
  other special ranges are refused, both when saving and when the check runs
  (after resolving, and on HTTP redirects).
- Turns off Docker and database checks, sets a 60s minimum interval, and
  caps the demo at 50 services.
- Records alerts in the delivery history without sending them, and turns off
  test alerts, registration and user changes.
- Makes Settings read-only: the theme still changes, but timezone, password
  and instance settings can't be saved.

## Resets

The free plan has no disk, so each deploy or restart starts from fresh demo
data. The [Reset demo](../.github/workflows/demo.yml) workflow redeploys
every hour, and the Docker workflow redeploys after each release. Both use
the `RENDER_DEPLOY_HOOK` secret and skip when it isn't set.

A free instance also sleeps after about 15 minutes without traffic. The next
visitor waits up to a minute for it to start, and gets fresh data.

## Setup

1. In Render, choose **New → Blueprint** and pick this repository. It reads
   [`render.yaml`](../render.yaml), which creates the `stamon-demo` web
   service and generates its `JWT_SECRET`.
2. In the service's **Settings**, copy the **Deploy Hook** URL and save it
   as the `RENDER_DEPLOY_HOOK` secret of the GitHub repository.
3. Add a DNS record `demo CNAME stamon-demo.onrender.com` (use the service's
   own `.onrender.com` host). Render then verifies `demo.stamon.org` and
   issues its certificate.
4. Run the **Reset demo** workflow once, and check that
   `https://demo.stamon.org/api/check` answers with `"demo": true`.

The image must be a release with demo mode. Until then, point the service
at the `edge` tag.

## Trying it locally

```sh
docker run --rm -e DEMO_MODE=true -e JWT_SECRET=dev -p 3000:3000 k4htoo/stamon
```

Without a volume, every run starts from fresh demo data.
