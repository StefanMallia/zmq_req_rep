use std::sync::Arc;

pub struct Request
{
  socket: Arc<futures::lock::Mutex<zmq::Socket>>,
}

impl Request
{
  pub fn new(connection_string: &str, is_server: bool) -> Request
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
    Request{socket}
  }

  pub fn send_string(&self, data: &str)
  {
    let message = data.to_string();
    tokio::spawn(
    {
      let socket = Arc::clone(&self.socket);
      async move
      {
        socket.lock().await.send(&message, 0).expect(format!("Failed to send: {}", &message));
      }
    });
  }

  pub fn send<T>(&self, data: &T)
    where T: ?Sized + serde::Serialize
  {
    let message = &serde_json::to_string(data).unwrap().as_str();
    tokio::spawn(
    {
      let socket = Arc::clone(&self.socket);
      async move
      {
        socket.lock().await.send(&message , 0).expect(format!("Failed to send: {}", &message));
      }
    });
  }
}


