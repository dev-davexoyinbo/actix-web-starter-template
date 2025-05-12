# Actix Web Starter Template

A robust and production-ready starter template for building REST APIs with [Actix Web](https://actix.rs/), a powerful, pragmatic, and extremely fast web framework for Rust.

![Rust](https://img.shields.io/badge/rust-1.86%2B-orange)
![Actix Web](https://img.shields.io/badge/actixweb-4.10.2-blue)
![License](https://img.shields.io/badge/license-MIT-green)

## Features

- **Authentication System**
  - Token-based authentication (not JWT)
  - User registration and login
  - Email verification
  - Role-based access control with permissions
  - Secure password handling with Argon2

- **Database Integration**
  - PostgreSQL with [SeaORM](https://www.sea-ql.org/SeaORM/) for type-safe database access
  - Migration system for database versioning
  - Entity models and relations

- **API Features**
  - Middleware for authentication, rate limiting, and CORS
  - Request validation with `validator`
  - Error handling with custom error types
  - Structured JSON responses
  - Pagination support for list endpoints

- **Messaging**
  - Kafka integration using `rdkafka` for event-driven architecture
  - Email sending capabilities with `lettre`
  - Templated emails with `askama`

- **Observability**
  - Structured logging with `tracing`, `log`, and `tracing-actix-web`
  - Health check endpoint
  - Request tracing

- **Security**
  - Password hashing with `argon2`
  - CORS configuration with `actix-cors`
  - Rate limiting via `actix-governor`

- **Background Jobs**
  - Scheduled tasks with `actix-jobs`
  - Token cleanup job for expired auth tokens

- **Development Experience**
  - Docker Compose for local development
  - Hot reloading with `bacon`
  - Comprehensive Makefile

## Project Structure

```
actix-web-starter-template/
├── app/                 # Main application code
│   ├── src/
│       ├── api/         # API endpoints and handlers
│       │   └── auth_module/  # Authentication related endpoints and middleware
│       ├── app_config/  # Application configuration
│       ├── cron_jobs/   # Background jobs
│       ├── globals/     # Global state
│       ├── handlers/    # Generic request handlers
│       ├── lib.rs       # Application entrypoint
│       ├── persistence_state.rs # Database connection state
│       └── telemetry.rs # Logging and tracing setup
├── common/              # Shared utilities and types
│   ├── src/
│       ├── app_config.rs # Configuration loading
│       ├── common_error.rs # Error handling
│       ├── dto_wrappers/ # Data transfer objects
│       ├── helpers.rs   # Utility functions
│       ├── lib.rs
│       └── password_utils.rs # Password handling with Argon2
├── entity/              # Database entities and models
│   ├── src/
│       ├── auth_tokens.rs # Authentication tokens
│       ├── permissions.rs # Permission definitions
│       ├── roles.rs     # User roles
│       ├── role_permissions.rs # Role-permission relations
│       ├── sea_orm_active_enums.rs # Enum definitions
│       └── users.rs     # User model
├── errors/              # Error handling system
│   ├── src/
│       ├── app_error.rs # Application errors
│       ├── lib.rs
│       └── user_error.rs # User-related errors
├── messaging/           # Messaging and event system
│   ├── src/
│       ├── app_message/  # Message definitions
│       │   └── email_template/ # Email templates
│       ├── messaging_client.rs # Client for sending/receiving messages
│       ├── messaging_consumer.rs # Kafka consumer wrapper
│       ├── messaging_producer.rs # Kafka producer wrapper
│       └── messaging_error.rs # Messaging-related errors
│   └── templates/       # HTML email templates (Askama)
├── migration/           # Database migrations
│   ├── src/
│       ├── m20250324_*.rs # User table migration
│       ├── m20250329_*.rs # Role and permission migrations
│       ├── m20250413_*.rs # Auth token table migration
│       └── main.rs      # Migration runner
├── tests/               # Integration tests
├── compose.yaml         # Docker configuration
├── Dockerfile           # Container definition
└── Makefile             # Build and development commands
```

## Core Technology Stack

- **Web Framework**: [Actix Web](https://actix.rs/) 4.10.2
- **Database ORM**: [SeaORM](https://www.sea-ql.org/SeaORM/) 1.1.11 with PostgreSQL
- **Authentication**: Custom token-based authentication system
- **Password Hashing**: [Argon2](https://docs.rs/argon2/) 0.5.3
- **Messaging**: [rdkafka](https://docs.rs/rdkafka/) 0.37.0
- **Email**: [lettre](https://docs.rs/lettre/) 0.11.15 with [askama](https://docs.rs/askama/) 0.14.0 templates
- **Logging**: [tracing](https://docs.rs/tracing/) 0.1.41 with [tracing-actix-web](https://docs.rs/tracing-actix-web/) 0.7.18
- **Rate Limiting**: [actix-governor](https://docs.rs/actix-governor/) 0.8.0
- **CORS**: [actix-cors](https://docs.rs/actix-cors/) 0.7.1
- **Scheduled Jobs**: [actix-jobs](https://docs.rs/actix-jobs/) 0.1.7
- **Serialization**: [serde](https://docs.rs/serde/) 1.0.219 and [serde_json](https://docs.rs/serde_json/) 1.0.140
- **Input Validation**: [validator](https://docs.rs/validator/) 0.20.0

## Getting Started

### Prerequisites

- Rust 1.86+
- Docker and Docker Compose
- PostgreSQL (or use the provided Docker container)
- Kafka (or use the provided Docker container)

### Environment Setup

Create a `.env` file in the root directory with the following variables:

```env
# App Configuration
APP_PORT=9000
APP_HOST=0.0.0.0
APP_NAME=MyApp
APP_SECRET=your-secret-key

# Database Configuration
DATABASE_URL=postgres://postgres:postgres@localhost:5434/app
TEST_DATABASE_URL=postgres://postgres:postgres@localhost:5435/app

# Messaging Configuration
KAFKA_BOOTSTRAP_SERVERS=localhost:9092
DEFAULT_EMAIL_FROM=noreply@example.com
SMTP_USERNAME=your-smtp-username
SMTP_PASSWORD=your-smtp-password
SMTP_HOST=smtp.example.com
SMTP_PORT=587
APP_KAFKA_SERVER=localhost:9092
APP_KAFKA_GROUP_ID=app-group

# For testing
TEST_APP_PORT=9001
TEST_APP_HOST=0.0.0.0
```

### Running the Application

#### Development Mode

Start the dev environment with Docker Compose:

```bash
make up-dev
```

Or in detached mode:

```bash
make up-dev-d
```

Start the application with hot reloading:

```bash
make watch
```

#### Production Mode

Build and run the application in production mode:

```bash
make up-prod-d
```

### Database Migrations

Run migrations:

```bash
make migrate
```

Refresh migrations (drop and recreate all tables):

```bash
make migrate-refresh
```

### Testing

Run tests:

```bash
make test
```

With hot reloading:

```bash
make watch-test
```

## Authentication Flow

1. **User Registration**: `POST /api/auth/register`
2. **Email Verification**: A verification email is sent to the user
3. **Verify Email**: `POST /api/auth/verify-email`
4. **Login**: `POST /api/auth/login` - Returns access token
5. **Access Protected Resources**: Include token in `Authorization: Bearer <token>` header

## Middleware Components

The template includes several middleware components:

- `auth_middleware_global` - Extracts and validates the authorization token from the request header
- `require_auth_middleware` - Ensures the user is authenticated
- `guest_middleware` - Ensures the user is not authenticated (for guest-only routes)
- `require_email_verification` - Ensures the user's email is verified

## Docker Services

The template includes the following Docker services:

- **PostgreSQL**: Main database
- **PostgreSQL Test**: Separate database for testing
- **Kafka**: Message broker
- **Kafka UI**: Web interface for Kafka
- **Traefik**: Reverse proxy (production only)
- **Watchtower**: Automatic container updates (production only)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
