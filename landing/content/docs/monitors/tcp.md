+++
title = "TCP port"
description = "Check that a TCP port accepts connections."
weight = 3
+++

A TCP monitor is up when Stamon can open a connection to **host** and **port**
(1–65535). Nothing is sent over the connection, so it works for any TCP
service: SSH, SMTP, game servers, or a database you don't want to log into.

To check that a database answers queries, use a [Database](@/docs/monitors/database.md)
monitor instead.
