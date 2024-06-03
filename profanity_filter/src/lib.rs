#[derive(Debug, PartialEq, Eq)]
pub struct Message {
    content: String,
    user: String,

}


impl Message {
  pub fn new(message_content: String, message_user: String) -> Message {
    Message {
      content: message_content,
      user: message_user,
    }

  }
  pub fn send_ms(&self) -> Option<&str> {
    if self.content.is_empty() || self.content.contains("stupid"){
      None
    } else {
      Some(&self.content)
    }
  }
}

pub fn check_ms(ms: &Message) -> (bool, &str) {
   match ms.send_ms(){
     Some(s) => (true, s),
     None => (false, "ERROR: illegal")
   }
}
