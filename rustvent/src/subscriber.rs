use std::sync::Arc;

pub trait Subscriber {
   fn update(&self);
}

pub trait SubscriberAsync {
   fn update(&self);

   fn into_arc(self) -> Arc<(dyn SubscriberAsync + Send + Sync + 'static)> 
   where 
   Self: SubscriberAsync + Sized + Send + Sync + 'static {
      Arc::new(self)
   }
}

pub trait SubscriberAsyncMut {
   fn update_mut(&mut self);
}
