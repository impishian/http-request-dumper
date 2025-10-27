# http-request-dumper
A minimal, zero-dependency (except Hyper &amp; Tokio) HTTP “echo” server that listens on :8085 and returns every incoming request as a pretty-printed JSON object.

Useful for debugging web-hooks, reverse proxies, load-balancers, or any client that needs to inspect exactly what it sent over the wire.

## Features

- Shows full URI, HTTP version, method, Host header, remote IP, and all headers
- Captures and re-prints the raw body (up to memory limits) together with its length
- Single-file, async, ~70 LOC, runs on stable Rust – just cargo run

In addition to the Rust implementation, Go implementation is also provided.

## Example

```bash
$ curl -s -X POST -d '{"a":1}' -H "Content-Type: application/json" -H "Custome-Header: 1234"  "localhost:8085?b=2&c=3"|jq .
```

Result:

```json
{
  "host": "localhost:8085",
  "remote_addr": "127.0.0.1:52569",
  "url": "/?b=2&c=3",
  "request_uri": "/?b=2&c=3",
  "proto": "HTTP/1.1",
  "method": "POST",
  "headers": {
    "user-agent": "curl/8.7.1",
    "host": "localhost:8085",
    "accept": "*/*",
    "content-length": "7",
    "content-type": "application/json",
    "custome-header": "1234"
  },
  "content_length": 7,
  "body": "{\"a\":1}"
}
```
