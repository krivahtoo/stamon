+++
title = "DNS"
description = "Check that a name resolves to the expected records."
weight = 4
+++

Pick a record type (`A`, `AAAA`, `CNAME`, `MX`, `NS` or `TXT`) and, optionally,
a resolver (`1.1.1.1` or `ip:port`; the system resolver otherwise).

**Expected values** must all be in the answers, compared without case or a
trailing dot, e.g. `93.184.215.14` or `10 mail.example.com`. Without expected
values, the monitor is up whenever the name resolves.

Answers aren't cached between checks.
