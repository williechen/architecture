// TODO SWAGGER:Auto-generate this file!

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::create_user,
        crate::handlers::login,
        crate::handlers::get_user,
        crate::handlers::list_users,
        crate::handlers::update_user,
        crate::handlers::delete_user,
        crate::handlers::myid,
    ),
    components(
        schemas(UserResponse, CreateUser, UpdateUser, LoginRequest)
    ),
    tags(
        (name = "users", description = "User management endpoints"),
        (name = "auth", description = "Authentication endpoints")
    )
)]
pub struct ApiDoc;
