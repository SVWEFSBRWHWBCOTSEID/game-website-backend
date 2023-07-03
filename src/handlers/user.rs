use actix_web::{web, HttpRequest, HttpResponse, get, post};

use crate::CustomError;
use crate::prisma::{PrismaClient, user};
use crate::models::req::{CreateUserReq, LoginReq};


// route for creating a new user
#[post("/api/user/new")]
pub async fn create_user(
    client: web::Data<PrismaClient>,
    data: web::Json<CreateUserReq>
) -> Result<HttpResponse, CustomError> {

    let create_user_req: CreateUserReq = data.into_inner();
    if !create_user_req.validate(&client).await {
        return Err(CustomError::BadRequest);
    }

    let user = create_user_req.create_user(&client).await;
    Ok(HttpResponse::Ok().json(user.to_user_res()))
}

// route for getting user info
#[get("/api/user/{username}")]
pub async fn get_user(
    req: HttpRequest,
    client: web::Data<PrismaClient>,
) -> Result<HttpResponse, CustomError> {

    let username: String = req.match_info().get("username").unwrap().parse().unwrap();

    Ok(HttpResponse::Ok().json(
        match client
            .user()
            .find_unique(user::name::equals(username))
            .exec()
            .await
            .unwrap()
        {
            Some(u) => Some(u.to_user_res()),
            None => None,
        }
    ))
}

// route for logging in user
#[post("api/login")]
pub async fn login(
    client: web::Data<PrismaClient>,
    data: web::Json<LoginReq>,
) -> Result<HttpResponse, CustomError> {

    let login_req: LoginReq = data.into_inner();

    let user = match client
        .user()
        .find_unique(user::name::equals(login_req.name))
        .exec()
        .await
        .unwrap()
    {
        Some(u) => u,
        None => return Err(CustomError::BadRequest),
    };

    if !bcrypt::verify(&login_req.password, &user.password).unwrap() {
        return Err(CustomError::BadRequest);
    }

    Ok(HttpResponse::Ok().json(user.to_user_res()))
}
