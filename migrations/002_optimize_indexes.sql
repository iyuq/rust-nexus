-- Performance optimization indexes for better QPS
-- This migration adds indexes to improve query performance
-- Note: Using regular CREATE INDEX instead of CONCURRENTLY since migrations run in transactions

-- Add index on name for searches (case-insensitive)
CREATE INDEX IF NOT EXISTS idx_users_name_lower ON users(LOWER(name));

-- Add composite index for pagination queries (created_at + id for stable ordering)
CREATE INDEX IF NOT EXISTS idx_users_created_at_id ON users(created_at DESC, id);

-- Add index on updated_at for filtering recently updated users
CREATE INDEX IF NOT EXISTS idx_users_updated_at ON users(updated_at DESC);

-- Improve the existing email index by making it case-insensitive
DROP INDEX IF EXISTS idx_users_email;
CREATE UNIQUE INDEX idx_users_email_lower ON users(LOWER(email));

-- Add partial index for active users (if you plan to add soft deletes later)
-- CREATE INDEX IF NOT EXISTS idx_users_active ON users(id) WHERE deleted_at IS NULL;

-- Update statistics to help query planner
ANALYZE users;
