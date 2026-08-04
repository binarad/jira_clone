-- 1. Create independent base tables
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    role VARCHAR(50) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL
);

CREATE TABLE IF NOT EXISTS projects (
    id SERIAL PRIMARY KEY,
    key VARCHAR(50) UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL,
    owner_id INTEGER NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,

    FOREIGN KEY (owner_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Create the sequence tracker
CREATE TABLE project_sequences (
    project_id INTEGER PRIMARY KEY REFERENCES projects(id) ON DELETE CASCADE,
    last_value INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS issues (
    id SERIAL PRIMARY KEY,
    project_id INTEGER NOT NULL,

    issue_number INTEGER NOT NULL,

    -- Issue Metadata
    issue_type VARCHAR(50) NOT NULL, -- e.g Bug, Task, etc.
    summary VARCHAR(255) NOT NULL,
    description TEXT,
    status VARCHAR(50) NOT NULL,
    priority VARCHAR(50) NOT NULL,

    -- User involved
    assignee_id INTEGER, -- who is fixing the issue (can be null)
    reporter_id INTEGER NOT NULL, -- who created issue,

    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,

    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    FOREIGN KEY (assignee_id) REFERENCES users(id) ON DELETE SET NULL,
    FOREIGN KEY (reporter_id) REFERENCES users(id) ON DELETE CASCADE,

    UNIQUE(project_id, issue_number)
);

-- 4. Create the function
CREATE OR REPLACE FUNCTION assign_issue_number()
RETURNS TRIGGER AS $$
DECLARE
    next_num INTEGER;
BEGIN
    -- Upsert and lock the sequence row for this specific project
    INSERT INTO project_sequences (project_id, last_value)
    VALUES (NEW.project_id, 1)
    ON CONFLICT (project_id) DO UPDATE
    SET last_value = project_sequences.last_value + 1
    RETURNING last_value INTO next_num;

    -- Assign the calculated issue number before insertion
    NEW.issue_number := next_num;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 5. Attach the trigger to the table
CREATE TRIGGER trg_assign_issue_number
BEFORE INSERT ON issues
FOR EACH ROW
EXECUTE FUNCTION assign_issue_number();
