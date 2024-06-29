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
	var formData = new FormData(form);
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
		body: JSON.stringify(Object.fromEntries(formData)),
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
		window.location.reload();
	}
}

window.onload = function() {
	var register = document.getElementById('register');
	register.onsubmit = function(event) {
		event.preventDefault();
		try {
			setMsg("Registering...");
			submit(register);
		} catch (error) {
			setErrorMsg('A JS error occurred, check logs for more info and open an issue if this persists');
			console.log(error);
		}
		return false;
	};
}
