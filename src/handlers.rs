use actix_web::Responder;

pub async fn get_users() -> impl Responder {
    format!("hello from get users")
}

pub async fn get_user_by_id() -> impl Responder {
    format!("hello from get users by id")
}

pub async fn add_user() -> impl Responder {
    format!("hello from add user")
}

pub async fn delete_user() -> impl Responder {
    format!("hello from delete user")
}

pub async fn get_articles() -> impl Responder {
    format!("hello from get articles")
}

// pub async fn save_articles

// pub async fn delete_article