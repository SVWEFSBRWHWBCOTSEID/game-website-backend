use actix_web::{web, HttpRequest, HttpResponse, get};

use crate::common::WebErr;
use crate::helpers::general::get_user_with_relations;
use crate::prisma::PrismaClient;


// route for getting user info
#[get("/api/user/{username}")]
pub async fn get_user(
    req: HttpRequest,
    client: web::Data<PrismaClient>,
) -> Result<HttpResponse, WebErr> {

    let username: String = req.match_info().get("username").unwrap().parse().unwrap();
    let mut user = get_user_with_relations(&client, &username).await?;
    user.update_perfs(&client).await?;

    Ok(HttpResponse::Ok().json(user.to_user_res()?))
}
