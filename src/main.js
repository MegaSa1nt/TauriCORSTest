let greetInputEl;
let greetMsgEl;

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
});

async function greet() {
	fetch('https://gcs.icu/songs/tauri.php').then(response => response.arrayBuffer()).then((chunk) => {
		console.log(chunk);
		greetMsgEl.textContent = window.__TAURI__.core.invoke("greet", chunk, {
				headers: {
					path: 'test.mp3'
				}
			}).catch(e => {
			return false;
		});
	});
}

window.greet = greet;