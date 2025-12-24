// TODO SWAGGER:Auto-generate this file!

use crate::logic::common;

use utoipa::OpenApi;
#[derive(OpenApi)]
#[openapi(
    paths(
        common::get_cache,
        common::health_check,
    ),
    components(
        schemas(),
    ),
    tags(
        (name = "common", description = "Common API endpoints"),
    )
)]
pub struct ApiDoc;
