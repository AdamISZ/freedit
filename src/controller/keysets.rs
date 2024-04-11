use super::{
    into_response, 
    meta_handler::PageData,
    Claim, SiteConfig,
};
use crate::{error::AppError, DB};
use askama::Template;
use axum::response::IntoResponse;
use axum_extra::{
    headers::Cookie,
    TypedHeader,
};




/// Page data: `keysets.html`
#[derive(Template)]
#[template(path = "keysets.html", escape = "none")]
struct PageKeysets<'a> {
    page_data: PageData<'a>,
}

/// `GET /home`
pub(crate) async fn keysets(
    cookie: Option<TypedHeader<Cookie>>
) -> Result<impl IntoResponse, AppError> {
    let site_config = SiteConfig::get(&DB)?;
    let claim = cookie.and_then(|cookie| Claim::get(&DB, &cookie, &site_config));

    let page_data = PageData::new("Home", &site_config, claim, false);
    let page_keysets = PageKeysets {
        page_data,
    };

    Ok(into_response(&page_keysets))
}
