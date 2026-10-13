+++
title = "HTTP(S)"
description = "Check a URL's status code and response body."
weight = 1
+++

An HTTP(S) monitor requests a URL and checks the response.

- Choose the **method**, and optionally **headers** and a **body**. A JSON body
  is sent as `application/json` unless you set `Content-Type` yourself.
- **Expected code**: an exact status like `204`, or a single digit for a whole
  class (`2` accepts any 2xx). Without one, any 2xx is up.
- **Expected payload**: JSON the whole response must equal.
- **Keyword**: text the response must contain.
- **JSON value**: a [JSON pointer](https://datatracker.ietf.org/doc/html/rfc6901)
  such as `/status` or `/checks/0/ok`, and optionally the value it must have. On
  its own, the value only has to exist.

Every assertion you set must pass for the service to be up.

Header values are hidden from the API once saved; they show as `********`.
