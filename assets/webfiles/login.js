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
	$('btn').disabled = true;
	let response = await fetch(prefix + 'api/auth/login', {
		method: 'POST',
		mode: 'same-origin',
		cache: 'no-cache',
		credentials: 'same-origin',
		headers: {
			'Content-Type': 'application/json',
		},
		redirect: 'follow',
		referrerPolicy: 'no-referrer',
		body: JSON.stringify(Object.fromEntries(new FormData($('login')))),
	});
	if (response.status !== 200) {
		let errInfo = await response.json();
		console.log(errInfo);
		if (errInfo.error == 'AuthError') {
			setErrorMsg(errInfo.msg);
		} else {
			setErrorMsg('Unknown error... check logs if this persists');
		}
	} else {
		window.location.reload();
	}
	$('btn').disabled = false;
}

window.onload = function() {
	$('login').onsubmit = function(e) {
		e.preventDefault();
		try {
			setMsg('Logging in...');
			submit();
		} catch (error) {
			setErrorMsg('A JS error occurred, check logs for more info and open an issue if this persists');
			console.log(error);
		}
		return false;
	};
}

