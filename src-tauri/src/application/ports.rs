trait Networkable {
    fn send_message(&self, message: &str) -> Result<(), String>;
    fn receive_message(&self) -> Result<String, String>;
}
trait Errorable {
    fn handle_error(&self, error: &str);
}
