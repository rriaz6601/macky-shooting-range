use serialport::SerialPort;
use std::sync::{Arc, Mutex};
use std::io::Write;

pub struct SerialHandler {
    port: Arc<Mutex<Option<Box<dyn SerialPort>>>>,
    port_name: String,
}

impl SerialHandler {
    pub fn new(port_name: String) -> Self {
        Self {
            port: Arc::new(Mutex::new(None)),
            port_name,
        }
    }
    
    pub fn connect(&self) -> Result<(), String> {
        let port = serialport::new(&self.port_name, 115200)
            .timeout(std::time::Duration::from_millis(1000))
            .open()
            .map_err(|e| format!("Failed to open port: {}", e))?;
        
        *self.port.lock().unwrap() = Some(port);
        Ok(())
    }
    
    pub fn set_port(&mut self, port_name: String) {
        self.port_name = port_name;
    }
    
    pub fn send_command(&self, node_id: i32, state: bool) -> Result<(), String> {
        let mut port_guard = self.port.lock().unwrap();
        if let Some(ref mut port) = *port_guard {
            let command = format!("{},{}\n", node_id, if state { "true" } else { "false" });
            port.write_all(command.as_bytes())
                .map_err(|e| format!("Failed to write: {}", e))?;
            println!("Sent: {}", command.trim());
            Ok(())
        } else {
            // Simulate if not connected
            println!("Simulated send: node_id={}, state={}", node_id, state);
            Ok(())
        }
    }
    
    pub fn disconnect(&self) {
        *self.port.lock().unwrap() = None;
    }
    
    pub fn is_connected(&self) -> bool {
        self.port.lock().unwrap().is_some()
    }
}

