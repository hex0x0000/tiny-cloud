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

async function get(type) {
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

async function totp() {
    let totpqr = $('totp-qr');
    let totpurl = $('totp-url');
    totpqr.src = '';
    totpurl.innerHTML = '';

	var form = Object.fromEntries(new FormData($('totp')));

    form.password = form.tpasswd;
    delete form.tpasswd;

    if (form.totp_as_qr === "on") {
        form.totp_as_qr = true;
    } else {
        form.totp_as_qr = false;
    }

	let response = await fetch(prefix + 'api/auth/changetotp', {
		method: 'POST',
		mode: 'same-origin',
		cache: 'no-cache',
		credentials: 'same-origin',
		redirect: 'follow',
		referrerPolicy: 'no-referrer',
		headers: {
			'Content-Type': 'application/json',
		},
		body: JSON.stringify(form),
	});

    if (response.status !== 200) {
        let errInfo = await response.json();
        console.log(errInfo);
        setErrorMessage("Failed to retrieve TOTP:<br>" + errInfo.msg);
    } else {
        let resp = await response.json();

		if (form.totp_as_qr) {
			totpqr.src = 'data:image/png;base64, ' + resp.totp_qr;
			$('totp-res').hidden = false;
		} else {
			totpurl.innerHTML += resp.totp_url;
			$('totp-res').hidden = false;
		}
        setMsg('TOTP successfully changed. Save it before reloading.');
	}
}

async function changepwd() {
	let form = Object.fromEntries(new FormData($('changepwd')));
	if (form.new_password != form.newpasswd_rep) {
		setErrorMsg('Passwords do not match.');
		return;
	}
	delete form.newpasswd_rep;
	if (form.istoken == "on") {
		form.token = form.oldpassword;
		delete form.oldpassword;
	}
	delete form.istoken;

	let response = await fetch(prefix + 'api/auth/changepwd', {
		method: 'POST',
		mode: 'same-origin',
		cache: 'no-cache',
		credentials: 'same-origin',
		redirect: 'follow',
		referrerPolicy: 'no-referrer',
		headers: {
			'Content-Type': 'application/json',
		},
		body: JSON.stringify(form),
	});

	if (response.status !== 200) {
		let errInfo = await response.json();
		console.log(errInfo);
		setErrorMsg(`Failed to ${type} :(<br>` + errInfo.msg);
	} else {
        alert('Password changed. Logging out...');
        window.location.reload();
    }
}

window.onload = function() {
	navbar_onload();
	$('logout').onclick = function(e) {
		if (confirm('Are you sure you want to logout?')) {
			get('logout');
		}
	};
	$('delete').onclick = function(e) {
		if (confirm("If you delete your account all your files will be deleted. Are you sure you want to continue?")) {
			if (confirm("Are you totally sure?? You won't be able to undo your choice if you click OK.")) {
				get('delete');
			}
		}
	};
	$('session').onclick = function(e) {
		if (confirm("Are you sure you want to log out from all your sessions?")) {
			get('logoutall');
		}
	};
    $('totp-btn').onclick = function(e) {
        if (confirm("You'll be logged out and you won't be able to see your TOTP secret anymore. Are you sure you want to reload?")) {
            window.location.reload();
        }
    };
	$('changepwd').onsubmit = function(e) {
		e.preventDefault();
		try {
			setMsg('Changing password...');
			changepwd();
		} catch (error) {
			setErrorMsg('An error occurred, check logs for more info and open an issue if this persists');
			console.log(error);
		}
		return false;
	};
    $('totp').onsubmit = function(e) {
        e.preventDefault();
        try {
            setMsg('Changing TOTP...');
            totp();
        } catch (error) {
			setErrorMsg('An error occurred, check logs for more info and open an issue if this persists');
            console.log(error);
        }
        return false;
    };
}

