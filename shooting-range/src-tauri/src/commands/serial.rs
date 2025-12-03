use crate::serial::SerialHandler;
use tauri::State;
use std::sync::Mutex;

type SerialState<'a> = State<'a, Mutex<SerialHandler>>;

#[tauri::command]
pub fn send_target_command(
    node_id: i32,
    state: bool,
    serial: SerialState<'_>,
) -> Result<(), String> {
    let handler = serial.lock().map_err(|e| e.to_string())?;
    handler.send_command(node_id, state)
}

#[tauri::command]
pub fn get_serial_ports() -> Result<Vec<String>, String> {
    let ports = serialport::available_ports()
        .map_err(|e| e.to_string())?;
    
    Ok(ports.iter().map(|p| p.port_name.clone()).collect())
}

#[tauri::command]
pub fn connect_serial(
    port: String,
    serial: SerialState<'_>,
) -> Result<(), String> {
    let mut handler = serial.lock().map_err(|e| e.to_string())?;
    handler.disconnect();
    handler.set_port(port);
    handler.connect()?;
    Ok(())
}

#[tauri::command]
pub fn disconnect_serial(serial: SerialState<'_>) -> Result<(), String> {
    let handler = serial.lock().map_err(|e| e.to_string())?;
    handler.disconnect();
    Ok(())
}

#[tauri::command]
pub fn is_serial_connected(serial: SerialState<'_>) -> Result<bool, String> {
    let handler = serial.lock().map_err(|e| e.to_string())?;
    Ok(handler.is_connected())
}

