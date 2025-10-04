# tasks.md — MediaForge MVP (feature)

Feature: MediaForge — MVP
Branch: `001-mediaforge-create-mvp`
Spec: `/Users/ticusb/Projects/mediaForge/specs/001-mediaforge-create-mvp/spec.md`

Overview
- This tasks list implements the Phase 1 design for the MediaForge MVP: upload, conversion, background removal, color grading, job queueing, quotas, and export. Tasks are ordered by dependency. [P] marks tasks that can be executed in parallel by independent agents.

Task numbering conventions: T001, T002, ...

T001 — Setup repository scaffolding and CI (Setup)
- Description: Initialize project folders for backend and frontend, add basic README, CI pipeline stub for lint/test.
- Files/paths: create `backend/`, `frontend/`, `README.md`, `.github/workflows/ci.yml`
- Dependencies: none
- Parallel: no

T002 — Create contracts-based failing tests (TDD) [P]
- Description: For each OpenAPI path in `specs/001-mediaforge-create-mvp/contracts/openapi.yaml`, create a failing contract test that asserts request/response shape.
- Files/paths: `tests/contract/test_upload.py`, `tests/contract/test_convert.py`, `tests/contract/test_status.py`
- Notes: Use simple HTTP client in test harness (Python + pytest example or JS+Jest as preferred by implementer). Tests intentionally fail until server implemented.
- Dependencies: T001
- Parallel: [P]

T003 — Create data models (entities) [P]
- Description: Implement data model definitions and DB schema migrations for `User`, `MediaAsset`, `Job` per `data-model.md`.
- Files/paths: `backend/src/models/user.rs`, `backend/src/models/media_asset.rs`, `backend/src/models/job.rs`, `backend/migrations/*`
- Dependencies: T001
- Parallel: [P]

T004 — Implement storage integration (S3-compatible) (Core)
- Description: Implement server-side storage adapter to upload media to S3-compatible bucket, with ephemeral 24h expiry metadata.
- Files/paths: `backend/src/services/storage.rs`, config in `backend/config/*.toml`
- Dependencies: T003
- Parallel: no

T005 — Implement upload endpoint & multipart handling (Core)
- Description: Implement `POST /api/upload` to accept multipart uploads, validate file type and size (images <=5MB, videos <=50MB && <=30s), persist MediaAsset record with status `uploaded` and store raw file via storage adapter; return jobId or mediaAssetId.
- Files/paths: `backend/src/api/upload.rs`, route registration in `backend/src/main.rs`
- Dependencies: T003, T004, T002
- Parallel: no

T006 — Implement job queue and worker skeleton (Core)
- Description: Add Redis-backed job queue and a basic worker process that can pick jobs, update Job.status and progress_percent.
- Files/paths: `backend/src/services/queue.rs`, `backend/src/worker.rs`
- Dependencies: T003, T004
- Parallel: no

T007 — Implement convert / processing endpoints (Core)
- Description: Implement `POST /api/convert` which enqueues a convert/remove_bg/color_grade job with parameters. Validate LUT imports (.cube <=1MB).
- Files/paths: `backend/src/api/convert.rs`, worker handlers in `backend/src/worker_handlers.rs`
- Dependencies: T005, T006
- Parallel: no

T008 — Implement status endpoint (Core)
- Description: Implement `GET /api/status/{jobId}` to return job status and result location.
- Files/paths: `backend/src/api/status.rs`
- Dependencies: T006, T007
- Parallel: no

T009 — Implement quota and tier enforcement (Core)
- Description: Enforce Free/Pro quotas (Free: 3 jobs/day, 1 concurrent; Pro: unlimited, up to 5 concurrent). Return 402/429 with friendly message or soft paywall prompts when exceeded.
- Files/paths: `backend/src/services/quota.rs`, integrate middleware in `backend/src/api/*`
- Dependencies: T003, T006, T005
- Parallel: no

