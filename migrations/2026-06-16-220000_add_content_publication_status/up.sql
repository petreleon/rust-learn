ALTER TABLE contents
    ADD COLUMN publication_status VARCHAR NOT NULL DEFAULT 'published';

ALTER TABLE contents
    ADD CONSTRAINT contents_publication_status_check
    CHECK (publication_status IN ('published', 'unpublished'));

CREATE INDEX idx_contents_chapter_publication_order
    ON contents (chapter_id, publication_status, "order");
