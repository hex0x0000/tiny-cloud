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
	let response = await fetch(prefix + 'api/auth/login', {
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
			setErrorMsg(errInfo.msg);
		} else {
			setErrorMsg("Unknown error... check logs if this persists");
		}
	} else {
		window.location.reload();
	}
}

window.onload = function() {
	var login = document.getElementById('login');
	login.onsubmit = function(event) {
		event.preventDefault();
		try {
			setMsg("Logging in...");
			submit(login);
		} catch (error) {
			setErrorMsg('A JS error occurred, check logs for more info and open an issue if this persists');
			console.log(error);
		}
		return false;
	};
}

