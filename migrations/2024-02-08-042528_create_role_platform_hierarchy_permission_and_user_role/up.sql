-- Your SQL goes here
-- Create role_platform_hierarchy table
CREATE TABLE role_platform_hierarchy (
    id SERIAL PRIMARY KEY,
    role_id INT REFERENCES roles(id),
    hierarchy_level INT NOT NULL
);

-- Create role_permission_platform table
CREATE TABLE role_permission_platform (
    id SERIAL PRIMARY KEY,
    role_id INT REFERENCES roles(id),
    permission VARCHAR NOT NULL
);

-- Create user_role_platform table
CREATE TABLE user_role_platform (
    id SERIAL PRIMARY KEY,
    user_id INT REFERENCES users(id),
    role_id INT REFERENCES roles(id)
);
