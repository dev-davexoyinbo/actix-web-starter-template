use std::sync::Arc;
use actix_web::{
    middleware::from_fn, web::{self, Json, ServiceConfig}, Responder
};
use app_errors::UserError;
use common::ResponseDto;
use serde::Serialize;

use crate::{
    api::auth_module::auth_extractors::AuthExtractor,
    middlewares::auth_middleware::{
        AccessControl, AccessControlCondition, AccessControlFunction
    },
};

use super::access_control_test_dtos::AccessControlTestResponseDto;

/// Handler for endpoints that require the "admin" role
pub async fn require_admin_role(
    _auth: AuthExtractor,
) -> Result<Json<ResponseDto<AccessControlTestResponseDto>>, UserError> {
    let response = AccessControlTestResponseDto {
        message: "You have successfully accessed a protected resource".to_string(),
        endpoint_name: "require_admin_role".to_string(),
        access_requirement: "Role: admin".to_string(),
    };

    Ok(Json(ResponseDto::new("Access granted".to_string(), response)))
}

/// Handler for endpoints that require the "blog:write" permission
pub async fn require_blog_write_permission(
    _auth: AuthExtractor,
) -> Result<Json<ResponseDto<AccessControlTestResponseDto>>, UserError> {
    let response = AccessControlTestResponseDto {
        message: "You have successfully accessed a protected resource".to_string(),
        endpoint_name: "require_blog_write_permission".to_string(),
        access_requirement: "Permission: blog:write".to_string(),
    };

    Ok(Json(ResponseDto::new("Access granted".to_string(), response)))
}

/// Handler for endpoints that require BOTH the "editor" role AND "blog:write" permission
pub async fn require_editor_role_and_blog_write(
    _auth: AuthExtractor,
) -> Result<Json<ResponseDto<AccessControlTestResponseDto>>, UserError> {
    let response = AccessControlTestResponseDto {
        message: "You have successfully accessed a protected resource".to_string(),
        endpoint_name: "require_editor_role_and_blog_write".to_string(),
        access_requirement: "Role: editor AND Permission: blog:write".to_string(),
    };

    Ok(Json(ResponseDto::new("Access granted".to_string(), response)))
}

/// Handler for endpoints that require EITHER the "admin" role OR "settings:update" permission
pub async fn require_admin_role_or_settings_permission(
    _auth: AuthExtractor,
) -> Result<Json<ResponseDto<AccessControlTestResponseDto>>, UserError> {
    let response = AccessControlTestResponseDto {
        message: "You have successfully accessed a protected resource".to_string(),
        endpoint_name: "require_admin_role_or_settings_permission".to_string(),
        access_requirement: "Role: admin OR Permission: settings:update".to_string(),
    };

    Ok(Json(ResponseDto::new("Access granted".to_string(), response)))
}

/// Handler for endpoints that require complex nested conditions:
/// (Role "moderator" AND Permission "comments:delete") OR (Role "admin")
pub async fn require_complex_condition(
    _auth: AuthExtractor,
) -> Result<Json<ResponseDto<AccessControlTestResponseDto>>, UserError> {
    let response = AccessControlTestResponseDto {
        message: "You have successfully accessed a protected resource".to_string(),
        endpoint_name: "require_complex_condition".to_string(),
        access_requirement: "(Role: moderator AND Permission: comments:delete) OR (Role: admin)".to_string(),
    };

    Ok(Json(ResponseDto::new("Access granted".to_string(), response)))
}

/// Handler for endpoints that require a custom function check
pub async fn require_custom_function(
    _auth: AuthExtractor,
) -> Result<Json<ResponseDto<AccessControlTestResponseDto>>, UserError> {
    let response = AccessControlTestResponseDto {
        message: "You have successfully accessed a protected resource".to_string(),
        endpoint_name: "require_custom_function".to_string(),
        access_requirement: "Custom function: Has a permission starting with 'blog:'".to_string(),
    };

    Ok(Json(ResponseDto::new("Access granted".to_string(), response)))
}

/// Handler for endpoints that require multiple permissions
pub async fn require_multiple_permissions(
    _auth: AuthExtractor,
) -> Result<Json<ResponseDto<AccessControlTestResponseDto>>, UserError> {
    let response = AccessControlTestResponseDto {
        message: "You have successfully accessed a protected resource".to_string(),
        endpoint_name: "require_multiple_permissions".to_string(),
        access_requirement: "Permissions: blog:write AND comments:write".to_string(),
    };

    Ok(Json(ResponseDto::new("Access granted".to_string(), response)))
}

