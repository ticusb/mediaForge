# research.md

## Purpose
Collect and resolve all NEEDS CLARIFICATION items from the MediaForge MVP spec and record decisions with rationale and alternatives.

## Decisions

- Decision: Maximum video length for background removal set to 30 seconds.
  - Rationale: PRD suggested 30s; keeps processing feasible for MVP and reduces server cost.
  - Alternatives considered: 10s (too restrictive), 60s (higher cost/complexity).

- Decision: Free/Pro quotas — Free: 3 jobs/day, 1 concurrent job; Pro: unlimited jobs, up to 5 concurrent jobs, no watermark, priority queueing.
  - Rationale: Encourages upgrade while allowing casual use.

- Decision: Upload limits — Images: 5MB; Videos: max(50MB, 30s whichever smaller).
  - Rationale: Matches common mobile uploads and keeps processing times predictable.

- Decision: LUT support — .cube only, max 1MB.
  - Rationale: Widely-used format; small size keeps client/server footprint low.

- Decision: Retention timeframe — Ephemeral files auto-deleted after 24 hours post-upload or job completion.
  - Rationale: Privacy-first, cost containment.

- Decision: Performance targets — Images <10s for <5MB; Videos <60s for <=30s clips; Availability 99% monthly.
  - Rationale: Aligns with user expectations for "quick" edits in MVP.

## Research tasks (next steps)
- Validate typical mobile image sizes distribution to ensure 5MB limit sensible.
- Estimate per-minute processing cost for background removal on 30s clips to size infra.
- Confirm priority queueing mechanism design (simple priority flag + more workers for Pro jobs).
