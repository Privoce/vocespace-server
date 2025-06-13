use crate::api::license::{
    clear_license, create_license, get_license, get_license_by_domain, update_license,
};
use salvo::{cors::Cors, http::Method, Router};

pub fn build() -> Router {
    let cors = Cors::new()
        .allow_origin("*") // 允许所有来源
        .allow_methods(vec![Method::GET, Method::POST, Method::DELETE, Method::PUT]) // 允许的方法
        .allow_headers("*") // 允许所有请求头
        .expose_headers("content-disposition") // 暴露特定响应头
        .into_handler();

    Router::with_path("api")
        .hoop(cors)
        .push(
            Router::with_path("license")
                .post(create_license)
                .delete(clear_license)
                .put(update_license)
                .push(Router::with_path("domains/{value}").get(get_license_by_domain))
                .push(Router::with_path("{value}").get(get_license)),
        )
        .push(
            Router::with_path("rooms")
                .get(crate::api::rooms::get_rooms)
                .post(crate::api::rooms::awake_or_create_room)
                .put(crate::api::rooms::update_room),
        )
        .push(
            Router::with_path("s3")
                .push(Router::with_path("{room}").get(crate::api::s3::get_room_records))
                .push(Router::with_path("connect").get(crate::api::s3::s3_connect)),
        )
}
