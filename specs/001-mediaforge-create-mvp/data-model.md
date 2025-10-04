# data-model.md

## Entities

### User
- id: UUID
- email: string (unique)
- subscription_tier: enum {free, pro}
- daily_quota: integer (derived from tier)
- concurrent_jobs_allowed: integer (derived from tier)
- created_at: timestamp

### MediaAsset
- id: UUID
- user_id: UUID (FK)
- original_filename: string
- format: string (jpg, png, webp, heic, gif, mp4, mov, avi)
- size_bytes: integer
- width: integer (images/videos where available)
- height: integer
- duration_seconds: integer (for video)
- status: enum {uploaded, queued, processing, ready, failed}
- result_location: string (URL to processed asset)
- created_at: timestamp
- expires_at: timestamp (auto 24h after creation or job completion)

### Job
- id: UUID
- media_asset_ids: list<UUID>
- job_type: enum {convert, remove_bg, color_grade, merge, trim}
- parameters: json
- status: enum {pending, running, completed, failed}
- progress_percent: integer
- priority: integer (higher = earlier processing; Pro jobs higher)
- result_location: string
- created_at: timestamp
- completed_at: timestamp

## Validation Rules
- Images: size_bytes <= 5MB
- Videos: duration_seconds <= 30 AND size_bytes <= 50MB
- LUT import: format .cube AND size_bytes <= 1MB
