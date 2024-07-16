function setMsg(msg) {
	var msg_elem = document.getElementById("msg");
	msg_elem.style.color = "white";
	msg_elem.innerHTML = msg;
}

function setErrorMsg(msg) {
	var error_msg = document.getElementById("msg");
	error_msg.style.color = "red";
	error_msg.innerHTML = msg;
}

async function submit(form) {
	var formData = Object.fromEntries(new FormData(form));
	if (formData.totp_as_qr == "on") {
		formData.totp_as_qr = true;
	} else {
		formData.totp_as_qr = false;	
	}
	let response = await fetch(prefix + 'api/auth/register', {
		method: "POST",
		mode: "same-origin",
		cache: "no-cache",
		credentials: "same-origin",
		headers: {
			"Content-Type": "application/json",
		},
		redirect: "follow",
		referrerPolicy: "no-referrer",
		body: JSON.stringify(formData)
	});

	if (response.status !== 200) {
		let errInfo = await response.json();
		console.log(errInfo);
		if (errInfo.error == "AuthError") {
			setErrorMsg("Authentication Error:<br>" + errInfo.msg);
		} else if (errInfo.error == "TokenError") {
			setErrorMsg("Token Error:<br>" + errInfo.msg);
		} else {
			setErrorMsg("Unknown error... check logs if this persists");
		}
	} else {
		let resp = await response.json();
		console.log(resp);
		document.getElementById('form').setAttribute("hidden");
		document.getElementById('totp').removeAttribute("hidden");
		if (formData.totp_as_qr) {
			let img = document.getElementById('totp-qr');
			img.setAttribute("src", resp.totp_qr);
			img.removeAttribute("hidden");
		} else {
			document.getElementById('totp-url').innerHTML = resp.totp_url;
		}
	}
}

window.onload = function() {
	document.getElementById('register').onsubmit = function(e) {
		e.preventDefault();
		try {
			setMsg("Registering...");
			submit(register);
		} catch (error) {
			setErrorMsg('A JS error occurred, check logs for more info and open an issue if this persists');
			console.log(error);
		}
		return false;
	};
	document.getElementById('continue').onclick = function(e) {
		window.location.reload();
	}
}
