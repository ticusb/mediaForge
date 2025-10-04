-- SQL migration: create users, media_assets, jobs

CREATE TABLE users (
  id UUID PRIMARY KEY,
  email TEXT UNIQUE NOT NULL,
  subscription_tier TEXT NOT NULL,
  daily_quota INTEGER NOT NULL,
  concurrent_jobs_allowed INTEGER NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT now()
);

CREATE TABLE media_assets (
  id UUID PRIMARY KEY,
  user_id UUID REFERENCES users(id),
  original_filename TEXT,
  format TEXT,
  size_bytes BIGINT,
  width INTEGER,
  height INTEGER,
  duration_seconds INTEGER,
  status TEXT,
  result_location TEXT,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
  expires_at TIMESTAMP WITH TIME ZONE
);

CREATE TABLE jobs (
  id UUID PRIMARY KEY,
  media_asset_ids JSONB,
  job_type TEXT,
  parameters JSONB,
  status TEXT,
  progress_percent INTEGER,
  priority INTEGER,
  result_location TEXT,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
  completed_at TIMESTAMP WITH TIME ZONE
);
