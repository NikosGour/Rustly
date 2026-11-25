# Rustly - Url shortener in Rust

# Features

## Phase 1 – Project Setup

- [x] Install Rust and Cargo
- [x] Create a new Rust project
- [x] Set up Axum and Tokio
- [x] Create a basic HTTP server
- [x] Add a `GET /health` endpoint
- [x] Configure environment variables
- [ ] Set up structured logging with tracing

## Phase 2 – URL Shortening

- [x] Create `POST /url` endpoint
- [x] Accept an original URL as JSON
- [ ] Validate URL format
- [x] Generate a unique short code
- [ ] Return the result as JSON
- [x] Implement `GET /url/:code` redirect endpoint
- [ ] Return 404 for unknown short codes

## Phase 3 – PostgreSQL

- [x] Set up PostgreSQL
- [x] Connect using SQLx
- [x] Create database migrations
- [x] Create the `url` table
- [x] Store shortened URLs in the database
- [x] Retrieve URLs by short code
- [ ] Implement persistent URL deletion
- [ ] Handle database errors properly

## Phase 4 – URL Management

- [ ] Implement `GET /admin/url/` endpoint
- [ ] Implement `GET /admin/url/:id` endpoint
- [ ] Implement `DELETE /admin/url/:id` endpoint
- [ ] Prevent duplicate short codes
- [ ] Add URL expiration dates
- [ ] Implement pagination
- [ ] Add filtering and sorting

## Phase 5 – Security

- [ ] Validate and normalize URLs
- [ ] Add rate limiting
- [ ] Limit URL creation requests
- [ ] Add API key authentication
- [ ] Avoid exposing sensitive database errors
- [ ] Validate request payload sizes

## Phase 6 – Performance

- [ ] Set up Redis
- [ ] Cache short-code lookups
- [ ] Implement cache expiration
- [ ] Invalidate cache when URLs are deleted
- [ ] Optimize database queries
- [ ] Add database indexes

## Phase 7 – Testing

- [ ] Write unit tests for short-code generation
- [ ] Test URL validation
- [ ] Test URL creation
- [ ] Test redirects
- [ ] Test unknown short codes
- [ ] Test expiration behavior
- [ ] Test database operations
- [ ] Write integration tests for API endpoints
- [ ] Test concurrent requests

## Phase 8 – Deployment

- [x] Create a Dockerfile
- [x] Create `docker-compose.yml`
- [x] Configure PostgreSQL in Docker
- [ ] Configure Redis in Docker
- [x] Run database migrations automatically
- [ ] Add production environment configuration
- [x] Add health checks