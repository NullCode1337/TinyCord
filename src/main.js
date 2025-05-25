const { invoke } = window.__TAURI__.core;
const { Window } = window.__TAURI__.window;
const { WebviewWindow } = window.__TAURI__.webviewWindow;

const response = await fetch("browser.js");
const vencordCode = await response.text();

const webview = new WebviewWindow("discordMain", {
  url: "https://discord.com/channels/@me",
  width: 1280,
  height: 720,
  title: "TinyCord"
});

webview.once('tauri://created', async () => {
  try {
    await invoke("greet", {script: vencordCode});
  } catch (e) {
    console.log(
      "Injection failed:", e
    );
  }
});

// webview.once('tauri://destroyed', async () => {
//   try {
//     await invoke("exit_app");
//   } catch (e) {
//     console.error('Failed to exit:', e);
//   }
// });