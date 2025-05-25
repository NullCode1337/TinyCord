const { Window } = window.__TAURI__.window;
const { WebviewWindow } = window.__TAURI__.webviewWindow;

const webview = new WebviewWindow("discordMain", {
  url: 'https://discord.com/channels/@me',
  width: 1280,
  height: 720,
  title: "TinyCord"
});

