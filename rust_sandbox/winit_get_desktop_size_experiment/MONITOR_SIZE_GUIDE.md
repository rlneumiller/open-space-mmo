# Getting System's Maximum Window Size / Desktop Size in Winit

This guide explains how to get monitor and desktop size information in winit, which is essential for determining the maximum window size or desktop dimensions.

## Quick Summary

In winit, you can get monitor/desktop size information through the `ActiveEventLoop`:

```rust
// Get primary monitor size
if let Some(primary) = event_loop.primary_monitor() {
    let size = primary.size(); // PhysicalSize<u32>
    println!("Primary monitor: {}x{}", size.width, size.height);
}

// Get all monitors and find the largest
let largest_monitor = event_loop.available_monitors()
    .max_by_key(|monitor| {
        let size = monitor.size();
        size.width * size.height
    });
```

## Key Methods

### 1. ActiveEventLoop Methods

- `available_monitors()` - Returns iterator over all monitors
- `primary_monitor()` - Returns the primary monitor (if any)

### 2. MonitorHandle Methods

- `size()` - Returns `PhysicalSize<u32>` (actual pixels)
- `position()` - Returns `PhysicalPosition<i32>` (monitor position)
- `scale_factor()` - Returns `f64` (DPI scaling factor)
- `name()` - Returns `Option<String>` (monitor name)
- `refresh_rate_millihertz()` - Returns `Option<u32>` (refresh rate)

### 3. Size Types

- **Physical Size**: Actual pixels on screen (`PhysicalSize<u32>`)
- **Logical Size**: DPI-scaled size for UI (`LogicalSize<f64>`)

```rust
let physical_size = monitor.size(); // e.g., 3840x2160 on 4K monitor
let scale_factor = monitor.scale_factor(); // e.g., 2.0 for high-DPI
let logical_size = physical_size.to_logical::<f64>(scale_factor); // e.g., 1920x1080
```

## Common Use Cases

### 1. Maximum Window Size

```rust
fn get_max_window_size(event_loop: &ActiveEventLoop) -> Option<PhysicalSize<u32>> {
    event_loop.available_monitors()
        .max_by_key(|monitor| {
            let size = monitor.size();
            size.width * size.height
        })
        .map(|monitor| monitor.size())
}
```

### 2. Primary Monitor Size

```rust
fn get_primary_monitor_size(event_loop: &ActiveEventLoop) -> Option<PhysicalSize<u32>> {
    event_loop.primary_monitor().map(|monitor| monitor.size())
}
```

### 3. Total Desktop Area (Multi-Monitor)

```rust
fn get_total_desktop_size(event_loop: &ActiveEventLoop) -> Option<(u32, u32)> {
    let monitors: Vec<_> = event_loop.available_monitors().collect();
    
    if monitors.is_empty() {
        return None;
    }
    
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;
    
    for monitor in &monitors {
        let pos = monitor.position();
        let size = monitor.size();
        
        min_x = min_x.min(pos.x);
        min_y = min_y.min(pos.y);
        max_x = max_x.max(pos.x + size.width as i32);
        max_y = max_y.max(pos.y + size.height as i32);
    }
    
    Some(((max_x - min_x) as u32, (max_y - min_y) as u32))
}
```

### 4. DPI-Aware Window Sizing

```rust
fn create_dpi_aware_window(event_loop: &ActiveEventLoop) -> Result<Window, Box<dyn Error>> {
    if let Some(primary) = event_loop.primary_monitor() {
        let scale_factor = primary.scale_factor();
        
        // Create a logical 800x600 window that scales properly
        let logical_size = LogicalSize::new(800.0, 600.0);
        
        let window_attributes = Window::default_attributes()
            .with_title("DPI-Aware Window")
            .with_inner_size(logical_size);
            
        event_loop.create_window(window_attributes)
    } else {
        // Fallback for no monitors
        let window_attributes = Window::default_attributes()
            .with_title("Fallback Window")
            .with_inner_size(LogicalSize::new(800.0, 600.0));
            
        event_loop.create_window(window_attributes)
    }
}
```

## Complete Example

```rust
use winit::application::ApplicationHandler;
use winit::dpi::{PhysicalPosition, PhysicalSize, LogicalSize};
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

#[derive(Default)]
struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Print monitor information
        print_monitor_info(event_loop);
        
        // Create window
        let window_attributes = Window::default_attributes()
            .with_title("Monitor Info Example");
            
        self.window = Some(event_loop.create_window(window_attributes).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => {}
        }
    }
}

fn print_monitor_info(event_loop: &ActiveEventLoop) {
    println!("=== Monitor Information ===");
    
    // Primary monitor
    if let Some(primary) = event_loop.primary_monitor() {
        let size = primary.size();
        println!("Primary Monitor: {}x{}", size.width, size.height);
    }
    
    // All monitors
    for (i, monitor) in event_loop.available_monitors().enumerate() {
        let name = monitor.name().unwrap_or_else(|| format!("Monitor {}", i + 1));
        let size = monitor.size();
        let pos = monitor.position();
        let scale = monitor.scale_factor();
        
        println!("Monitor {}: {} ({}x{} at {},{}, scale: {:.2})", 
                 i + 1, name, size.width, size.height, pos.x, pos.y, scale);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;
    let mut app = App::default();
    event_loop.run_app(&mut app)?;
    Ok(())
}
```

## Platform-Specific Notes

- **Windows**: Monitor information is accurate and includes multi-monitor setups
- **macOS**: Works well, but coordinates may be in different coordinate systems
- **Linux X11**: Requires XRandR extension for accurate monitor information
- **Linux Wayland**: May have limitations depending on compositor
- **Web**: Monitor information is limited for security reasons
- **Android/iOS**: Typically returns single screen information

## Key Points

1. **Physical vs Logical**: Always consider DPI scaling when sizing windows
2. **Multi-Monitor**: Handle cases where users have multiple monitors
3. **Primary Monitor**: Not all systems have a clearly defined primary monitor
4. **Error Handling**: Monitor information might not be available on some platforms
5. **Dynamic Changes**: Monitor configuration can change at runtime (hot-plugging)

For a complete working example, see `examples/desktop_size.rs` in the winit repository.
