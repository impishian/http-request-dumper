# http-request-dumper
A minimal, zero-dependency (except Hyper &amp; Tokio) HTTP “echo” server that listens on :8085 and returns every incoming request as a pretty-printed JSON object.

Useful for debugging web-hooks, reverse proxies, load-balancers, or any client that needs to inspect exactly what it sent over the wire.

## Features

- Shows full URI, HTTP version, method, Host header, remote IP, and all headers
- Captures and re-prints the raw body (up to memory limits) together with its length
- Single-file, async, ~70 LOC, runs on stable Rust – just cargo run
