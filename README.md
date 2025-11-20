# Portfolio Backend

Rust backend using [axum](https://crates.io/crates/axum) that provides:
- **Contact form** endpoint forwarding to Resend API
- **JWT authentication** with Argon2 password hashing
- **CRUD APIs** for Skills and Projects
- **PostgreSQL database** with SQLx migrations

## Tech Stack

- **Framework**: Axum 0.7
- **Database**: PostgreSQL with SQLx
- **Auth**: JWT + Argon2
- **Email**: Resend API
- **Deployment**: Railway

## Project Structure

```
src/
├── main.rs              # Server entry point
├── config.rs            # App configuration
├── db.rs                # Database connection
├── error.rs             # Error handling
├── auth_middleware.rs   # JWT middleware
├── models/              # Data models
│   ├── auth.rs
│   └── content.rs
└── routes/              # API endpoints
    ├── auth.rs          # Login
    ├── contact.rs       # Contact form
    └── content.rs       # Skills & Projects CRUD
```

## Prerequisites

- Rust toolchain (stable)
- PostgreSQL database
- Resend API key

## Environment Variables

Create a `.env` file with:

```env
DATABASE_URL="postgresql://user:password@host:5432/dbname"
RESEND_API_KEY="re_..."
FROM_EMAIL="Your Site <sender@domain.com>"
TO_EMAIL="your-email@example.com"
FRONTEND_URL="http://localhost:5173"
JWT_SECRET="your-secret-key"
```

## Local Development

```sh
# Install dependencies
cargo build

# Run migrations
cargo sqlx migrate run

# Start server
cargo run

# Server runs on http://localhost:3001
```

## API Endpoints

### Public Endpoints
- `GET /api/skills` - List all skills
- `GET /api/projects` - List all projects
- `POST /api/login` - Authenticate user
- `POST /api/contact` - Send contact form

### Protected Endpoints (require JWT)
- `POST /api/skills` - Create skill
- `DELETE /api/skills/:id` - Delete skill
- `POST /api/projects` - Create project
- `DELETE /api/projects/:id` - Delete project

## Creating Admin User

Generate password hash:
```sh
cargo run --bin hash_password "your-password"
```

Then insert into database:
```sql
INSERT INTO users (username, password_hash) 
VALUES ('admin', '$argon2id$...');
```

## Deployment

Deployed on Railway with automatic migrations on startup.

**Railway Environment Variables:**
- `DATABASE_URL` (auto-configured)
- `RESEND_API_KEY`
- `FROM_EMAIL`
- `TO_EMAIL`
- `FRONTEND_URL` (production: `https://manpreet.tech`)
- `JWT_SECRET`

## Notes

- CORS configured for production frontend
- Rate limiting disabled for Railway proxy compatibility
- Migrations run automatically on server start
