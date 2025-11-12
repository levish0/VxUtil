mod app;
mod ui;

use app::VxUtil;

fn main() -> iced::Result {
    // Force Vulkan backend for consistency across all platforms
    unsafe {
        std::env::set_var("WGPU_BACKEND", "vulkan");
    }

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("vxutil=debug,info")
        .init();

    tracing::info!("Starting VxUtil video editor");

    iced::application(VxUtil::title, VxUtil::update, VxUtil::view)
        .theme(VxUtil::theme)
        .run_with(VxUtil::new)
}