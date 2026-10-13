+++
title = "Ping"
description = "Check that a host answers ICMP echo requests."
weight = 2
+++

A ping monitor sends an ICMP echo request to a **host** and is up when it gets
a reply within the timeout. The response time is recorded.

The host can be a name (`example.com`), an IP address, or a URL; for a URL or a
`host:port` value, only the host part is used.
