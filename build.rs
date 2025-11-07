fn main() {
    // Embed icon resource for .exe file icon (Windows Explorer, taskbar, etc.)
    embed_resource::compile("app.rc", embed_resource::NONE);
}
