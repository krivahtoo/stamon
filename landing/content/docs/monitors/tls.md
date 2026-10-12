+++
title = "TLS certificate"
description = "Get warned before a certificate expires."
weight = 5
+++

A TLS monitor reads a server's certificate and reports down when it expires
within **warn days** (default 14) or has already expired.

Use it alongside an HTTP(S) monitor: the HTTP check tells you the site is
answering, and the TLS check gives you time to renew before visitors see
certificate errors.
