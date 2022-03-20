use std::sync::Arc;

pub struct Reply<T>
where T: MessageProcessor
{
  socket: Arc<futures::lock::Mutex<zmq::Socket>>,
  message_processor: T
}

pub trait MessageProcessor
{
  fn process_message(&self, message: &str) -> String;
}

impl<T: MessageProcessor> Reply<T>
{
  pub fn new(message_processor: T,
             connection_string: &str, is_server: bool) -> Reply<T>
  {
    let ctx = zmq::Context::new();
    let socket = ctx.socket(zmq::REP).unwrap();
    if is_server
    {
      socket.bind(connection_string).unwrap();
    }
    else
    {
      socket.connect(connection_string).unwrap();
    }
    let socket = Arc::new(futures::lock::Mutex::new(socket));
    Reply{message_processor, socket}
  }

  pub async fn receive(&self)
  {
    let message_result = self.socket.lock().await.recv_string(0);
    let response = match message_result
    {
      Ok(msg) => self.message_processor.process_message(&msg.unwrap()),
      Err(x) => format!("Error receiving message: {}", x)
    };
    tokio::spawn(
    {
      let socket = Arc::clone(&self.socket);
      async move
      {
        socket.lock().await.send(&response, 0).expect(format!("Failed to send: {}", &response).as_str());
      }
    });
  }
}


