// SPDX-License-Identifier: AGPL-3.0-or-later

function setMsg(msg) {
	$('msg').style.color = 'white';
	$('msg').innerHTML = msg;
}

function setErrorMsg(msg) {
	$('msg').style.color = 'red';
	$('msg').innerHTML = msg;
}

async function submit() {
	var form = Object.fromEntries(new FormData($('register')));
	if (form.password_rep != form.password) {
		setErrorMsg('Passwords do not match.');
		return;
	}
	delete form.password_rep;
	if (form.totp_as_qr == 'on') {
		form.totp_as_qr = true;
	} else {
		form.totp_as_qr = false;	
	}
	$('btn').disabled = true;
	let response = await fetch(prefix + 'api/auth/register', {
		method: 'POST',
		mode: 'same-origin',
		cache: 'no-cache',
		credentials: 'same-origin',
		headers: {
			'Content-Type': 'application/json',
		},
		redirect: 'follow',
		referrerPolicy: 'no-referrer',
		body: JSON.stringify(form),
	});
	if (response.status !== 200) {
		let errInfo = await response.json();
		console.log(errInfo);
		if (errInfo.error == 'AuthError') {
			setErrorMsg('Authentication Error:<br>' + errInfo.msg);
		} else if (errInfo.error == 'TokenError') {
			setErrorMsg('Token Error:<br>' + errInfo.msg);
		} else {
			setErrorMsg('Unknown error... check logs if this persists');
		}
	} else {
		let resp = await response.json();
		setMsg('');
		$('register').style.display = "none";
		$('totp').hidden = false;
		if (form.totp_as_qr) {
			let img = $('totp-qr');
			img.src = 'data:image/png;base64, ' + resp.totp_qr;
			img.hidden = false;
		} else {
			$('totp-url').innerHTML = resp.totp_url;
		}
	}
	$('btn').disabled = false;
}

window.onload = function() {
	$('register').onsubmit = function(e) {
		e.preventDefault();
		try {
			setMsg('Registering...');
			submit();
		} catch (error) {
			setErrorMsg('An error occurred, check logs for more info and open an issue if this persists');
			console.log(error);
		}
		return false;
	};
	$('continue').onclick = function(e) {
		window.location.reload();
	};
	$('totp_as_qr').checked = true;
}
