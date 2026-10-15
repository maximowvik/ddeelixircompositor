pub struct Dock {
    pub pinned_apps: Vec<String>,
    pub open_windows: Vec<WindowInfo>,
}

pub struct WindowInfo {
    pub title: String,
    pub app_id: String,
    pub is_active: bool,
}

impl Dock {
    pub fn new() -> Self {
        Self {
            pinned_apps: vec![
                "firefox".to_string(),
                "kitty".to_string(),
                "dolphin".to_string(),
            ],
            open_windows: Vec::new(),
        }
    }
    
    pub fn add_window(&mut self, info: WindowInfo) {
        self.open_windows.push(info);
    }
    
    pub fn remove_window(&mut self, app_id: &str) {
        self.open_windows.retain(|w| w.app_id != app_id);
    }
    
    pub fn update_active_window(&mut self, app_id: &str) {
        for window in &mut self.open_windows {
            window.is_active = window.app_id == app_id;
        }
    }
}