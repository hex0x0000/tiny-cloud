// SPDX-License-Identifier: AGPL-3.0-or-later

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
		alert(`Error: Failed to ${type} :(<br>` + errInfo.msg);
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
        alert("Error: Failed to retrieve TOTP:<br>" + errInfo.msg);
    } else {
        let resp = await response.json();

		if (form.totp_as_qr) {
			totpqr.src = 'data:image/png;base64, ' + resp.totp_qr;
			$('totp-res').hidden = false;
		} else {
			totpurl.innerHTML += resp.totp_url;
			$('totp-res').hidden = false;
		}
        alert('TOTP successfully changed. Save it before reloading.');
	}
}

async function changepwd() {
	let form = Object.fromEntries(new FormData($('changepwd')));
	if (form.new_password != form.newpasswd_rep) {
		alert('Error: Passwords do not match.');
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
		alert(`Error: Failed to ${type} :(<br>` + errInfo.msg);
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
			changepwd();
		} catch (error) {
			console.log(error);
			alert('An error occurred, check logs for more info and open an issue if this persists');
		}
		return false;
	};
    $('totp').onsubmit = function(e) {
        e.preventDefault();
        try {
            totp();
        } catch (error) {
            console.log(error);
			alert('An error occurred, check logs for more info and open an issue if this persists');
        }
        return false;
    };
}

