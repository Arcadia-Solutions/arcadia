use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(info(title = "arcadia-tracker API",), paths(), components(schemas(),))]
pub struct ApiDoc;
