DROP INDEX IF EXISTS idx_contents_chapter_publication_order;

ALTER TABLE contents
    DROP CONSTRAINT IF EXISTS contents_publication_status_check;

ALTER TABLE contents
    DROP COLUMN IF EXISTS publication_status;
