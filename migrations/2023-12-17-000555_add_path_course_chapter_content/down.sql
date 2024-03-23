-- This file should undo anything in `up.sql`
-- Drop the join table for the many-to-many relationship between path and course
DROP TABLE IF EXISTS paths_courses;

-- Drop the contents table
DROP TABLE IF EXISTS contents;

-- Drop the chapters table
DROP TABLE IF EXISTS chapters;

-- Drop the courses table
DROP TABLE IF EXISTS courses;

-- Drop the paths table
DROP TABLE IF EXISTS paths;
