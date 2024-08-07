// This file is part of the Tiny Cloud project.
// You can find the source code of every repository here:
//		https://github.com/personal-tiny-cloud
//
// Copyright (C) 2024  hex0x0000
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
// 
// Email: hex0x0000@protonmail.com

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
    if config!(registration).is_some() {
        if user.is_none() {
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
    if user.is_none() {
        HttpResponse::Ok().body(login::page())
    } else {
        Redirect::to(utils::make_url("/ui"))
            .see_other()
            .respond_to(&req)
            .map_into_boxed_body()
    }
}
