/// # MessageBus
/// Implementation of the Observer patterns to allow us to share messages and events throughout the
/// app.
pub struct MessageBus{
    subscribers: Vec<fn(&str, &str)>,
}

impl MessageBus {
    pub fn new() -> Self {
        Self {
            subscribers: Vec::new(),
        }
    }

    pub fn register(&mut self, update: fn(&str, &str)){
        &mut self.subscribers.push(update);
    }

    pub fn unregister(&mut self, update: fn(&str, &str)){
        //TODO: Remove subscriber...

    }

    pub fn send_message(&self) {
        //TODO: Sends a message to subscribers...
        &self.subscribers.iter().for_each(|subscriber| subscriber("Hello", "World"));
    }

    pub fn get_message(&self)  {
        //TODO: Receive message from subscribers...

    }
}