/// Handler for endpoints that require any permission from a set
pub async fn require_any_content_permission(
    _auth: AuthExtractor,
) -> Result<Json<ResponseDto<AccessControlTestResponseDto>>, UserError> {
    let response = AccessControlTestResponseDto {
        message: "You have successfully accessed a protected resource".to_string(),
        endpoint_name: "require_any_content_permission".to_string(),
        access_requirement: "Any permission: blog:write OR comments:write OR blog:delete".to_string(),
    };

    Ok(Json(ResponseDto::new("Access granted".to_string(), response)))
}

/// Handler for endpoints that are only accessible to guests
pub async fn guest_only_endpoint(
) -> Result<Json<ResponseDto<AccessControlTestResponseDto>>, UserError> {
    let response = AccessControlTestResponseDto {
        message: "This endpoint can only be accessed by unauthenticated users".to_string(),
        endpoint_name: "guest_only_endpoint".to_string(),
        access_requirement: "Guest only (unauthenticated)".to_string(),
    };

    Ok(Json(ResponseDto::new("Access granted".to_string(), response)))
}

/// Handler for endpoints that require authentication but no specific role/permission
pub async fn authenticated_users_only(
    _auth: AuthExtractor,
) -> Result<Json<ResponseDto<AccessControlTestResponseDto>>, UserError> {
    let response = AccessControlTestResponseDto {
        message: "This endpoint requires authentication but no specific permissions".to_string(),
        endpoint_name: "authenticated_users_only".to_string(),
        access_requirement: "Any authenticated user".to_string(),
    };

    Ok(Json(ResponseDto::new("Access granted".to_string(), response)))
}

/// Handler for endpoints that require verified email
pub async fn require_verified_email(
    _auth: AuthExtractor,
) -> Result<Json<ResponseDto<AccessControlTestResponseDto>>, UserError> {
    let response = AccessControlTestResponseDto {
        message: "This endpoint requires a verified email address".to_string(),
        endpoint_name: "require_verified_email".to_string(),
        access_requirement: "Email verification".to_string(),
    };

    Ok(Json(ResponseDto::new("Access granted".to_string(), response)))
}

/// Documentation response for listing all available endpoints
#[derive(Serialize)]
struct EndpointDocumentation {
    path: String,
    description: String,
    access_requirement: String,
}

/// Documentation response wrapper
#[derive(Serialize)]
struct AccessControlDocsResponse {
    message: String,
    endpoints: Vec<EndpointDocumentation>,
}

/// Handler that returns documentation about all available test endpoints
pub async fn list_access_control_endpoints() -> impl Responder {
    let endpoints = vec![
        EndpointDocumentation {
            path: "/api/access-control-test/admin-role".to_string(),
            description: "Tests access control based on the admin role".to_string(),
            access_requirement: "Role: admin".to_string(),
        },
        EndpointDocumentation {
            path: "/api/access-control-test/blog-write-permission".to_string(),
            description: "Tests access control based on a specific permission".to_string(),
            access_requirement: "Permission: blog:write".to_string(),
        },
        EndpointDocumentation {
            path: "/api/access-control-test/editor-role-and-blog-write".to_string(),
            description: "Tests access control requiring both a role and a permission".to_string(),
            access_requirement: "Role: editor AND Permission: blog:write".to_string(),
        },
        EndpointDocumentation {
            path: "/api/access-control-test/admin-role-or-settings-permission".to_string(),
            description: "Tests access control requiring either a role or a permission".to_string(),
            access_requirement: "Role: admin OR Permission: settings:update".to_string(),
        },
        EndpointDocumentation {
            path: "/api/access-control-test/complex-condition".to_string(),
            description: "Tests complex nested access control conditions".to_string(),
            access_requirement: "(Role: moderator AND Permission: comments:delete) OR (Role: admin)".to_string(),
        },
        EndpointDocumentation {
            path: "/api/access-control-test/custom-function".to_string(),
            description: "Tests access control using a custom function check".to_string(),
            access_requirement: "Custom function: Has a permission starting with 'blog:'".to_string(),
        },
        EndpointDocumentation {
            path: "/api/access-control-test/multiple-permissions".to_string(),
            description: "Tests access control requiring multiple specific permissions".to_string(),
            access_requirement: "Permissions: blog:write AND comments:write".to_string(),
        },
        EndpointDocumentation {
            path: "/api/access-control-test/any-content-permission".to_string(),
            description: "Tests access control requiring any permission from a set".to_string(),
            access_requirement: "Any permission: blog:write OR comments:write OR blog:delete".to_string(),
        },
        EndpointDocumentation {
            path: "/api/access-control-test/guest-only".to_string(),
            description: "Tests access control for guest-only resources".to_string(),
            access_requirement: "Guest only (unauthenticated)".to_string(),
        },
        EndpointDocumentation {
            path: "/api/access-control-test/authenticated-only".to_string(),
            description: "Tests basic authentication without specific permissions".to_string(),
            access_requirement: "Any authenticated user".to_string(),
        },
        EndpointDocumentation {
            path: "/api/access-control-test/verified-email".to_string(),
            description: "Tests access control requiring email verification".to_string(),
            access_requirement: "Email verification".to_string(),
        },
    ];

    let response = AccessControlDocsResponse {
        message: "Available access control test endpoints".to_string(),
        endpoints,
    };

    Json(ResponseDto::new("Access control test documentation".to_string(), response))
}

