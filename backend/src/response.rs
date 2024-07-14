use serde::Serialize;

use crate::model::Post;

#[derive(Serialize, Debug)]
pub struct PostListResponse {
    pub results: usize,
    pub posts: Vec<Post>,
}
