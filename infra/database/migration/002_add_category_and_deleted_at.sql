-- Add category and deleted_at columns to streams table
ALTER TABLE streams ADD COLUMN category TEXT NOT NULL DEFAULT '';
ALTER TABLE streams ADD COLUMN deleted_at TIMESTAMPTZ;
