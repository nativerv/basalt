pub trait Publisher {
  fn subscribe(&mut self, subscriber: Subscriber) -> usize;
  fn unsubscribe(&mut self, subscriber: usize);
  fn notify_all(&mut self);
}

pub type Subscriber = Box<dyn Fn()>;
