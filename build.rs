fn main() {
    // Embed the application icon and version metadata into the Windows executable
    #[cfg(windows)]
    {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/netlens.ico");
        res.set("ProductName", "NetLens");
        res.set("FileDescription", "NetLens - Local Network Device Explorer");
        if let Err(e) = res.compile() {
            // Never fail the build because of resource compilation (e.g. rc.exe not installed)
            println!("cargo:warning=Could not embed Windows icon resource: {}", e);
        }
    }
}
