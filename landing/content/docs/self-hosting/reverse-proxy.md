+++
title = "Reverse proxy"
description = "Serve Stamon over HTTPS behind Caddy or nginx."
weight = 2
+++

Put Stamon behind a reverse proxy to serve it over HTTPS on your own domain.

{% <devlab.callout type="warning" title="HTTPS is required to sign in"> %}
Stamon's sign-in cookie is marked `Secure`, so browsers only keep it over HTTPS,
or on `localhost`. Opening Stamon at `http://<server-ip>:3000` from another
machine won't keep you signed in.
{% </devlab.callout> %}

## What the proxy must do

- Proxy everything on the domain to `http://<stamon-host>:3000`. Stamon must be
  at the root of the domain; sub-paths aren't supported.
- Forward WebSocket upgrades on `/ws`, which carries the dashboard's live
  updates.
- Pass the original `Host` header, or set `X-Forwarded-Host`. Stamon only
  accepts live updates connections from a page on the same host.

## Examples

{% <devlab.tabs sync="reverse-proxy" label="Reverse proxy"> %}
{% <devlab.tab name="Caddy" value="caddy" selected={true}> %}
Caddy gets a certificate and proxies WebSockets automatically:

```
status.example.com {
    reverse_proxy localhost:3000
}
```
{% </devlab.tab> %}
{% <devlab.tab name="nginx" value="nginx"> %}
```nginx
server {
    listen 443 ssl;
    server_name status.example.com;
    # ssl_certificate and ssl_certificate_key here

    location / {
        proxy_pass http://127.0.0.1:3000;
        proxy_set_header Host $host;
    }

    location /ws {
        proxy_pass http://127.0.0.1:3000;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_read_timeout 1h;
    }
}
```
{% </devlab.tab> %}
{% </devlab.tabs> %}

{% <devlab.callout type="warning" title="Live updates stop after upgrading"> %}
If the dashboard keeps showing "Websocket disconnected", your proxy is
rewriting `Host` on `/ws`. With nginx, add `proxy_set_header Host $host;` to the
`location /ws` block, as in the example above.
{% </devlab.callout> %}

## Known limitations

- **Client addresses:** Stamon doesn't read `X-Forwarded-For`, so its logs show
  the proxy's address.
