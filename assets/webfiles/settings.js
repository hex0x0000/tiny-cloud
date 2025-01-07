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

function setMsg(msg) {
	$('msg').style.color = 'white';
	$('msg').innerHTML = msg;
}

function setErrorMsg(msg) {
	$('msg').style.color = 'red';
	$('msg').innerHTML = msg;
}

async function send(type) {
	let response = await fetch(prefix + `api/auth/${type}`, {
		method: 'GET',
		mode: 'same-origin',
		cache: 'no-cache',
		credentials: 'same-origin',
		redirect: 'follow',
		referrerPolicy: 'no-referrer',
	});
	if (response.status !== 200) {
		let errInfo = await response.json();
		console.log(errInfo);
		setErrorMsg(`Failed to ${type} :(<br>` + errInfo.msg);
	} else {
		window.location.reload();
	}
}

window.onload = function() {
	navbar_onload();
	$('logout').onclick = function(e) {
		if (confirm('Are you sure you want to logout?')) {
			send('logout');
		}
	};
	$('delete').onclick = function(e) {
		if (confirm("If you delete your account all your files will be deleted. Are you sure you want to continue?")) {
			if (confirm("Are you totally sure?? You won't be able to undo your choice if you click OK.")) {
				send('delete');
			}
		}
	};
}

