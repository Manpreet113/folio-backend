-- Seed Skills (idempotent - won't fail if data already exists)
INSERT INTO skills (name, category, proficiency, icon) VALUES
('Python', 'Backend scripting, data structures, automation', 'Intermediate', 'Code2'),
('Rust', 'Systems programming, memory safety, performance', 'Learning', 'Boxes'),
('Web Development', 'TypeScript, React, Astro, Tailwind CSS', 'Intermediate', 'Globe'),
('Git & Version Control', 'GitHub workflows, branching strategies, collaboration', 'Intermediate', 'GitBranch'),
('Linux & Terminal', 'Command line proficiency, shell scripting, system tools', 'Intermediate', 'Terminal'),
('Backend Concepts', 'APIs, databases, server-side architecture', 'Learning', 'Database')
ON CONFLICT DO NOTHING;

-- Seed Projects (idempotent - won't fail if data already exists)
INSERT INTO projects (title, description, tech_stack, github_url, demo_url) VALUES
(
    'Terminal Portfolio Site', 
    'Built this minimal portfolio using React and TypeScript. Focused on clean design, responsive layout, and semantic HTML. Learning deployment and optimization along the way.',
    ARRAY['React', 'TypeScript', 'Tailwind CSS'],
    '#',
    '#'
),
(
    'CLI Task Manager',
    'Python-based command-line tool for managing daily tasks and notes. Implemented file I/O, argument parsing, and basic data persistence. Simple but functional.',
    ARRAY['Python', 'argparse', 'JSON'],
    '#',
    NULL
),
(
    'Algorithm Visualizer',
    'Interactive web app visualizing sorting algorithms. Helped solidify understanding of time complexity and algorithm behavior through visual representation.',
    ARRAY['JavaScript', 'Canvas API', 'CSS'],
    '#',
    '#'
),
(
    'Rust Systems Experiments',
    'Learning Rust through small projects: file utilities, basic HTTP parser, memory-safe data structures. Still early in the journey but enjoying the challenge.',
    ARRAY['Rust', 'Systems Programming'],
    '#',
    NULL
)
ON CONFLICT DO NOTHING;

