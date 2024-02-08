
-- Create roles table
CREATE TABLE roles (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description TEXT
);

-- Create role_permission_organization with foreign key to roles
CREATE TABLE role_permission_organization (
    id SERIAL PRIMARY KEY,
    organization_id INT4 REFERENCES organizations(id),
    role_id INT4 REFERENCES roles(id),
    permission VARCHAR NOT NULL
);

-- Create user_role_organization with foreign key to roles
CREATE TABLE user_role_organization (
    id SERIAL PRIMARY KEY,
    user_id INT4 REFERENCES users(id),
    role_id INT4 REFERENCES roles(id),
    organization_id INT4 REFERENCES organizations(id)
);

-- Create role_organization_hierarchy with hierarchy level
CREATE TABLE role_organization_hierarchy (
    id SERIAL PRIMARY KEY,
    role_id INT4 REFERENCES roles(id),
    organization_id INT4 REFERENCES organizations(id),
    hierarchy_level INT4 NOT NULL
);
