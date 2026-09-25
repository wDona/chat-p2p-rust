trait Networkable {
    fn send_message(&self, message: &str) -> Result<(), String>;
    fn receive_message(&self) -> Result<String, String>;
    fn connect_peer(&self, address: &str, port: u16) -> Result<(), String>;
    fn disconnect_peer(&self, address: &str, port: u16) -> Result<(), String>;
}
trait Errorable {
    fn handle_error(&self, error: &str);
}
