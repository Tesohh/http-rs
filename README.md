# HTTP-rs

CLI for running http requests from .http files, following RFC2161 specs.

## Use case

Writing .sh files for curl requests is painful and repetitive.
Especially if you need to send JSON data.
The best way to do this for me is to use .http files, but the problem is that every editor
integrates them a little differently, or some just don't work like they should.
This CLI aims to be fully compatible with the RFC2161 spec, which means it will be compatible with the
VSCode "rest client" plugin.

## Why rust?

I find rust to be very nice for projects that involve a parser.
