const { invoke } = window.__TAURI__.core;
const { WebviewWindow } = window.__TAURI__.webviewWindow;

const webview = new WebviewWindow("discordMain", {
  url: "https://discord.com/channels/@me",
  width: 1280,
  height: 720,
  title: "TinyCord"
});

webview.once('tauri://created', async () => {
  try {
    await invoke("inject_equicord");
  } catch (e) {
    console.log(
      "Injection failed:", e
    );
  }
});

webview.once('tauri://destroyed', async () => {
  try {
    await invoke("exit_app");
  } catch (e) {
    console.log('Failed to exit:', e);
  }
});