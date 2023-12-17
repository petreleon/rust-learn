-- Create path table
CREATE TABLE paths (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL
    -- other fields
);

-- Create course table
CREATE TABLE courses (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL
    -- other fields
);

-- Create chapter table
CREATE TABLE chapters (
    id SERIAL PRIMARY KEY,
    course_id INTEGER REFERENCES courses(id),
    title VARCHAR NOT NULL,
    order INTEGER NOT NULL,
    UNIQUE (course_id, order)
    -- other fields
);

-- Create content table with order and unique constraint on chapter_id and order
CREATE TABLE contents (
    id SERIAL PRIMARY KEY,
    chapter_id INTEGER REFERENCES chapters(id),
    order INTEGER NOT NULL,
    content_type VARCHAR NOT NULL,
    data TEXT,
    UNIQUE (chapter_id, order)
    -- other fields
);

-- Create join table for many-to-many relationship between path and course
-- with order and unique constraint on path_id and order
CREATE TABLE paths_courses (
    path_id INTEGER REFERENCES paths(id),
    course_id INTEGER REFERENCES courses(id),
    order INTEGER NOT NULL,
    PRIMARY KEY (path_id, course_id),
    UNIQUE (path_id, order)
);

