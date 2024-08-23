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

function toggle() {
	let hidden = $("navbar").getElementsByClassName("hidden");
	let shown = $("navbar").getElementsByClassName("shown");
	if (hidden.length > 0) {
		let len = hidden.length;
		for (let i = 0; i < len; i++) {
			hidden[0].classList.replace("hidden", "shown");
		}
	} else if (shown.length > 0) {
		let len = shown.length;
		for (let i = 0; i < len; i++) {
			shown[0].classList.replace("shown", "hidden");
		}
	}
}
window.onload = function() {
	$("logo").src = prefix + 'static/logo.png';
	$("logobox").onclick = toggle;
}
