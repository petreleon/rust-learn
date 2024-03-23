-- Your SQL goes here
-- Up Migration

CREATE TABLE role_permission_course (
    id SERIAL PRIMARY KEY,
    course_id INT NOT NULL,
    role_id INT NOT NULL,
    permission VARCHAR NOT NULL,
    FOREIGN KEY (course_id) REFERENCES courses(id),
    FOREIGN KEY (role_id) REFERENCES roles(id)
);

CREATE TABLE user_role_course (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    role_id INT NOT NULL,
    course_id INT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (role_id) REFERENCES roles(id),
    FOREIGN KEY (course_id) REFERENCES courses(id)
);

CREATE TABLE role_course_hierarchy (
    id SERIAL PRIMARY KEY,
    role_id INT NOT NULL,
    course_id INT NOT NULL,
    hierarchy_level INT NOT NULL,
    FOREIGN KEY (role_id) REFERENCES roles(id),
    FOREIGN KEY (course_id) REFERENCES courses(id)
);
