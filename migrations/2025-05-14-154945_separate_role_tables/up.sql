-- Your SQL goes here
-- Step 1: Create new role tables
CREATE TABLE platform_roles (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description TEXT
);

CREATE TABLE organization_roles (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description TEXT,
    created_for INT4 REFERENCES organizations(id)
);

CREATE TABLE course_roles (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description TEXT,
    created_for INT4 REFERENCES courses(id)
);

-- Step 2: Recreate all dependent tables to use new role tables

DROP TABLE IF EXISTS user_role_platform;
CREATE TABLE user_role_platform (
    id SERIAL PRIMARY KEY,
    user_id INT4 REFERENCES users(id),
    role_id INT4 REFERENCES platform_roles(id)
);

DROP TABLE IF EXISTS user_role_organization;
CREATE TABLE user_role_organization (
    id SERIAL PRIMARY KEY,
    user_id INT4 REFERENCES users(id),
    role_id INT4 REFERENCES organization_roles(id),
    organization_id INT4 REFERENCES organizations(id)
);

DROP TABLE IF EXISTS user_role_course;
CREATE TABLE user_role_course (
    id SERIAL PRIMARY KEY,
    user_id INT4 REFERENCES users(id),
    role_id INT4 REFERENCES course_roles(id),
    course_id INT4 REFERENCES courses(id)
);

DROP TABLE IF EXISTS role_permission_platform;
CREATE TABLE role_permission_platform (
    id SERIAL PRIMARY KEY,
    role_id INT4 REFERENCES platform_roles(id),
    permission VARCHAR NOT NULL
);

DROP TABLE IF EXISTS role_permission_organization;
CREATE TABLE role_permission_organization (
    id SERIAL PRIMARY KEY,
    organization_id INT4 REFERENCES organizations(id),
    role_id INT4 REFERENCES organization_roles(id),
    permission VARCHAR NOT NULL
);

DROP TABLE IF EXISTS role_permission_course;
CREATE TABLE role_permission_course (
    id SERIAL PRIMARY KEY,
    course_id INT4 REFERENCES courses(id),
    role_id INT4 REFERENCES course_roles(id),
    permission VARCHAR NOT NULL
);

DROP TABLE IF EXISTS role_platform_hierarchy;
CREATE TABLE role_platform_hierarchy (
    id SERIAL PRIMARY KEY,
    role_id INT4 REFERENCES platform_roles(id),
    hierarchy_level INT4 NOT NULL
);

DROP TABLE IF EXISTS role_organization_hierarchy;
CREATE TABLE role_organization_hierarchy (
    id SERIAL PRIMARY KEY,
    role_id INT4 REFERENCES organization_roles(id),
    organization_id INT4 REFERENCES organizations(id),
    hierarchy_level INT4 NOT NULL
);

DROP TABLE IF EXISTS role_course_hierarchy;
CREATE TABLE role_course_hierarchy (
    id SERIAL PRIMARY KEY,
    role_id INT4 REFERENCES course_roles(id),
    course_id INT4 REFERENCES courses(id),
    hierarchy_level INT4 NOT NULL
);

-- Step 3: Remove original roles table
DROP TABLE IF EXISTS roles;
