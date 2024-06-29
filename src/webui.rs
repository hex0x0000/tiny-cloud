mod home;
pub mod images;
mod login;
mod register;
#[macro_use]
mod macros;
use crate::{config, utils};
use actix_identity::Identity;
use actix_web::{get, web::Redirect, HttpRequest, HttpResponse, Responder};

#[get("")]
pub async fn root(req: HttpRequest, user: Option<Identity>) -> impl Responder {
    if let Some(user) = user {
        match user.id() {
            Ok(username) => HttpResponse::Ok().body(home::page(username)),
            Err(e) => utils::id_err_into(e),
        }
    } else {
        Redirect::to(utils::make_url("/ui/login"))
            .see_other()
            .respond_to(&req)
            .map_into_boxed_body()
    }
}

#[get("/register")]
pub async fn register_page(req: HttpRequest, user: Option<Identity>) -> impl Responder {
    if let Some(_) = *config!(registration) {
        if let None = user {
            HttpResponse::Ok().body(register::page())
        } else {
            Redirect::to(utils::make_url("/ui"))
                .see_other()
                .respond_to(&req)
                .map_into_boxed_body()
        }
    } else {
        HttpResponse::NotFound().body("")
    }
}

#[get("/login")]
pub async fn login_page(req: HttpRequest, user: Option<Identity>) -> impl Responder {
    if let None = user {
        HttpResponse::Ok().body(login::page())
    } else {
        Redirect::to(utils::make_url("/ui"))
            .see_other()
            .respond_to(&req)
            .map_into_boxed_body()
    }
}
