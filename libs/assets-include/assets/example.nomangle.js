// SPDX-License-Identifier: AGPL-3.0-or-later

"use strict";
const $ = function(elem) { return document.getElementById(elem); };
try {
	var _prefix = document.querySelector('meta[name="tcloud-prefix"]').content;
	if (_prefix == '') {
		_prefix = '/';
	} else {
		_prefix = '/' + _prefix + '/';
	}
} catch (exception) {
	var _prefix = '/';
}
const prefix = _prefix;
