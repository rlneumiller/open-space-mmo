//! Example showing how to get the system's maximum window size / desktop size in winit @v0.30.11
//!
//! This example demonstrates different ways to determine the maximum window size:
//! - Primary monitor size
//! - Largest monitor size  
//! - Total desktop area (multi-monitor setups)
//! - Physical vs logical sizes (DPI-aware)

use winit::application::ApplicationHandler;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

#[derive(Default)]
struct DesktopSizeApp {
    window: Option<Window>,
}

impl ApplicationHandler for DesktopSizeApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Print desktop/monitor size information
        print_desktop_size_info(event_loop);
        
        // Create a window using the information we gathered
        let window_attributes = Window::default_attributes()
            .with_title("Desktop Size Example")
            .with_inner_size(winit::dpi::LogicalSize::new(800, 600));
            
        let window = event_loop.create_window(window_attributes).unwrap();
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // Handle redraw if needed
            }
            _ => {}
        }
    }
}

fn print_desktop_size_info(event_loop: &ActiveEventLoop) {
    println!("=== Desktop Size Information ===\n");
    
    // 1. Get all available monitors
    let monitors: Vec<_> = event_loop.available_monitors().collect();
    
    if monitors.is_empty() {
        println!("Warning: No monitors detected!");
        return;
    }
    
    println!("Found {} monitor(s):", monitors.len());
    
    // 2. Primary monitor information
    if let Some(primary_monitor) = event_loop.primary_monitor() {
        let size = primary_monitor.size();
        let scale_factor = primary_monitor.scale_factor();
        let logical_size = size.to_logical::<f64>(scale_factor);
        
        println!("\n--- Primary Monitor ---");
        println!("Name: {}", primary_monitor.name().unwrap_or_else(|| "Unknown".to_string()));
        println!("Physical Size: {}x{} pixels", size.width, size.height);
        println!("Logical Size: {:.0}x{:.0} (for window sizing)", logical_size.width, logical_size.height);
        println!("Scale Factor: {:.2}", scale_factor);
        
        if let Some(refresh_rate) = primary_monitor.refresh_rate_millihertz() {
            println!("Refresh Rate: {:.1} Hz", refresh_rate as f64 / 1000.0);
        }
    }
    
    // 3. Information about all monitors
    println!("\n--- All Monitors ---");
    for (i, monitor) in monitors.iter().enumerate() {
        let name = monitor.name().unwrap_or_else(|| format!("Monitor {}", i + 1));
        let PhysicalSize { width, height } = monitor.size();
        let PhysicalPosition { x, y } = monitor.position();
        let scale_factor = monitor.scale_factor();
        
        println!("Monitor {}: {} ({}x{} at {},{}, scale: {:.2})", 
                 i + 1, name, width, height, x, y, scale_factor);
    }
    
    // 4. Find the largest monitor (useful for max window size)
    if let Some(largest_monitor) = monitors.iter().max_by_key(|m| {
        let size = m.size();
        size.width * size.height
    }) {
        let size = largest_monitor.size();
        let scale_factor = largest_monitor.scale_factor();
        let logical_size = size.to_logical::<f64>(scale_factor);
        let name = largest_monitor.name().unwrap_or_else(|| "Unknown".to_string());
        
        println!("\n--- Largest Monitor (Max Window Size) ---");
        println!("Monitor: {}", name);
        println!("Physical Size: {}x{} pixels", size.width, size.height);
        println!("Logical Size: {:.0}x{:.0}", logical_size.width, logical_size.height);
        println!("Use this as your maximum window size!");
    }
    
    // 5. Calculate total desktop area (multi-monitor setups)
    if monitors.len() > 1 {
        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;
        
        for monitor in &monitors {
            let PhysicalPosition { x, y } = monitor.position();
            let PhysicalSize { width, height } = monitor.size();
            
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x + width as i32);
            max_y = max_y.max(y + height as i32);
        }
        
        let total_width = (max_x - min_x) as u32;
        let total_height = (max_y - min_y) as u32;
        
        println!("\n--- Total Desktop Area ---");
        println!("Combined Size: {}x{} pixels", total_width, total_height);
        println!("Bounds: ({}, {}) to ({}, {})", min_x, min_y, max_x, max_y);
        println!("This represents the entire desktop across all monitors");
    }
    
    // 6. Practical usage examples
    println!("\n--- Practical Usage Examples ---");
    
    // Example 1: Create a window that fills 80% of the largest monitor
    if let Some(largest_monitor) = monitors.iter().max_by_key(|m| {
        let size = m.size();
        size.width * size.height
    }) {
        let size = largest_monitor.size();
        let scale_factor = largest_monitor.scale_factor();
        let window_width = (size.width as f64 * 0.8 / scale_factor) as u32;
        let window_height = (size.height as f64 * 0.8 / scale_factor) as u32;
        
        println!("Example: 80% of largest monitor = {}x{} logical pixels", window_width, window_height);
        println!("  window.request_inner_size(LogicalSize::new({}, {}));", window_width, window_height);
    }
    
    // Example 2: Get maximum size for fullscreen borderless
    if let Some(primary) = event_loop.primary_monitor() {
        let size = primary.size();
        println!("Example: Fullscreen size = {}x{} physical pixels", size.width, size.height);
        println!("  window.set_fullscreen(Some(Fullscreen::Borderless(None)));");
    }
    
    println!("\n=== End Desktop Size Information ===\n");
}

/// Simple helper function to get the largest monitor size
pub fn get_max_window_size(event_loop: &ActiveEventLoop) -> Option<PhysicalSize<u32>> {
    event_loop.available_monitors()
        .max_by_key(|monitor| {
            let size = monitor.size();
            size.width * size.height
        })
        .map(|monitor| monitor.size())
}

/// Helper function to get the primary monitor size
pub fn get_primary_monitor_size(event_loop: &ActiveEventLoop) -> Option<PhysicalSize<u32>> {
    event_loop.primary_monitor().map(|monitor| monitor.size())
}

/// Helper function to get desktop bounds (useful for multi-monitor setups)
pub fn get_desktop_bounds(event_loop: &ActiveEventLoop) -> Option<(i32, i32, u32, u32)> {
    let monitors: Vec<_> = event_loop.available_monitors().collect();
    
    if monitors.is_empty() {
        return None;
    }
    
    if monitors.len() == 1 {
        let monitor = &monitors[0];
        let pos = monitor.position();
        let size = monitor.size();
        return Some((pos.x, pos.y, size.width, size.height));
    }
    
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;
    
    for monitor in &monitors {
        let PhysicalPosition { x, y } = monitor.position();
        let PhysicalSize { width, height } = monitor.size();
        
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x + width as i32);
        max_y = max_y.max(y + height as i32);
    }
    
    Some((min_x, min_y, (max_x - min_x) as u32, (max_y - min_y) as u32))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;
    let mut app = DesktopSizeApp::default();
    
    println!("Desktop Size Example");
    println!("This will show information about your monitors and desktop size.");
    println!("Close the window to exit.\n");
    
    event_loop.run_app(&mut app)?;
    
    Ok(())
}
