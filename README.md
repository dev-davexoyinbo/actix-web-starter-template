# Actix Web Starter Template

A robust and production-ready starter template for building REST APIs with [Actix Web](https://actix.rs/), a powerful, pragmatic, and extremely fast web framework for Rust.

![Rust](https://img.shields.io/badge/rust-1.86%2B-orange)
![Actix Web](https://img.shields.io/badge/actixweb-4.10.2-blue)
![License](https://img.shields.io/badge/license-MIT-green)

## ✨ Features

### 🔐 Authentication & Authorization
- **Token-based authentication** (not JWT) with secure token generation
- **Role-Based Access Control (RBAC)** with flexible permissions system
- **Granular access control** with role and permission-based middleware
- **Email verification** system with OTP generation
- **Secure password handling** with Argon2 hashing

### 🗄️ Database Integration
- **PostgreSQL** with [SeaORM](https://www.sea-ql.org/SeaORM/) for type-safe database access
- **Comprehensive entity models** with relations
- **Database migrations** and seeders for roles, permissions, and users
- **Optimized queries** with proper indexing and foreign key constraints

### 🚀 API Features
- **RESTful API design** with comprehensive endpoints
- **Middleware ecosystem** for authentication, rate limiting, and CORS
- **Request validation** with custom DTOs and error handling
- **Pagination support** for list endpoints
- **Structured error responses** with proper HTTP status codes

### 📨 Messaging & Email System
- **Kafka integration** using `rdkafka` for event-driven architecture
- **HTML email templates** with `askama` templating engine
- **SMTP email delivery** with configurable providers
- **Background email processing** with consumer groups
- **Template-based messaging** for verification emails and notifications

### 📊 Observability & Monitoring
- **Structured logging** with `tracing`, `log`, and `tracing-actix-web`
- **Request tracing** with correlation IDs
- **Error tracking** and debugging support
- **Performance monitoring** ready

### 🛡️ Security Features
- **Rate limiting** via `actix-governor`
- **CORS configuration** for cross-origin requests
- **Request size limits** and validation
- **Secure password policies** and token management

### ⚙️ Background Processing
- **Scheduled tasks** with `actix-jobs`
- **Token cleanup job** for expired authentication tokens
- **Extensible job system** for custom background tasks

### 🏗️ Development Experience
- **Docker Compose** for local development environment
- **Comprehensive Makefile** with common tasks
- **Environment-based configuration** with `.env` support
- **Database seeding** with sample data for testing

## 📁 Project Structure

```
actix-web-starter-template/
├── app/                           # Main application code
│   ├── src/
│   │   ├── api/                   # API endpoints and handlers
│   │   │   └── auth_module/       # Authentication endpoints & middleware
│   │   │       ├── handlers.rs    # Auth endpoint handlers
│   │   │       ├── auth_service.rs # Auth business logic
│   │   │       ├── auth_dtos.rs   # Request/response DTOs
│   │   │       ├── auth_models.rs # Auth-related models
│   │   │       ├── auth_extractors.rs # Auth middleware extractors
│   │   │       └── access_control_test_handlers.rs # Access control examples
│   │   ├── middlewares/           # Custom middleware
│   │   │   └── auth_middleware/   # Authentication & authorization middleware
│   │   │       ├── simple.rs      # Basic auth middleware
│   │   │       ├── access_control.rs # Role/permission-based access control
│   │   │       ├── access_control_condition.rs # Complex access conditions
│   │   │       └── access_control_middleware.rs # Middleware implementation
│   │   ├── cron_jobs/             # Background scheduled tasks
│   │   ├── globals.rs             # Global application state
│   │   ├── app_config.rs          # Configuration management
│   │   ├── persistence_state.rs   # Database connection state
│   │   ├── telemetry.rs           # Logging and tracing setup
│   │   └── lib.rs                 # Application entry point
├── common/                        # Shared utilities and types
│   ├── src/
│   │   ├── app_config.rs          # Environment configuration loading
│   │   ├── dto_wrappers/          # Common response wrappers
│   │   ├── password_utils.rs      # Password hashing utilities
│   │   └── helpers.rs             # Common helper functions
├── entity/                        # Database entity models
│   ├── src/
│   │   ├── users.rs               # User entity with status enum
│   │   ├── roles.rs               # Role entity
│   │   ├── permissions.rs         # Permission entity
│   │   ├── user_role.rs           # User-Role junction table
│   │   ├── user_permission.rs     # User-Permission junction table
│   │   ├── role_permissions.rs    # Role-Permission junction table
│   │   ├── auth_tokens.rs         # Authentication tokens
│   │   └── links.rs               # Complex entity relationships
├── errors/                        # Error handling
│   ├── src/
│   │   ├── app_error.rs           # Application-level errors
│   │   └── user_error.rs          # User-facing errors
├── messaging/                     # Kafka & email messaging
│   ├── src/
│   │   ├── messaging_client.rs    # Main messaging client
│   │   ├── messaging_producer.rs  # Kafka message producer
│   │   ├── messaging_consumer.rs  # Kafka message consumer
│   │   └── app_message/           # Message types and templates
│   │       ├── email_message.rs   # Email message handling
│   │       └── email_template/    # HTML email templates
│   └── templates/                 # Email HTML templates
│       ├── verify-email.html      # Email verification template
│       └── hello.html             # Welcome email template
├── migration/                     # Database migrations and seeders
│   ├── src/
│   │   ├── m20250324_*_create_users_table.rs
│   │   ├── m20250329_*_create_roles_table.rs
│   │   ├── m20250329_*_create_permissions_table.rs
│   │   ├── m20250329_*_create_role_permissions_table.rs
│   │   ├── m20250413_*_create_auth_token_table.rs
│   │   ├── m20250517_*_create_user_role_table.rs
│   │   ├── m20250517_*_create_user_permission_table.rs
│   │   └── seeder/                # Database seeders
│   │       ├── role_seeder.rs     # Seed default roles
│   │       ├── permission_seeder.rs # Seed default permissions
│   │       ├── role_permission_seeder.rs # Seed role-permission mappings
│   │       └── user_seeder.rs     # Seed default users
│   └── data/                      # Seed data JSON files
│       ├── roles.json             # Default roles
│       ├── permissions.json       # Default permissions
│       ├── role_permission.json   # Role-permission mappings
│       └── users.json             # Default users
├── tests/                         # Integration tests
├── src/main.rs                    # Application main entry point
├── Cargo.toml                     # Workspace configuration
├── compose.yaml                   # Docker Compose configuration
├── Dockerfile                     # Production Docker image
├── Makefile                       # Development tasks
└── .env.example                   # Environment variables template
```

## 🚀 Getting Started

### Prerequisites

- **Rust** 1.86+ (install via [rustup](https://rustup.rs/))
- **Docker** and **Docker Compose** (for local development)
- **PostgreSQL** (if running without Docker)
- **Apache Kafka** (if running without Docker)

### Quick Start

1. **Fork and clone the repository**
   - Click the "Fork" button on GitHub to create your own copy
   - Clone your forked repository:
   ```bash
   git clone https://github.com/YOUR_USERNAME/actix-web-starter-template.git
   cd actix-web-starter-template
   ```

2. **Set up environment variables**
   ```bash
   cp .env.example .env
   # Edit .env with your configuration
   ```

3. **Start development services**
   ```bash
   make up-dev
   ```

4. **Run database migrations (includes seeding)**
   ```bash
   make migrate
   ```

5. **Start the application**
   ```bash
   make run
   ```

The API will be available at `http://localhost:9000`.

### 🚀 Quick API Testing

Once your application is running, you can:

- **View access control examples**: Visit `http://localhost:9000/api/access-control-test/docs`
- **Test authentication**: Use the seeded users (see [Default Seeded Users](#default-seeded-users))
- **Explore full API documentation**: Check out our [**Postman Collection**](https://documenter.getpostman.com/view/34761218/2sB2jAa815) with all endpoints and examples

### Environment Configuration

Create a `.env` file in the root directory with the following variables:

```env
# App Configuration
APP_PORT=9000
APP_HOST=0.0.0.0
APP_NAME=MyApp
APP_SECRET=your-secret-key-at-least-5-chars

# Database Configuration
DATABASE_URL=postgres://postgres:postgres@localhost:5434/app

# Messaging Configuration
APP_KAFKA_SERVER=localhost:9092
APP_KAFKA_GROUP_ID=app-group-id

# Email Configuration
DEFAULT_EMAIL_FROM=noreply@example.com
SMTP_USERNAME=your-smtp-username
SMTP_PASSWORD=your-smtp-password
SMTP_HOST=smtp.example.com
SMTP_PORT=587

# Admin User Password (for seeding)
ADMIN_PASSWORD=secure-admin-password

# Test Configuration (optional)
TEST_DATABASE_URL=postgres://postgres:postgres@localhost:5435/app
TEST_APP_PORT=9001
TEST_APP_HOST=0.0.0.0
```

## 📚 API Documentation

### 📋 Complete API Reference

**🚀 [View Full API Documentation on Postman](https://documenter.getpostman.com/view/34761218/2sB2jAa815)**

Our comprehensive Postman collection includes:
- **All authentication endpoints** with example requests and responses
- **Access control test endpoints** demonstrating security patterns
- **Complete request/response schemas**
- **Environment setup for easy testing**
- **Pre-configured authentication examples**

### Quick API Reference

#### Authentication Endpoints

All authentication endpoints are prefixed with `/api/auth`.

**Register a new user**
```http
POST /api/auth/register
Content-Type: application/json

{
  "name": "John Doe",
  "email": "john@example.com",
  "password": "securepassword123"
}
```

**Login**
```http
POST /api/auth/login
Content-Type: application/json

{
  "email": "john@example.com",
  "password": "securepassword123"
}
```

**Get current user info**
```http
GET /api/auth/me
Authorization: Bearer <your-token>
```

**Verify email**
```http
POST /api/auth/verify-email
Content-Type: application/json

{
  "email": "john@example.com",
  "token": "123456"
}
```

**Resend verification email**
```http
POST /api/auth/resend-verification-email
Content-Type: application/json

{
  "email": "john@example.com"
}
```

#### Access Control Test Endpoints

Test various access control patterns at `/api/access-control-test/`:

- **`GET /docs`** - View all available test endpoints
- **`GET /admin-role`** - Test role-based access (requires admin role)
- **`GET /blog-write-permission`** - Test permission-based access
- **`GET /guest-only`** - Test guest-only access (no authentication)
- **And many more...** (see [Access Control Test Endpoints](#access-control-test-endpoints))

### Testing with Seeded Users

Use these pre-seeded accounts for testing (passwords in [Default Seeded Users](#default-seeded-users)):

```bash
# Login as admin
curl -X POST http://localhost:9000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email": "admin@email.com", "password": "your-admin-password"}'

# Login as editor  
curl -X POST http://localhost:9000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email": "bob.smith@example.com", "password": "editorpassword123"}'
```

## 🔐 Access Control System

The template provides a powerful and flexible access control system with multiple layers of security, supporting role-based access control (RBAC) with granular permissions.

### Access Control Architecture

The system uses a comprehensive RBAC model with:

- **Users** - Individual accounts with authentication
- **Roles** - Named groups of permissions (e.g., admin, editor, moderator)  
- **Permissions** - Granular access rights (e.g., blog:write, user:manage)
- **Flexible assignments** - Users can have both roles and individual permissions
- **Middleware-based enforcement** - Route-level access control
- **Complex conditions** - Support for AND/OR logic and custom functions

### Database Schema

```
Users (1) ────→ (*) UserRole (*) ←──── (1) Roles
  │                                       │
  │                                       │
  ↓                                       ↓
  (*) UserPermission (*) ←─────→ (1) RolePermissions
  │                                       │
  │                                       │
  ↓                                       ↓  
  (1) ←──────── (*) Permissions ←──────── (1)
```

### Default Roles and Permissions

The system seeds the following RBAC structure:

#### Roles (`migration/data/roles.json`)
- **admin** - Administrator with full access to the system
- **editor** - Editor with permissions to manage content
- **moderator** - Moderator with permissions to manage comments and notifications  
- **viewer** - Viewer with read-only access to content and analytics
- **user** - Regular user with limited access

#### Permissions (`migration/data/permissions.json`)
- **blog:read**, **blog:write**, **blog:delete** - Blog management
- **user:manage** - User account management
- **settings:update** - System settings management
- **comments:read**, **comments:write**, **comments:delete** - Comment management
- **analytics:view** - Analytics access
- **notifications:manage** - Notification management

#### Role-Permission Mappings (`migration/data/role_permission.json`)
- **admin**: All permissions (full system access)
- **editor**: blog:read, blog:write, comments:read, comments:write
- **moderator**: comments:read, comments:delete, notifications:manage
- **viewer**: blog:read, comments:read, analytics:view
- **user**: blog:read, comments:read, comments:write

### Access Control Test Endpoints

The template includes comprehensive access control examples at `/api/access-control-test/docs` that demonstrate all access control patterns:

#### Available Test Endpoints

| Endpoint | Access Requirement | Description |
|----------|-------------------|-------------|
| `/api/access-control-test/admin-role` | Role: admin | Simple role-based access |
| `/api/access-control-test/blog-write-permission` | Permission: blog:write | Permission-based access |
| `/api/access-control-test/editor-role-and-blog-write` | Role: editor AND Permission: blog:write | Combined requirements |
| `/api/access-control-test/admin-role-or-settings-permission` | Role: admin OR Permission: settings:update | Alternative requirements |
| `/api/access-control-test/complex-condition` | (Role: moderator AND Permission: comments:delete) OR (Role: admin) | Complex nested conditions |
| `/api/access-control-test/custom-function` | Has permission starting with 'blog:' | Custom function checks |
| `/api/access-control-test/multiple-permissions` | Permissions: blog:write AND comments:write | Multiple permissions |
| `/api/access-control-test/any-content-permission` | Any of: blog:write OR comments:write OR blog:delete | Any from set |
| `/api/access-control-test/guest-only` | Guest only (unauthenticated) | Unauthenticated access |
| `/api/access-control-test/authenticated-only` | Any authenticated user | Basic authentication |
| `/api/access-control-test/verified-email` | Email verification required | Email verification |

### Access Control Middleware Usage

#### 1. Simple Role Check
```rust
use crate::middlewares::auth_middleware::AccessControl;

.route("/admin", web::get().to(admin_handler))
.wrap(AccessControl::Role("admin".to_string()).into_middleware())
```

#### 2. Permission Check
```rust
.route("/blog", web::post().to(create_blog))
.wrap(AccessControl::Permission("blog:write".to_string()).into_middleware())
```

#### 3. Multiple Requirements (ALL - AND Logic)
```rust
use crate::middlewares::auth_middleware::AccessControlCondition;

.route("/editor-blog", web::post().to(editor_blog_handler))
.wrap({
    let mut middleware = AccessControlCondition::all()
        .add(AccessControl::Role("editor".to_string()))
        .add(AccessControl::Permission("blog:write".to_string()))
        .into_middleware();
    middleware.error_message = Some("You need editor role with blog:write permission".to_string());
    middleware
})
```

#### 4. Alternative Requirements (ANY - OR Logic)
```rust
.route("/settings", web::put().to(update_settings))
.wrap({
    let mut middleware = AccessControlCondition::any()
        .add(AccessControl::Role("admin".to_string()))
        .add(AccessControl::Permission("settings:update".to_string()))
        .into_middleware();
    middleware.error_message = Some("You need either admin role or settings:update permission".to_string());
    middleware
})
```

#### 5. Complex Nested Conditions
```rust
// (moderator AND comments:delete) OR admin
.route("/complex", web::delete().to(complex_handler))
.wrap({
    let mut middleware = AccessControlCondition::any()
        .add(AccessControl::Role("admin".to_string()))
        .add(
            AccessControlCondition::all()
                .add(AccessControl::Role("moderator".to_string()))
                .add(AccessControl::Permission("comments:delete".to_string()))
        )
        .into_middleware();
    middleware.error_message = Some("You need to be either an admin or a moderator with comments:delete permission".to_string());
    middleware
})
```

#### 6. Custom Function Checks
```rust
use std::sync::Arc;
use crate::middlewares::auth_middleware::{AccessControl, AccessControlFunction};

.route("/custom", web::get().to(custom_handler))
.wrap({
    // Custom logic for access control
    let custom_check = Arc::new(|auth_info: Option<&AuthInfo>| -> bool {
        auth_info.is_some_and(|info| {
            // Check if user has any blog-related permission
            info.permissions.iter().any(|p| p.starts_with("blog:"))
        })
    });

    let mut middleware = AccessControl::Function(
        AccessControlFunction(custom_check)
    ).into_middleware();
    middleware.error_message = Some("You need a blog-related permission".to_string());
    middleware
})
```

#### 7. Guest-Only Access
```rust
// Only accessible to unauthenticated users
.route("/register", web::post().to(register_handler))
.wrap(AccessControl::Guest.into_middleware())
```

#### 8. Email Verification Requirement
```rust
use actix_web::middleware::from_fn;

.route("/verified-only", web::get().to(verified_handler))
.wrap(from_fn(crate::middlewares::auth_middleware::require_email_verification))
```

#### 9. Basic Authentication (Any Authenticated User)
```rust
.route("/profile", web::get().to(profile_handler))
.wrap(from_fn(crate::middlewares::auth_middleware::require_auth_middleware))
```

### Custom Access Control Functions

You can create complex custom access control logic:

```rust
// Example: Time-based access control
let business_hours_check = Arc::new(|auth_info: Option<&AuthInfo>| -> bool {
    use chrono::Utc;
    
    let now = Utc::now();
    let hour = now.hour();
    
    // Allow access only during business hours (9 AM - 5 PM UTC)
    if hour < 9 || hour >= 17 {
        return false;
    }
    
    // Also require authentication
    auth_info.is_some()
});

.route("/business-hours", web::get().to(handler))
.wrap(AccessControl::Function(AccessControlFunction(business_hours_check)).into_middleware())
```

```rust
// Example: Resource ownership check
let resource_owner_check = Arc::new(|auth_info: Option<&AuthInfo>| -> bool {
    // This would typically need access to request context
    // For real ownership checks, consider implementing custom middleware
    auth_info.is_some_and(|info| {
        info.permissions.contains(&"resource:owner".to_string())
    })
});
```

### Access Control Best Practices

1. **Principle of Least Privilege**: Grant minimal permissions necessary
2. **Role Hierarchy**: Design roles with clear hierarchy (user < viewer < editor < moderator < admin)
3. **Permission Naming**: Use consistent naming patterns (`resource:action`)
4. **Error Messages**: Provide clear, user-friendly error messages
5. **Testing**: Use the test endpoints to verify access control behavior
6. **Logging**: Monitor access control failures for security auditing

### Testing Access Control

Use the built-in test endpoints to verify your access control setup:

```bash
# Get documentation of all test endpoints
curl http://localhost:9000/api/access-control-test/docs

# Test admin role access (requires authentication)
curl -H "Authorization: Bearer <token>" \
     http://localhost:9000/api/access-control-test/admin-role

# Test permission-based access
curl -H "Authorization: Bearer <token>" \
     http://localhost:9000/api/access-control-test/blog-write-permission

# Test guest-only access (no auth header)
curl http://localhost:9000/api/access-control-test/guest-only
```

### Extending Access Control

#### Adding New Permissions

1. **Add to permissions.json**:
   ```json
   {
     "name": "reports:generate",
     "description": "Permission to generate reports"
   }
   ```

2. **Update role-permission mappings**:
   ```json
   {
     "role": "admin", 
     "permissions": ["reports:generate", "...existing permissions"]
   }
   ```

3. **Run migrations**: `make migrate`

4. **Apply to routes**:
   ```rust
   .route("/reports", web::post().to(generate_report))
   .wrap(AccessControl::Permission("reports:generate".to_string()).into_middleware())
   ```

#### Creating Custom Middleware

For complex scenarios, you can create custom middleware:

```rust
// Example: Custom resource-based access control
use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web::middleware::{from_fn, Next};

async fn resource_ownership_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // Extract resource ID from path
    let resource_id: i64 = req.match_info()
        .get("id")
        .and_then(|id| id.parse().ok())
        .ok_or_else(|| UserError::from_message("Invalid resource ID", StatusCode::BAD_REQUEST))?;
    
    // Get auth info from request extensions
    let auth_info = req.extensions().get::<AuthInfo>()
        .ok_or_else(|| UserError::from_message("Authentication required", StatusCode::UNAUTHORIZED))?;
    
    // Check ownership (this would involve database query)
    if !check_resource_ownership(auth_info.user_id, resource_id).await? {
        return Err(UserError::from_message("Access denied", StatusCode::FORBIDDEN).into());
    }
    
    next.call(req).await
}

// Apply to routes
.route("/posts/{id}", web::put().to(update_post))
.wrap(from_fn(resource_ownership_middleware))
```

## 📧 Email & Messaging System

The template includes a comprehensive messaging system built on **Apache Kafka** for reliable event-driven communication and **SMTP** for email delivery. All email sending is processed asynchronously through Kafka topics.

### Architecture Overview

The messaging system consists of:

- **`MessagingClient`** - Main interface for sending messages
- **`MessagingProducer`** - Kafka producer for publishing messages  
- **`MessagingConsumer`** - Kafka consumer for processing messages
- **`MessagingConsumerWrapper`** - Manages multiple consumer groups and SMTP transport
- **Email Templates** - HTML templates using Askama templating engine

### How Email Processing Works

1. **Application sends email** → Publishes message to Kafka topic
2. **Kafka consumer receives** → Message deserialized and processed
3. **Email rendered** → HTML template rendered with data
4. **SMTP delivery** → Email sent via configured SMTP provider
5. **Message committed** → Kafka message marked as processed

### Email Templates

Email templates are located in `messaging/templates/` and use the [Askama](https://github.com/djc/askama) templating engine.

#### Existing Templates

- **`hello.html`** - Welcome/greeting emails (uses `HelloEmailTemplate`)
- **`verify-email.html`** - Email verification with OTP (uses `VerifyEmailTemplate`)

#### Template Structure

```rust
// messaging/src/app_message/email_template/verify_email_template.rs
#[derive(Template, Serialize, Deserialize, Debug, Clone)]
#[template(path = "verify-email.html")]
pub struct VerifyEmailTemplate {
    pub name: String,    // User's name
    pub otp: String,     // 6-digit verification code
}
```

#### Creating New Email Templates

1. **Create the HTML template**
   ```html
   <!-- messaging/templates/welcome.html -->
   <!doctype html>
   <html>
   <head>
       <meta charset="UTF-8">
       <title>Welcome to {{app_name}}</title>
   </head>
   <body>
       <h1>Welcome, {{user_name}}!</h1>
       <p>Thank you for joining {{app_name}}.</p>
       <p>Your account ID is: {{user_id}}</p>
   </body>
   </html>
   ```

2. **Create the template struct**
   ```rust
   // messaging/src/app_message/email_template/welcome_template.rs
   use askama::Template;
   use serde::{Deserialize, Serialize};
   use super::EmailTemplate;

   #[derive(Template, Serialize, Deserialize, Debug, Clone)]
   #[template(path = "welcome.html")]
   pub struct WelcomeTemplate {
       pub app_name: String,
       pub user_name: String,
       pub user_id: i64,
   }

   impl From<WelcomeTemplate> for EmailTemplate {
       fn from(val: WelcomeTemplate) -> EmailTemplate {
           EmailTemplate::Welcome(val)
       }
   }
   ```

3. **Add to EmailTemplate enum**
   ```rust
   // messaging/src/app_message/email_template/mod.rs
   pub use welcome_template::*; // Add import

   #[derive(Clone, Debug, Deserialize, Serialize)]
   pub enum EmailTemplate {
       Hello(HelloEmailTemplate),
       VerifyEmail(VerifyEmailTemplate),
       Welcome(WelcomeTemplate), // Add new variant
   }

   impl EmailTemplate {
       pub fn render(&self) -> Result<String, MessagingError> {
           match self {
               EmailTemplate::Hello(template) => template.render().map_err(|_| {
                   MessagingError::TemplateError("Failed to render HelloEmailTemplate".to_string())
               }),
               EmailTemplate::VerifyEmail(template) => template.render().map_err(|_| {
                   MessagingError::TemplateError("Failed to render VerifyEmailTemplate".to_string())
               }),
               EmailTemplate::Welcome(template) => template.render().map_err(|_| {
                   MessagingError::TemplateError("Failed to render WelcomeTemplate".to_string())
               }),
           }
       }
   }
   ```

### Sending Emails

The system provides a simple interface for sending emails through the Kafka messaging system:

```rust
use messaging::{AppMessageTopic, AppMessageWrapper, EmailMessage, VerifyEmailTemplate};
use crate::globals;

async fn send_verification_email(user_name: &str, user_email: &str, otp: &str) -> Result<(), Error> {
    let messaging_client = globals::messaging::get()?;
    let app_config = app_config::get()?;

    let email_message = AppMessageWrapper {
        key: None, // Optional: use for message ordering
        topic: AppMessageTopic::GeneralEmail, // or PriorityEmail for urgent emails
        message: EmailMessage {
            from: format!("{} <{}>", app_config.app.name, app_config.messaging.default_email_from),
            to: format!("{} <{}>", user_name, user_email),
            reply_to: None, // Optional reply-to address
            subject: "Verify your email address".to_string(),
            template: VerifyEmailTemplate {
                name: user_name.to_string(),
                otp: otp.to_string(),
            }.into(),
        }.into(),
    };

    // Send message to Kafka (non-blocking)
    messaging_client
        .read()
        .await
        .send_message(email_message)
        .await?;

    Ok(())
}
```

#### Email Sending Best Practices

1. **Use appropriate topics**: `GeneralEmail` for regular emails, `PriorityEmail` for urgent communications
2. **Include context**: Provide meaningful message keys for ordered delivery when needed
3. **Handle errors**: The messaging system provides detailed error information
4. **Template validation**: Ensure your template variables match the struct fields
5. **SMTP configuration**: Test SMTP settings in development environment

### Kafka Message Topics

The system defines different topics for message prioritization and processing:

```rust
// messaging/src/app_message/app_message_topic.rs
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum AppMessageTopic {
    GeneralEmail,    // Regular email notifications (processed by general consumer)
    PriorityEmail,   // High-priority emails (processed by dedicated consumer)
}
```

Each topic has its own consumer group for parallel processing and fault tolerance.

### Message Consumer Configuration

The messaging system automatically processes messages in the background with separate consumer groups:

```rust
// Consumer groups are configured in messaging_consumer_wrapper.rs
let topic_groups: Vec<Vec<AppMessageTopic>> = vec![
    vec![AppMessageTopic::GeneralEmail],    // Consumer group 1
    vec![AppMessageTopic::PriorityEmail],   // Consumer group 2
];
```

**Key Features:**
- **Parallel processing** - Multiple consumer groups handle different message types
- **Fault tolerance** - Failed messages can be retried
- **Message ordering** - Use message keys to ensure ordered delivery within partitions
- **Automatic commit** - Messages are committed only after successful processing
- **Graceful shutdown** - Consumers can be stopped cleanly

### Email Verification System

The template includes a complete email verification system:

#### Registration Flow
1. User registers → Account created with `email_verified_at: null`
2. System generates 6-digit OTP → Stored in `auth_tokens` table
3. Verification email sent → OTP included in email template
4. User submits OTP → Account verified and `email_verified_at` updated

#### API Endpoints
```bash
# Verify email with OTP
POST /api/auth/verify-email
{
  "email": "user@example.com",
  "token": "123456"
}

# Resend verification email
POST /api/auth/resend-verification-email
{
  "email": "user@example.com"  
}
```

#### Email Verification Middleware
```rust
// Require email verification for protected routes
.route("/protected", web::get().to(handler))
.wrap(from_fn(require_email_verification))
```

### SMTP Configuration

Configure your SMTP provider in `.env`:

```env
# Email Configuration
DEFAULT_EMAIL_FROM=noreply@yourapp.com
SMTP_USERNAME=your-smtp-username
SMTP_PASSWORD=your-smtp-password
SMTP_HOST=smtp.gmail.com        # or your SMTP provider
SMTP_PORT=587                   # or 465 for SSL

# Kafka Configuration  
APP_KAFKA_SERVER=localhost:9092
APP_KAFKA_GROUP_ID=your-app-consumer-group
```

#### Popular SMTP Providers

| Provider | SMTP Host | Port | Security |
|----------|-----------|------|----------|
| Gmail | smtp.gmail.com | 587 | TLS |
| SendGrid | smtp.sendgrid.net | 587 | TLS |
| Mailgun | smtp.mailgun.org | 587 | TLS |
| AWS SES | email-smtp.region.amazonaws.com | 587 | TLS |
| Outlook | smtp-mail.outlook.com | 587 | TLS |

### Development & Testing

For local development, you can:

1. **Use a test SMTP service** like [Mailtrap](https://mailtrap.io/) or [MailHog](https://github.com/mailhog/MailHog)
2. **Check Kafka UI** at `http://localhost:8080` to monitor message flow
3. **View email templates** by rendering them directly in tests

```rust
// Example: Test email template rendering
#[test]
fn test_verify_email_template() {
    let template = VerifyEmailTemplate {
        name: "Test User".to_string(),
        otp: "123456".to_string(),
    };
    
    let html = template.render().unwrap();
    assert!(html.contains("Test User"));
    assert!(html.contains("123456"));
}
```

## 🗄️ Database Schema & Migrations

### Entity Relationships

The database uses a comprehensive RBAC schema:

```sql
Users (1) -----> (*) UserRole (*) <----- (1) Roles
  |                                         |
  |                                         |
  v                                         v
  (*) UserPermission (*) <---------> (1) RolePermissions
  |                                         |
  |                                         |
  v                                         v
  (1) <------------ (*) Permissions <----- (1)
```

### Running Migrations

```bash
# Run all pending migrations
make migrate

# Reset database (drop and recreate)
make db-reset

# Create a new migration
cargo run --bin migration -- generate <migration_name>
```

### Seeding Data

The template includes comprehensive seeders for initial data that run automatically when you execute migrations. **There is no separate `make seed` command** - seeding happens as part of the migration process.

```bash
# Run migrations (includes seeding)
make migrate

# The seeder runs as the last "migration" and populates:
# 1. Default roles (admin, editor, moderator, viewer, user)
# 2. Default permissions (blog:*, user:*, comments:*, etc.)
# 3. Role-permission mappings
# 4. Test users with assigned roles and permissions
```

#### Seeder Structure

The seeding system is located in `migration/src/seeder/` and consists of:

- **`role_seeder.rs`** - Seeds roles from `migration/data/roles.json`
- **`permission_seeder.rs`** - Seeds permissions from `migration/data/permissions.json`
- **`role_permission_seeder.rs`** - Maps roles to permissions from `migration/data/role_permission.json`
- **`user_seeder.rs`** - Seeds test users from `migration/data/users.json`

The seeders run in this order (defined in `migration/src/seeder/mod.rs`):
1. `RoleSeeder` → 2. `PermissionSeeder` → 3. `RolePermissionSeeder` → 4. `UserSeeder`

### Default Seeded Users

The system seeds the following test users (from `migration/data/users.json`):

| Email | Role | Password | Additional Permissions |
|-------|------|---------|----------------------|
| admin@email.com | admin | ADMIN_PASSWORD (env var) | Full access via role |
| bob.smith@example.com | editor | editorpassword123 | settings:update |
| charlie.brown@example.com | moderator | moderatorpassword456 | blog:write |
| diana.prince@example.com | viewer | viewerpassword789 | comments:delete |
| eve.adams@example.com | user, viewer | userpassword101 | notifications:manage |

## ⚙️ Background Jobs & Scheduled Tasks

The template includes a flexible background job system using `actix-jobs`.

### Existing Jobs

#### Token Cleanup Job
Automatically removes expired authentication tokens to keep the database clean.

```rust
// app/src/cron_jobs/delete_expired_auth_tokens_job.rs
impl Job for DeleteExpiredAuthTokensJob {
    fn schedule(&self) -> Schedule {
        Schedule::from_str("0 0 * * * *").unwrap() // Every hour
    }

    fn handle(&self, _ctx: JobContext) -> JobFuture {
        Box::pin(async move {
            // Cleanup logic
        })
    }
}
```

### Adding Custom Jobs

1. **Create a new job struct**
   ```rust
   // app/src/cron_jobs/my_custom_job.rs
   use actix_jobs::{Job, JobContext, JobFuture, Schedule};
   use std::str::FromStr;

   pub struct MyCustomJob;

   impl Job for MyCustomJob {
       fn schedule(&self) -> Schedule {
           Schedule::from_str("0 */30 * * * *").unwrap() // Every 30 minutes
       }

       fn handle(&self, _ctx: JobContext) -> JobFuture {
           Box::pin(async move {
               println!("Running custom job!");
               // Your job logic here
           })
       }
   }
   ```

2. **Register the job**
   ```rust
   // app/src/cron_jobs/mod.rs
   impl CronJob {
       fn jobs() -> Vec<Box<dyn Job>> {
           vec![
               Box::new(DeleteExpiredAuthTokensJob),
               Box::new(MyCustomJob), // Add your job
           ]
       }
   }
   ```

## 🧪 Testing

### Running Tests

```bash
# Run all tests
make test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_health_check
```

### Test Environment

Tests use a separate database configuration:

```env
TEST_DATABASE_URL=postgres://postgres:postgres@localhost:5435/app
TEST_APP_PORT=9001
```

### Example Test

```rust
#[tokio::test]
async fn test_register_user() {
    let app = spawn_app().await;
    
    let response = app.post_register(&serde_json::json!({
        "name": "Test User",
        "email": "test@example.com",
        "password": "testpassword123"
    })).await;
    
    assert_eq!(response.status(), 200);
}
```

## 🐳 Docker & Deployment

### Development Environment

```bash
# Start all services (PostgreSQL, Kafka, Kafka UI)
make up-dev

# Start in detached mode
make up-dev-d

# Stop all services
make down
```

### Production Deployment

The template includes production-ready Docker configuration with:

- **Multi-stage build** for optimized image size
- **Traefik reverse proxy** with automatic HTTPS
- **Watchtower** for automatic updates
- **Health checks** and monitoring

```bash
# Build production image
docker build -t my-app:latest .

# Deploy with Docker Compose
docker-compose --profile prod up -d
```

### Environment-Specific Configuration

- **Development** - Full logging, hot reload
- **Test** - Separate database, test fixtures
- **Production** - Optimized builds, security headers, monitoring

## 🛠️ Development Commands

The included `Makefile` provides convenient commands for development, testing, and deployment:

### Docker & Environment Commands
```bash
make help           # Show all available commands with descriptions
make up-dev         # Start development environment (PostgreSQL, Kafka, Kafka UI)
make up-dev-d       # Start development environment in detached mode  
make down           # Stop all Docker services
make up-test        # Start test environment
make up-test-d      # Start test environment in detached mode
make up-prod        # Start production environment
make up-prod-d      # Start production environment in detached mode
```

### Database Commands  
```bash
make migrate        # Run all pending database migrations (includes seeding)
make migrate-refresh # Reset and re-run all migrations  
make migrate-test   # Run migrations on test database
```

### Development & Building
```bash
make build          # Build the application with cargo
make run            # Run the application (uses bacon for live reload)
make watch          # Watch mode with automatic restart on file changes
make watch-test     # Watch mode for running tests
make clean          # Clean build artifacts and target directory
```

### Testing & Quality
```bash
make test           # Run all tests with proper environment variables
make coverage       # Generate test coverage report with tarpaulin
make audit          # Run security audit on dependencies
make check          # Run full code quality checks (check, clippy, fmt)
```

### Key Development Tools

- **[bacon](https://github.com/Canop/bacon)** - Background rust code checker for live reload
- **sea-orm-cli** - Database migrations and entity generation
- **Docker Compose** - Multi-service development environment
- **cargo-tarpaulin** - Code coverage reporting  
- **cargo-audit** - Security vulnerability scanning

### Development Workflow

1. **Setup environment**:
   ```bash
   cp .env.example .env  # Configure environment variables
   make up-dev-d         # Start development services
   make migrate          # Set up database with seeders
   ```

2. **Start development**:
   ```bash
   make watch            # Start app with auto-reload
   # In another terminal:
   make watch-test       # Run tests in watch mode
   ```

3. **Access services**:
   - **Application**: http://localhost:9000
   - **Kafka UI**: http://localhost:8080
   - **PostgreSQL**: localhost:5434 (dev), localhost:5435 (test)

### Environment Variables

The application uses environment-based configuration. Key variables:

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `APP_PORT` | Server port | 9000 | No |
| `APP_HOST` | Server host | 0.0.0.0 | No |
| `APP_NAME` | Application name | Celeris | No |
| `APP_SECRET` | Secret key for hashing | - | **Yes** |
| `DATABASE_URL` | PostgreSQL connection string | - | **Yes** |
| `APP_KAFKA_SERVER` | Kafka bootstrap servers | localhost:9092 | **Yes** |
| `APP_KAFKA_GROUP_ID` | Kafka consumer group ID | - | **Yes** |
| `DEFAULT_EMAIL_FROM` | Default sender email | - | **Yes** |
| `SMTP_USERNAME` | SMTP username | - | **Yes** |
| `SMTP_PASSWORD` | SMTP password | - | **Yes** |
| `SMTP_HOST` | SMTP server host | - | **Yes** |
| `SMTP_PORT` | SMTP server port | 587 | **Yes** |
| `ADMIN_PASSWORD` | Admin user password for seeding | - | **Yes** |

### Test Environment Variables
| Variable | Description |
|----------|-------------|
| `TEST_DATABASE_URL` | Test database connection |
| `TEST_APP_PORT` | Test server port |
| `TEST_APP_HOST` | Test server host |

## 🔧 Configuration

### App Configuration Structure

The application uses a hierarchical configuration system defined in `common/src/app_config.rs`:

```rust
pub struct AppConfig {
    pub app: AppAppConfig,           // Server settings (port, host, name, secret)
    pub db: DbConfig,                // Database connection settings
    pub password: PasswordConfig,    // Password hashing configuration  
    pub messaging: MessagingConfig,  // Kafka & SMTP settings
}
```

### Configuration Loading

Configuration is loaded from environment variables with automatic type conversion and validation. The system supports:

- **Environment variable mapping** - Automatic loading from `.env` files
- **Type safety** - Strong typing with `serde` deserialization
- **Validation** - Built-in validation for required fields
- **Default values** - Sensible defaults for optional settings

### Production Deployment Considerations

When deploying to production:

1. **Security**: Use strong, randomly generated values for `APP_SECRET`
2. **Database**: Use a managed PostgreSQL service with SSL
3. **Kafka**: Configure Kafka with proper authentication and SSL
4. **SMTP**: Use a reliable email service provider
5. **Monitoring**: Enable structured logging and monitoring
6. **Environment**: Never commit `.env` files to version control

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Code Style

- Use `cargo fmt` for code formatting
- Run `cargo clippy` for linting
- Follow Rust naming conventions
- Add documentation for public APIs
- Include tests for new features

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Actix Web](https://actix.rs/) - The web framework
- [SeaORM](https://www.sea-ql.org/SeaORM/) - The ORM
- [Askama](https://github.com/djc/askama) - Template engine
- [rdkafka](https://github.com/fede1024/rust-rdkafka) - Kafka client

## 📞 Support

If you have any questions or need help with the template:

1. Check the [documentation](#-api-documentation)
2. Review the [examples](#access-control-test-endpoints)
3. Open an issue on GitHub
4. Check the existing discussions

---

**Happy coding!** 🦀✨