T010 — Implement background removal processing (Worker) (Core)
- Description: Implement worker pipeline step for `remove_bg` jobs that performs frame-by-frame separation for videos (<=30s, <=50MB) and alpha/matte for images. For MVP, integrate or call native/simple algorithm library; produce result file and update Job.result_location.
- Files/paths: `backend/src/processing/remove_bg.rs`, worker handler updates
- Dependencies: T006, T007, T004
- Parallel: no

T011 — Implement color grading processing and LUT import (Core)
- Description: Implement color grading worker that applies presets and imported `.cube` LUTs (<=1MB) to images/videos.
- Files/paths: `backend/src/processing/color_grade.rs`, `backend/src/services/lut.rs`
- Dependencies: T006, T007
- Parallel: no

T012 — Frontend: Upload UI, drag-and-drop, and preview (Core)
- Description: Implement React components for upload zone, tool selection (Convert/Remove Background/Color Grade), previews (side-by-side), and job list with progress.
- Files/paths: `frontend/src/components/UploadZone.tsx`, `frontend/src/components/Preview.tsx`, `frontend/src/pages/Dashboard.tsx`
- Dependencies: T005, T008
- Parallel: no

T013 — Frontend: Tool controls and export panel (Core)
- Description: Implement controls panel: format dropdown, resolution sliders, LUT import UI, presets grid, download/export button showing file size and format.
- Files/paths: `frontend/src/components/ToolControls.tsx`, `frontend/src/components/ExportPanel.tsx`
- Dependencies: T012
- Parallel: no

T014 — Integration tests for user stories (TDD) [P]
- Description: Implement integration tests (based on `quickstart.md`) covering main user flows: image convert, image bg removal & replace, video bg removal (<=30s), color grade preset application, and export. Tests should assert upload → process → download behavior with contract endpoints.
- Files/paths: `tests/integration/test_convert_flow.py`, `tests/integration/test_remove_bg_flow.py`, `tests/integration/test_color_grade_flow.py`
- Dependencies: T002, T005, T007, T008
- Parallel: [P]

T015 — Logging, metrics, and observability (Integration)
- Description: Add structured logging, job metrics (jobs/sec, avg processing time), and health endpoint to monitor availability. Emit metrics for SLA tracking.
- Files/paths: `backend/src/services/observability.rs`, `backend/src/api/health.rs`
- Dependencies: T006, T007
- Parallel: no

T016 — Security tasks: encryption and data lifecycle (Core)
- Description: Ensure uploads are encrypted in transit (TLS) and at rest (S3 encryption); implement ephemeral deletion job that removes expired MediaAssets after 24 hours.
- Files/paths: `backend/src/services/cleanup.rs`, storage config
- Dependencies: T004, T006
- Parallel: no

T017 — Polish: Unit tests, documentation, and performance tuning [P]
- Description: Add unit tests for core services, performance bench for background removal and conversions, and update README and quickstart examples.
- Files/paths: `backend/tests/*`, `frontend/tests/*`, `README.md`, `docs/quickstart.md`
- Dependencies: All core tasks
- Parallel: [P]

T018 — Release prep and CI integration (Setup)
- Description: Wire up CI to run contract and integration tests and linting; create release checklist and a PR from `001-mediaforge-create-mvp`.
- Files/paths: `.github/workflows/ci.yml`, `RELEASE.md`
- Dependencies: T002, T014, T017
- Parallel: no

Parallel execution examples
- Parallel group A [P]: T002 (contract tests), T003 (data models), T017 (polish unit tests) — independent code areas.
- Parallel group B [P]: T014 (integration tests) can run after contract tests (T002) and core API endpoints exist; run multiple integration tests in parallel where they hit separate test data.

How an LLM agent should run tasks
- Each task contains file paths and a specific goal. For example, to execute T005 the agent should:
  1. Create `backend/src/api/upload.rs` with a `POST /api/upload` handler
  2. Validate multipart body and enforce size/type rules from `data-model.md`
  3. Persist a `MediaAsset` row and upload raw file via storage service
  4. Return 200 with `mediaAssetId` or `jobId`

---

Generated by tools on: 2025-10-04
