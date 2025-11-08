fn main() {
    // Embed icon resource for .exe file icon (Windows Explorer, taskbar, etc.)
    embed_resource::compile("assets/tray.rc", embed_resource::NONE);
}
