// SPDX-License-Identifier: AGPL-3.0-or-later

use assets_include::include;

fn main() {
    include("assets", vec!["ico", "256.png"], vec!["global.js"]);
}
