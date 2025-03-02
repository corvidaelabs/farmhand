-- Remove the new columns
ALTER TABLE streams
DROP COLUMN video_id,
DROP COLUMN games;

-- Add back the old columns
ALTER TABLE streams
ADD COLUMN event_log TEXT,
ADD COLUMN video_url TEXT;