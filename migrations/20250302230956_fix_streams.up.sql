-- Drop the event_log and video_url columns
ALTER TABLE streams
DROP COLUMN IF EXISTS event_log_url,
DROP COLUMN IF EXISTS video_url;

-- Add the new columns
ALTER TABLE streams
ADD COLUMN games TEXT[] NOT NULL DEFAULT '{}',
ADD COLUMN video_id TEXT REFERENCES videos(id) ON DELETE SET NULL;