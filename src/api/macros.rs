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

#[macro_export]
macro_rules! get_user {
    ($id:expr) => {{
        match $id {
            Ok(user) => user,
            Err(err) => match err {
                GetIdentityError::SessionExpiryError(_) => {
                    return HttpResponse::Forbidden().body("The session has expired, login again")
                }
                GetIdentityError::MissingIdentityError(_) => return HttpResponse::Forbidden().body("Invalid session, login again"),
                _ => {
                    log::error!("An error occurred while getting username from identity: {}", err);
                    return HttpResponse::InternalServerError().body("An internal server error occurred while authenticating");
                }
            },
        }
    }};
}
