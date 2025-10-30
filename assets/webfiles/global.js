// SPDX-License-Identifier: AGPL-3.0-or-later

"use strict";
const $ = function(e) { return document.getElementById(e); };
try {
	let p = document.querySelector('meta[name="tcloud-prefix"]').content;
	var prefix = (p == '') ? '/' : '/' + p + '/';
} catch (e) {
	var prefix = '/';
}
