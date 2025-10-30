// This file is part of the Tiny Cloud project.
// You can find the source code of every repository here:
//		https://github.com/personal-tiny-cloud
//
// Copyright (C) 2024  hex0x0000
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNu Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//
// Email: hex0x0000@protonmail.com

pub mod error;
mod home;
pub mod images;
mod login;
mod register;
mod settings;
#[macro_use]
mod macros;
use crate::{auth::validate_user, config, utils};
use actix_identity::Identity;
use actix_web::{
    HttpRequest, HttpResponse, Responder, get,
    web::{self, Redirect},
};
use async_sqlite::Pool;

#[get("")]
pub async fn root(req: HttpRequest, pool: web::Data<Pool>, user: Option<Identity>) -> impl Responder {
    let pool = pool.into_inner();
    if let Some(user) = user {
        match validate_user(&pool, user).await {
            Ok((username, is_admin)) => HttpResponse::Ok().body(home::page(username, is_admin).await),
            Err(e) => error::to_response_page(e),
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
    if config!(registration).is_some() {
        if user.is_none() {
            HttpResponse::Ok().body(*register::PAGE)
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
    if user.is_none() {
        HttpResponse::Ok().body(*login::PAGE)
    } else {
        Redirect::to(utils::make_url("/ui"))
            .see_other()
            .respond_to(&req)
            .map_into_boxed_body()
    }
}

#[get("/settings")]
pub async fn settings_page(req: HttpRequest, pool: web::Data<Pool>, user: Option<Identity>) -> impl Responder {
    let pool = pool.into_inner();
    if let Some(user) = user {
        match validate_user(&pool, user).await {
            Ok((_, is_admin)) => HttpResponse::Ok().body(settings::page(is_admin)),
            Err(e) => error::to_response_page(e),
        }
    } else {
        Redirect::to(utils::make_url("/ui/login"))
            .see_other()
            .respond_to(&req)
            .map_into_boxed_body()
    }
}
