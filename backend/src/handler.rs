use actix_web::{
    get, post,
    web::{self, Data},
    HttpResponse, Responder,
};
use chrono::Utc;
use serde::Serialize;
use url::Url;
use uuid::Uuid;

use crate::{
    model::{AppState, Post, QueryOptions},
    response::PostListResponse,
};

#[derive(Serialize)]
pub struct GenericResponse {
    status: String,
    message: String,
}

#[derive(Serialize)]
pub struct CreatePostResponse {
    status: String,
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_check)
        .service(posts_list_handler)
        .service(create_post_handler)
        .app_data(Data::new(AppState::init()));

    conf.service(scope);
}

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(&GenericResponse {
        status: "success".to_string(),
        message: "that's stupid".to_string(),
    })
}

#[get("/posts")]
pub async fn posts_list_handler(
    opts: web::Query<QueryOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let posts = data.post_db.lock().unwrap();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let posts: Vec<Post> = posts.clone().into_iter().skip(offset).take(limit).collect();

    let json_response = PostListResponse {
        results: posts.len(),
        posts,
    };

    HttpResponse::Ok().json(json_response)
}

#[post("/posts")]
pub async fn create_post_handler(
    mut body: web::Json<Post>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut posts = data.post_db.lock().unwrap();

    let uuid_id = Uuid::new_v4();

    body.id = Some(uuid_id.to_string());
    body.created_at = Some(Utc::now().timestamp());

    match Url::parse(body.image.as_str()) {
        Ok(url) => url,
        Err(_) => {
            return HttpResponse::BadRequest().json(&CreatePostResponse {
                status: "error".to_string(),
            })
        }
    };

    posts.push(body.into_inner());

    HttpResponse::Ok().json(&CreatePostResponse {
        status: "success".to_string(),
    })
}