/// Configure all the access control test routes
pub fn configure_access_control_routes(config: &mut ServiceConfig) {
    config.service(
        web::scope("/access-control-test")
            // Test for specific role
            .route(
                "/admin-role",
                web::get()
                    .to(require_admin_role)
                    .wrap(AccessControl::Role("admin".to_string()).into_middleware()),
            )
            // Test for specific permission
            .route(
                "/blog-write-permission",
                web::get()
                    .to(require_blog_write_permission)
                    .wrap(AccessControl::Permission("blog:write".to_string()).into_middleware()),
            )
            // Test for role AND permission (both required)
            .route(
                "/editor-role-and-blog-write",
                web::get()
                    .to(require_editor_role_and_blog_write)
                    .wrap({
                        let mut middleware = AccessControlCondition::all()
                            .add(AccessControl::Role("editor".to_string()))
                            .add(AccessControl::Permission("blog:write".to_string()))
                            .into_middleware();
                        middleware.error_message = Some("You need to be an editor with blog:write permission to access this resource".to_string());
                        middleware
                    }),
            )
            // Test for role OR permission (either one satisfies)
            .route(
                "/admin-role-or-settings-permission",
                web::get()
                    .to(require_admin_role_or_settings_permission)
                    .wrap({
                        let mut middleware = AccessControlCondition::any()
                            .add(AccessControl::Role("admin".to_string()))
                            .add(AccessControl::Permission("settings:update".to_string()))
                            .into_middleware();
                        middleware.error_message = Some("You need either admin role or settings:update permission".to_string());
                        middleware
                    }),
            )
            // Test for complex nested conditions
            .route(
                "/complex-condition",
                web::get()
                    .to(require_complex_condition)
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
                    }),
            )
            // Test for custom function-based check
            .route(
                "/custom-function",
                web::get()
                    .to(require_custom_function)
                    .wrap({
                        // Custom function that checks if user has any permission that starts with "blog:"
                        let blog_permission_check = Arc::new(|auth_info: Option<&crate::api::auth_module::auth_models::AuthInfo>| -> bool {
                            auth_info.is_some_and(|info| {
                                info.permissions.iter().any(|p| p.starts_with("blog:"))
                            })
                        });

                        let mut middleware = AccessControl::Function(
                            AccessControlFunction(blog_permission_check)
                        ).into_middleware();
                        middleware.error_message = Some("You need a blog-related permission".to_string());
                        middleware
                    }),
            )
            // Test for multiple permissions (must have all)
            .route(
                "/multiple-permissions",
                web::get()
                    .to(require_multiple_permissions)
                    .wrap({
                        let mut middleware = AccessControlCondition::all()
                            .add(AccessControl::Permission("blog:write".to_string()))
                            .add(AccessControl::Permission("comments:write".to_string()))
                            .into_middleware();
                        middleware.error_message = Some("You need both blog:write and comments:write permissions".to_string());
                        middleware
                    }),
            )
            // Test for any permission from a set
            .route(
                "/any-content-permission",
                web::get()
                    .to(require_any_content_permission)
                    .wrap({
                        let mut middleware = AccessControlCondition::any()
                            .add(AccessControl::Permission("blog:write".to_string()))
                            .add(AccessControl::Permission("comments:write".to_string()))
                            .add(AccessControl::Permission("blog:delete".to_string()))
                            .into_middleware();
                        middleware.error_message = Some("You need at least one content management permission".to_string());
                        middleware
                    }),
            )
            // Test for guest-only access
            .route(
                "/guest-only",
                web::get()
                    .to(guest_only_endpoint)
                    .wrap(AccessControl::Guest.into_middleware()),
            )
            // Test for any authenticated user (no specific permission/role)
            .route(
                "/authenticated-only",
                web::get()
                    .to(authenticated_users_only)
                    .wrap(
                        from_fn(crate::middlewares::auth_middleware::require_auth_middleware)
                    ),
            )
            // Test for email verification
            .route(
                "/verified-email",
                web::get()
                    .to(require_verified_email)
                    .wrap(
                        from_fn(crate::middlewares::auth_middleware::require_email_verification)
                    ),
            )
            // Documentation endpoint
            .route(
                "/docs",
                web::get()
                    .to(list_access_control_endpoints),
            ),
    );
}