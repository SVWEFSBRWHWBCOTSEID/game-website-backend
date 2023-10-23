use std::io::Read;
use actix_multipart::form::MultipartForm;
use actix_session::Session;
use actix_web::{post, HttpResponse};
use actix_web::web::{Data};
use aws_sdk_s3::primitives::ByteStream;

use crate::common::WebErr;
use crate::helpers::general::{get_username, get_user_with_relations};
use crate::helpers::user::get_user_res;
use crate::models::req::ProfileReq;
use crate::prisma::{PrismaClient, user};


// route for updating a user's profile
#[post("/api/profile/update")]
pub async fn update_profile(
    client: Data<PrismaClient>,
    aws_client: Data<aws_sdk_s3::Client>,
    session: Session,
    MultipartForm(data): MultipartForm<ProfileReq>,
) -> Result<HttpResponse, WebErr> {

    let username: String = get_username(&session)?;
    let mut update_params = vec![
        user::country::set(data.country.to_string()),
        user::location::set(data.location.into_inner()),
        user::bio::set(data.bio.into_inner()),
        user::first_name::set(data.first_name.into_inner()),
        user::last_name::set(data.last_name.into_inner()),
    ];

    // Upload pfp to S3 if existent
    if let Some(mut f) = data.pfp {
        let mut bytes = Vec::new();
        f.file.read_to_end(&mut bytes)?;

        let file_name = username.clone() + "." + f.content_type.unwrap().subtype().into();

        aws_client
            .put_object()
            .bucket("gulpin-pfps")
            .key(file_name.clone())
            .body(ByteStream::from(bytes))
            .send()
            .await?;

        update_params.push(user::image_url::set(
            Some(format!("https://gulpin-pfps.s3.us-east-2.amazonaws.com/{}", file_name).to_string())
        ))
    }

    client
        .user()
        .update(user::username::equals(username.clone()), update_params)
        .exec()
        .await
        .or(Err(WebErr::Forbidden(format!("could not find user with username {}", username))))?;

    let new_user = get_user_with_relations(&client, &username).await?;
    Ok(HttpResponse::Ok().json(get_user_res(&client, new_user).await?))
}
