use chrono::Local;

pub struct TopBar {
    pub clock: String,
    pub network_status: String,
    pub battery_percent: String,
    pub volume_percent: String,
}

impl TopBar {
    pub fn new() -> Self {
        Self {
            clock: String::new(),
            network_status: "🌍".to_string(),
            battery_percent: "100%".to_string(),
            volume_percent: "75%".to_string(),
        }
    }
    
    pub fn update_clock(&mut self) {
        let now = Local::now();
        self.clock = now.format("%a, %d %b %H:%M").to_string();
    }
    
    pub fn update_network(&mut self, status: &str) {
        self.network_status = status.to_string();
    }
    
    pub fn update_battery(&mut self, percent: &str) {
        self.battery_percent = percent.to_string();
    }
    
    pub fn update_volume(&mut self, percent: &str) {
        self.volume_percent = percent.to_string();
    }
}