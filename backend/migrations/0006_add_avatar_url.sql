-- Migration: Add avatar_url to users table
-- Created: 2026-02-16

ALTER TABLE users ADD COLUMN avatar_url TEXT;