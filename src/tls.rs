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

use crate::config::Tls;
#[cfg(feature = "openssl")]
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
#[cfg(feature = "rustls")]
use rustls::{pki_types::CertificateDer, ServerConfig};
#[cfg(feature = "rustls")]
use rustls_pemfile::{certs, private_key};
#[cfg(feature = "rustls")]
use std::{
    fs::File,
    io::{BufReader, Error},
};

mutually_exclusive_features::exactly_one_of!("openssl", "rustls");

#[cfg(feature = "openssl")]
pub fn get_openssl_config(tls: &Tls) -> Result<SslAcceptorBuilder, String> {
    let mut builder =
        SslAcceptor::mozilla_intermediate(SslMethod::tls()).map_err(|e| format!("Failed to start openssl acceptor: {e}"))?;
    builder
        .set_private_key_file(&tls.privkey_path, SslFiletype::PEM)
        .map_err(|e| format!("Failed to get private key file: {e}"))?;
    builder
        .set_certificate_chain_file(&tls.cert_path)
        .map_err(|e| format!("Failed to get certificate file: {e}"))?;
    Ok(builder)
}

#[cfg(feature = "rustls")]
pub fn get_rustls_config(tls: &Tls) -> Result<ServerConfig, String> {
    // init server config builder
    let config = ServerConfig::builder().with_no_client_auth();

    // load TLS key/cert files
    let cert_file = &mut BufReader::new(
        File::open(&tls.cert_path).map_err(|e| format!("Failed to open certificate file at {}: {e}", tls.cert_path))?,
    );
    let key_file = &mut BufReader::new(
        File::open(&tls.privkey_path).map_err(|e| format!("Failed to open private key file at {}: {e}", tls.privkey_path))?,
    );

    // convert files to key/cert objects
    let cert_chain = certs(cert_file)
        .collect::<Result<Vec<CertificateDer<'_>>, Error>>()
        .map_err(|e| format!("Failed to read certificate file: {e}"))?;
    let key_der = private_key(key_file)
        .map_err(|e| format!("Failed to read private key: {e}"))?
        .ok_or("No private key found".to_string())?;

    Ok(config
        .with_single_cert(cert_chain, key_der)
        .map_err(|e| format!("Failed to parse certificate and key: {e}"))?)
}
