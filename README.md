# VarLog

VarLog is a simple mock HTTP service written in Rust that logs all incoming HTTP requests.

- 🌐 Accepts all HTTP methods (GET, POST, PUT, DELETE, etc.)
- ✅ Returns HTTP 200 OK for every request
- 📝 Logs request details (method, path, headers, body, timestamp)
- 🖥️ Provides a web UI to view and download request logs
- 🔄 Outputs request information to the console in real-time
- 💾 Stores all requests in a text file for persistence

## Requirements

* [Docker](https://docs.docker.com/engine/install/)
* [Rust](https://www.rust-lang.org/tools/install)
* Optional: bash/zsh alias `task='./task'` with [completion](https://taskfile.build/#completion)

## Usage

Clone the repo.

```bash
git clone git@github.com:Mint-System/VarLog.git
cd VarLog
```

### Run the service

```bash
task run
```

This will start the service at <http://127.0.0.1:8080>.

### Interacting with VarLog

You can send any HTTP request to any path on the server:

```bash
curl http://127.0.0.1:8080/any/path

curl -X POST http://127.0.0.1:8080/api/data -d '{"key": "value"}'

curl -X PUT http://127.0.0.1:8080/resource/123 -d '{"status": "updated"}'

curl -X DELETE http://127.0.0.1:8080/resource/123
```

To view the received requests visit <http://127.0.0.1:8080/ui> in your browser.

## Develop

### Container image

Build the image.

```bash
task build
```

Run the container.

```bash
task start
```

Publish the container.

```bash
task publish
```