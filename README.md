# Agentic Runtime

An async task runtime in Rust. You submit tasks over HTTP, a bounded worker pool runs them through a pluggable `Executor`, and every state change is published on an event bus.

Built with Tokio and Axum. Phase 1 (in-memory runtime) is complete; Phase 2 (durable Postgres backend) is in progress.

<!-- Add a 10-second GIF here: start the server, POST a task, watch it move Queued → Running → Completed in the logs. -->

## Quick start

```bash
git clone https://github.com/Vivek23456/agentic-runtime
cd agentic-runtime
cargo run -p api
```

```bash
# submit a task
curl -X POST localhost:3000/tasks \
  -H 'content-type: application/json' \
  -d '{"name": "summarize", "payload": {"text": "hello"}}'

# check it
curl localhost:3000/tasks/<id>
```

<!-- Confirm the port and the exact request body your handler accepts, and paste a real response below. -->

```json
{ "id": "…", "state": "Completed", "created_at": "…" }
```

## What works today

- **Task queue + worker pool:** tasks go onto a Tokio MPSC queue and are consumed by a fixed number of workers.
- **Pluggable execution:** workers call an `Executor` trait, so new task types don't touch the runtime core.
- **Task lifecycle:** `Created → Queued → Running → Completed | Failed | Cancelled`, tracked in a shared registry.
- **Event bus:** state changes are published on a `tokio::sync::broadcast` channel (`TaskRunning`, `TaskCompleted`, `TaskFailed`).
- **LLM provider:** OpenAI provider for model calls. <!-- Confirm what it supports (streaming? tool calls?) and which crate it lives in. -->
- **HTTP API (Axum):** `GET /health`, `POST /tasks`, `GET /tasks`, `GET /tasks/{id}`.
- **Observability:** structured JSON logs via `tracing`. CI on GitHub Actions.

## Architecture

```
POST /tasks ──► Runtime Manager ──► Task Registry (state)
                     │
                     ▼
                Task Queue (mpsc)
                     │
                     ▼
              Worker Pool (N workers) ──► Executor trait
                     │
                     ▼
              Event Bus (broadcast) ──► logs / future dashboard
```

```
crates/
├── runtime/   # manager, queue, worker pool, registry, executor, events
└── api/       # Axum HTTP server
```

## Design decisions

<!-- This section is what senior reviewers read. Answer each honestly from your code; delete any you can't answer yet. -->

- **Why a bounded queue?** What happens when submissions outpace workers: does `POST /tasks` wait, reject, or grow without limit?
- **Why broadcast for events?** Slow subscribers lag and drop messages (`RecvError::Lagged`). How the runtime handles that, and why that trade-off is acceptable for status events.
- **What if an executor panics?** Whether the worker survives, and whether the task is marked `Failed`.
- **Registry locking:** which lock you use (`Mutex` / `RwLock` / `DashMap`) and why.
- **Shutdown:** how in-flight tasks are handled on Ctrl-C.

## Testing

```bash
cargo test --workspace
```

<!-- List what the tests cover: lifecycle transitions, concurrent submissions, executor failure. -->

## Roadmap

**Phase 2: durability (in progress)**
- Storage trait with Postgres (sqlx) and SQLite backends
- Task claiming with `SELECT … FOR UPDATE SKIP LOCKED`
- Retries with exponential backoff, per-task timeouts
- Lease-based recovery of tasks orphaned by a crash
- Load-test results (tasks/sec, p99 latency)

**Later**
- Distributed workers with heartbeats
- HTTP / shell / LLM executors
- Live dashboard over WebSockets, Prometheus metrics

## License

MIT
