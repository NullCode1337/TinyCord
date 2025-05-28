# TinyCord
Discord, but tiny and powered by Rust (Tauri)

# STILL EXPERIMENTAL
Current status: WebApp loads, Equicord plugins work, not themes - stylesheet is broken

![image](https://github.com/user-attachments/assets/bca2f7cd-9609-428e-a9c2-9e0651c53508)

# Why?
- We hate electron around here
- 99.99999% file size reduction (source: i made it up)

![image](https://github.com/user-attachments/assets/255abd07-23ae-478a-9e81-e6ac268b8a0b)

# Roadmap
- Embed discord ✅
- Screenshare ✅
- File upload ✅
- Inject equicord ✅
- Download equicord from remote location ✅
- Make equicord persist on refresh ✅
- Drag and drop ✅
- Cross-platform ✅ (macOS will never be added because I don't have a device)
- Remember last loaded webpage via localstorage
- Figure out notifications
- Integrate arRPC server inside wails golang
- Tray icon
- CSP stuff affecting online themes and stylesheets (Allow foreign assets to be loaded)
- Use currently hidden main window act as Equibop updater, showing the user progress and any timeout errors (because I've noticed timeout errors are happening maybe cuz im being rate limited haha i wonder why)