# RORA - Notes Demo Cloudflare Worker

This is a Rust server app written with ([tide](https://github.com/http-rs/tide)) _running_ inside a Cloudflare
worker. The core app code lives in [notes-demo](https://github.com/rora-rs/notes-demo) and this repository contains the
glue code needed to _run_ the server.

Try the [demo](https://notes-demo-cf-worker.logankeenan.workers.dev/).

## Wrangler

```bash
# compiles your project to WebAssembly and will warn of any issues
wrangler build 

# run your Worker in an ideal development workflow (with a local server, file watcher & more)
wrangler dev

# deploy your Worker globally to the Cloudflare network (update your wrangler.toml file for configuration)
wrangler publish
```
