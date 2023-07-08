use actix_session::Session;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::{web, HttpRequest, HttpResponse, get, post};

use crate::common::CustomError;
use crate::prisma::{PrismaClient, user};
use crate::models::req::{CreateUserReq, LoginReq};


// route for creating a new user
#[post("/api/user/new")]
pub async fn create_user(
    client: web::Data<PrismaClient>,
    session: Session,
    data: web::Json<CreateUserReq>
) -> Result<HttpResponse, CustomError> {

    let create_user_req: CreateUserReq = data.into_inner();
    if !create_user_req.validate(&client).await {
        return Err(CustomError::BadRequest);
    }

    let user = create_user_req.create_user(&client).await;

    match session.insert("username", &user.username) {
        Err(_) => return Err(CustomError::InternalError),
        _ => {},
    }

    let mut cookie = Cookie::new("username", &user.username);
    cookie.set_same_site(SameSite::None);
    cookie.set_path("/");

    let mut res = HttpResponse::Ok().json(user.to_create_user_res());
    res.add_cookie(&cookie).unwrap();
    Ok(res)
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
            .find_unique(user::username::equals(username))
            .exec()
            .await
            .unwrap()
        {
            Some(u) => Some(u.to_create_user_res()),
            None => None,
        }
    ))
}

// route for logging in user
#[post("api/login")]
pub async fn login(
    client: web::Data<PrismaClient>,
    session: Session,
    data: web::Json<LoginReq>,
) -> Result<HttpResponse, CustomError> {

    let login_req: LoginReq = data.into_inner();

    let user = match client
        .user()
        .find_unique(user::username::equals(login_req.username))
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

    session.renew();
    match session.insert("username", &user.username) {
        Err(_) => return Err(CustomError::InternalError),
        _ => {},
    }

    let mut cookie = Cookie::new("username", &user.username);
    cookie.set_same_site(SameSite::None);
    cookie.set_path("/");

    let mut res = HttpResponse::Ok().json(user.to_create_user_res());
    res.add_cookie(&cookie).unwrap();
    Ok(res)
}

// route for logging in user
#[post("api/logout")]
pub async fn logout(req: HttpRequest, session: Session) -> Result<HttpResponse, CustomError> {
    session.purge();
    let mut res = HttpResponse::Ok().finish();

    match req.cookie("username") {
        Some(mut cookie) => {
            cookie.make_removal();
            cookie.set_same_site(SameSite::None);
            cookie.set_path("/");
            res.add_cookie(&cookie).expect("failed to remove username cookie");
        },
        None => {},
    }

    Ok(res)
}
