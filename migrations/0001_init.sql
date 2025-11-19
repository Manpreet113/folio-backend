-- Skills Table
CREATE TABLE IF NOT EXISTS skills (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    category TEXT NOT NULL, -- e.g. "Languages", "Frameworks"
    status TEXT NOT NULL,   -- e.g. "Intermediate", "Learning"
    icon TEXT,              -- Store the icon name string from lucide-react
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Projects Table
CREATE TABLE IF NOT EXISTS projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    tech_stack TEXT NOT NULL, -- Store as JSON string or comma-separated
    github_url TEXT,
    demo_url TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);