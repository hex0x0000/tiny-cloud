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
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//
// Email: hex0x0000@protonmail.com

use actix_web::{HttpResponse, HttpResponseBuilder};
use serde_json::json;

/// Common trait for errors that can be returned to the client.
pub trait ErrToResponse {
    /// Error's name
    fn error(&self) -> &'static str;

    /// Error's type
    fn err_type(&self) -> &'static str;

    /// Error's message
    fn msg(&self) -> String;

    /// Error's http code
    fn http_code(&self) -> HttpResponseBuilder;

    /// Handles special types before building the response
    fn handle(&self);

    /// Turns the error into a response
    fn to_response(&self) -> HttpResponse {
        self.handle();
        self.http_code().content_type("application/json").body(
            json!({
                "error": self.error(),
                "type": self.err_type(),
                "msg": self.msg()
            })
            .to_string(),
        )
    }
}
