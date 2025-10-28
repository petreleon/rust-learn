-- Re-add organization_id column to role_organization_hierarchy
-- Recreates the nullable organization_id column and its foreign key to organizations(id)

ALTER TABLE role_organization_hierarchy
    ADD COLUMN IF NOT EXISTS organization_id INT;

-- Add foreign key constraint (name chosen to be explicit)
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'role_org_hierarchy_organization_id_fkey'
    ) THEN
        ALTER TABLE role_organization_hierarchy
            ADD CONSTRAINT role_org_hierarchy_organization_id_fkey
            FOREIGN KEY (organization_id) REFERENCES organizations(id);
    END IF;
END$$;
