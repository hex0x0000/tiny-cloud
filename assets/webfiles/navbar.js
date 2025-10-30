// SPDX-License-Identifier: AGPL-3.0-or-later

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
function navbar_onload() {
	$("logo").src = prefix + 'static/logo.png';
	$("logobox").onclick = toggle;
}
