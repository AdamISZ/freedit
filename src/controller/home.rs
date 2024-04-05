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




/// Page data: `home.html`
#[derive(Template)]
#[template(path = "home.html", escape = "none")]
struct PageHome<'a> {
    page_data: PageData<'a>,
}

/// `GET /home`
pub(crate) async fn homepage(
    cookie: Option<TypedHeader<Cookie>>
) -> Result<impl IntoResponse, AppError> {
    let site_config = SiteConfig::get(&DB)?;
    let claim = cookie.and_then(|cookie| Claim::get(&DB, &cookie, &site_config));

    let page_data = PageData::new("Home", &site_config, claim, false);
    let page_home = PageHome {
        page_data,
    };

    Ok(into_response(&page_home))
}
