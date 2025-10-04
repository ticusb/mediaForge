<!--
Sync Impact Report

Version change: template -> 1.0.0

Modified principles:
- [PRINCIPLE_1_NAME] (added) -> I. Test-First (TDD Mandatory)
- [PRINCIPLE_2_NAME] (added) -> II. User Value & Simplicity
- [PRINCIPLE_3_NAME] (added) -> III. Observability & Metrics
- [PRINCIPLE_4_NAME] (added) -> IV. Data Privacy & Ephemeral Storage
- [PRINCIPLE_5_NAME] (added) -> V. Versioning & Change Management

Added sections:
- Security & Compliance Requirements
- Development Workflow & Quality Gates

Removed sections:
- None

Templates requiring updates:
- .specify/templates/plan-template.md ⚠ pending — ensure "Constitution Check" references new privacy and observability principles
- .specify/templates/spec-template.md ✅ aligned — confirms NFR and ambiguity checks; no changes required
- .specify/templates/tasks-template.md ⚠ pending — recommend adding explicit tasks for retention/cleanup and observability by default

Follow-up TODOs:
- Update plan-template.md Constitution Check to explicitly validate retention and encryption rules.
- Add a short example in tasks-template.md for ephemeral cleanup and SLA metrics tasks.

--> 

# MediaForge Constitution

## Core Principles

### I. Test-First (TDD Mandatory)
Every change to the codebase MUST be accompanied by tests that capture the intended behavior before implementation. This includes:
- Contract tests for public APIs (OpenAPI or equivalent) that are written and failing prior to implementation.
- Integration tests for user stories derived from `quickstart.md` or the feature spec.
- Unit tests for business logic and validation rules.

Rationale: Tests prevent regressions, make acceptance criteria explicit, and enable safe refactors.

### II. User Value & Simplicity
All design and implementation decisions MUST prioritize measurable user value and simplicity. Features MUST be minimally viable to satisfy primary user stories; avoid premature optimization and unnecessary surface area (YAGNI).

Rules:
- Prefer simple, well-documented behaviors with clear success criteria.
- If a tradeoff increases complexity, document rationale and alternatives in the feature spec or research.md.

Rationale: Shipping focused value reduces development cost and speeds feedback loops.

### III. Observability & Metrics
Systems MUST emit structured logs, request-level tracing (where feasible), and metrics required to measure the defined SLAs (e.g., job processing time, error rates, queue depth).

Rules:
- Provide a health endpoint and basic metrics (jobs/sec, avg processing latency, error counts).
- Instrument priority and tiering behavior so SLA impact is measurable.

Rationale: Observability enables operational reliability and data-driven improvements.

### IV. Data Privacy & Ephemeral Storage
Media and user data used for processing are privacy-sensitive. The system MUST:
- Encrypt uploads in transit (TLS) and at rest.
- Retain ephemeral processing artifacts for no longer than the configured retention period (default: 24 hours) unless user explicitly saves an asset.
- Provide clear error messages and UI guidance when retention or size constraints prevent processing.

Rationale: Minimizing data residency reduces regulatory and cost exposure while aligning with user privacy expectations.

### V. Versioning & Change Management
APIs, contracts, and the constitution itself MUST follow semantic versioning and a documented change process.

Rules:
- API contract changes that break consumers require a MAJOR version bump and a migration plan.
- Minor or patch changes that are backward compatible use MINOR/PATCH increments.
- Constitution amendments must be recorded with a version bump and documented rationale.

Rationale: Predictable versioning reduces downstream integration risk and clarifies expectations for consumers.

## Security & Compliance Requirements
The project MUST follow baseline security practices:
- Use TLS for all external endpoints.
- Encrypt objects at rest in storage backends.
- Apply least-privilege for service credentials and rotate secrets regularly.
- Adhere to GDPR/CCPA principles for user data; ephemeral processing artifacts default to 24-hour retention.

## Development Workflow & Quality Gates
The project follows a pull-request centric workflow with automated gates:
- All PRs MUST include tests for the changes and link to the relevant spec or quickstart scenario.
- CI pipelines MUST run contract tests, integration tests (selected smoke tests), and linting before merges to protected branches.
- A successful post-design constitution check (automated or manual) is REQUIRED before Phase 0 completion in `/plan`.

## Governance
Amendments to this constitution require:
1. A documented proposal committed to the repo (PR) describing the change, rationale, and migration plan.
2. Approval from at least two project maintainers or owners.
3. A version bump following semantic versioning: MAJOR for incompatible governance changes, MINOR for new principles or sections, PATCH for clarifications/typo fixes.

All PRs touching core behavior or contracts MUST reference the constitution where relevant and demonstrate compliance with applicable principles.

**Version**: 1.0.0 | **Ratified**: 2025-10-04 | **Last Amended**: 2025-10-04