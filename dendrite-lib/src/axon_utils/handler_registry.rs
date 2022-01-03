use anyhow::{anyhow, Result};
use bytes::Bytes;
use futures_core::Future;
use regex::Regex;
use std::collections::HashMap;
use std::pin::Pin; // better to not use the internals of futures_util

pub(crate) type Applicator<T> = &'static (dyn Fn(&mut T) -> Result<()>);
pub(crate) type Deserializer<'a, T> = &'a (dyn Fn(Bytes) -> Result<T, prost::DecodeError> + Sync);
pub(crate) type SendFuture<R> = dyn Future<Output = R> + Send;
pub(crate) type PinFuture<R> = Pin<Box<SendFuture<R>>>;
pub(crate) type ResultFuture<R> = PinFuture<Result<R>>;
pub(crate) type Handler<'a, T, M, P, R> = &'a (dyn Fn(T, M, P) -> ResultFuture<R> + Sync);
pub(crate) type Wrapper<R, W> = &'static (dyn Fn(&str, &R) -> Result<W> + Sync);

/// Describes a registry for handlers for a particular type projection (or context) and a particular return type.
/// I tried to make it possible to pass an `async fn` directly to parameter `handler`, but the return
/// type after desugaring is unnameable
/// (https://internals.rust-lang.org/t/naming-the-return-type-of-an-async-function/10085).
/// So it has to be wrapped in a closure that `Box::pin`s the return value, like this:
///
/// ```rust
/// handler_registry.insert(
///     "GreetCommand",
///     &GreetCommand::decode,
///     &(|c| Box::pin(handle_greet_command(c)))
/// )?;
/// ```
///
/// P: type of the projection that serves as context for the handlers
/// W: type of the wrapped result
pub trait HandlerRegistry<P, M, W>: Send {
    fn register(&mut self, applicator: Applicator<Self>) -> Result<()>;
    fn insert<T: Send + Clone>(
        &mut self,
        name: &str,
        deserializer: Deserializer<'static, T>,
        handler: Handler<'static, T, M, P, ()>,
    ) -> Result<()>;
    fn insert_handle(&mut self, handle: Box<dyn SubscriptionHandle<P, M, W>>) -> Result<()>;
    fn insert_ignoring_output<T: Send + Clone, R: Clone>(
        &mut self,
        name: &str,
        deserializer: Deserializer<'static, T>,
        handler: Handler<'static, T, M, P, Option<R>>,
    ) -> Result<()>;
    fn insert_with_output<T: Send + Clone>(
        &mut self,
        name: &str,
        deserializer: Deserializer<'static, T>,
        handler: Handler<'static, T, M, P, Option<W>>,
    ) -> Result<()>;
    fn insert_with_mapped_output<T: Send + Clone, R: Clone>(
        &mut self,
        name: &str,
        deserializer: Deserializer<'static, T>,
        handler: Handler<'static, T, M, P, Option<R>>,
        type_name: &str,
        wrapper: Wrapper<R, W>,
    ) -> Result<()>;
    fn append_category_handle(
        &mut self,
        regex: &str,
        handle: Box<dyn SubscriptionHandle<P, M, W>>,
    ) -> Result<()>;
    fn get(&self, name: &str) -> Option<&dyn SubscriptionHandle<P, M, W>>;
}

/// Concrete struct that implements the `HandlerRegistry` trait.
pub struct TheHandlerRegistry<P: Send, M, W: Clone> {
    pub(crate) handlers: HashMap<String, Box<dyn SubscriptionHandle<P, M, W>>>,
    pub(crate) category_handlers: Vec<(Regex, Box<dyn SubscriptionHandle<P, M, W>>)>,
}

pub struct HandleBuilder<T, M, P, R, W>
where
    T: 'static,
    M: 'static,
    P: 'static,
    R: 'static,
    W: 'static,
{
    name: String,
    deserializer: Deserializer<'static, T>,
    handler: Handler<'static, T, M, P, Option<R>>,
    wrapper: Option<ResponseWrapper<'static, R, W>>,
}

impl<T, M, P, R, W> HandleBuilder<T, M, P, R, W>
where
    T: Clone + Send,
    M: Clone + Send,
    P: Clone + Send,
    R: Clone,
    W: Clone,
{
    pub fn new(
        name: &str,
        deserializer: Deserializer<'static, T>,
        handler: Handler<'static, T, M, P, Option<R>>,
    ) -> Self {
        HandleBuilder {
            name: name.to_string(),
            deserializer,
            handler,
            wrapper: None,
        }
    }
    pub fn ignore_output(mut self) -> HandleBuilder<T, M, P, R, W> {
        self.wrapper = None;
        self
    }
    pub fn with_mapped_output(
        mut self,
        type_name: &str,
        mapper: Wrapper<R, W>,
    ) -> HandleBuilder<T, M, P, R, W> {
        self.wrapper = Some(ResponseWrapper {
            type_name: type_name.to_string(),
            convert: mapper,
        });
        self
    }
    pub fn build(self) -> Box<dyn SubscriptionHandle<P, M, W>> {
        Box::new(Subscription {
            name: self.name,
            deserializer: self.deserializer,
            handler: self.handler,
            wrapper: self.wrapper,
        })
    }
}
impl<T, M, P, R> HandleBuilder<T, M, P, R, R>
where
    R: Clone,
{
    pub fn with_output(mut self) -> HandleBuilder<T, M, P, R, R> {
        self.wrapper = Some(ResponseWrapper {
            type_name: "UNKNOWN".to_string(),
            convert: &(|_, r| Ok((*r).clone())),
        });
        self
    }
}

impl<P, M, W> HandlerRegistry<P, M, W> for TheHandlerRegistry<P, M, W>
where
    P: Send + Clone,
    M: Clone + Send,
    W: Clone + 'static,
{
    fn register(&mut self, applicator: Applicator<Self>) -> Result<()> {
        applicator(self)
    }

    fn insert<T: Send + Clone>(
        &mut self,
        name: &str,
        deserializer: Deserializer<'static, T>,
        handler: Handler<'static, T, M, P, ()>,
    ) -> Result<()> {
        let name = name.to_string();
        let key = name.clone();
        let handle: Box<dyn SubscriptionHandle<P, M, W>> = Box::new(SubscriptionVoid {
            name,
            deserializer,
            handler,
        });
        if self.handlers.contains_key(&key) {
            return Err(anyhow!("Handler already registered: {:?}", key));
        }
        self.handlers.insert(key, handle.box_clone());
        Ok(())
    }

    fn insert_handle(&mut self, handle: Box<dyn SubscriptionHandle<P, M, W>>) -> Result<()> {
        let key = handle.name();
        if self.handlers.contains_key(&key) {
            return Err(anyhow!("Handler already registered: {:?}", key));
        }
        self.handlers.insert(key, handle.box_clone());
        Ok(())
    }

    fn insert_ignoring_output<T: Send + Clone, R: Clone>(
        &mut self,
        name: &str,
        deserializer: Deserializer<'static, T>,
        handler: Handler<'static, T, M, P, Option<R>>,
    ) -> Result<()> {
        let handle = HandleBuilder::new(name, deserializer, handler)
            .ignore_output()
            .build();
        self.insert_handle(handle)
    }

    fn insert_with_output<T: Send + Clone>(
        &mut self,
        name: &str,
        deserializer: Deserializer<'static, T>,
        handler: Handler<'static, T, M, P, Option<W>>,
    ) -> Result<()> {
        let handle = HandleBuilder::new(name, deserializer, handler)
            .with_output()
            .build();
        self.insert_handle(handle)
    }

    fn insert_with_mapped_output<T: Send + Clone, R: Clone>(
        &mut self,
        name: &str,
        deserializer: Deserializer<'static, T>,
        handler: Handler<'static, T, M, P, Option<R>>,
        type_name: &str,
        wrapper: Wrapper<R, W>,
    ) -> Result<()> {
        let handle = HandleBuilder::new(name, deserializer, handler)
            .with_mapped_output(type_name, wrapper)
            .build();
        self.insert_handle(handle)
    }

    fn append_category_handle(
        &mut self,
        regex: &str,
        handle: Box<dyn SubscriptionHandle<P, M, W>>,
    ) -> Result<()> {
        let regex = Regex::new(regex)?;
        let name = handle.name();
        let key = name.clone();
        if self.handlers.contains_key(&key) {
            return Err(anyhow!("Handler already registered: {:?}", key));
        }
        self.category_handlers.push((regex, handle.box_clone()));
        Ok(())
    }

    fn get(&self, name: &str) -> Option<&dyn SubscriptionHandle<P, M, W>> {
        self.handlers.get(name).map(|t| t.as_ref())
    }
}

/// Creates an empty handler registry for a type of projection and a type of return values that can
/// be populated with SubscriptionHandles.
pub fn empty_handler_registry<P: Send, M: Send + Clone, W: Clone>() -> TheHandlerRegistry<P, M, W> {
    TheHandlerRegistry {
        handlers: HashMap::new(),
        category_handlers: Vec::new(),
    }
}

#[tonic::async_trait]
pub trait SubscriptionHandle<P, M, W>: Send + Sync {
    fn name(&self) -> String;

    async fn handle(&self, buf: Vec<u8>, metadata: M, projection: P) -> Result<Option<W>>;

    fn box_clone(&self) -> Box<dyn SubscriptionHandle<P, M, W>>;
}

#[derive(Clone)]
struct Subscription<'a, P, T, M, R, W> {
    pub name: String,
    pub deserializer: Deserializer<'a, T>,
    pub handler: Handler<'a, T, M, P, Option<R>>,
    pub wrapper: Option<ResponseWrapper<'a, R, W>>,
}

#[derive(Clone)]
struct ResponseWrapper<'a, R, W> {
    pub type_name: String,
    pub convert: &'a (dyn Fn(&str, &R) -> Result<W> + Sync),
}

#[tonic::async_trait]
impl<P: Send + Clone, T: Send + Clone, M: Clone + Send, R: Clone, W: Clone>
    SubscriptionHandle<P, M, W> for Subscription<'static, P, T, M, R, W>
{
    fn name(&self) -> String {
        self.name.clone()
    }

    async fn handle(&self, buf: Vec<u8>, metadata: M, projection: P) -> Result<Option<W>> {
        let message: T = (self.deserializer)(Bytes::from(buf))?;
        if let Some(result) = (self.handler)(message, metadata, projection).await? {
            if let Some(wrapper) = self.wrapper.as_ref() {
                return Ok(Some((wrapper.convert)(&wrapper.type_name, &result)?));
            }
        }
        Ok(None)
    }

    fn box_clone(&self) -> Box<dyn SubscriptionHandle<P, M, W>> {
        Box::new(Subscription::clone(&self))
    }
}

#[derive(Clone)]
struct SubscriptionVoid<'a, P, M, T> {
    pub name: String,
    pub deserializer: &'a (dyn Fn(Bytes) -> Result<T, prost::DecodeError> + Sync),
    pub handler: &'a (dyn Fn(T, M, P) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> + Sync),
}

#[tonic::async_trait]
impl<P: Send + Clone, T: Send + Clone, M: Send + Clone, W: Clone + 'static>
    SubscriptionHandle<P, M, W> for SubscriptionVoid<'static, P, M, T>
{
    fn name(&self) -> String {
        self.name.clone()
    }

    async fn handle(&self, buf: Vec<u8>, metadata: M, projection: P) -> Result<Option<W>> {
        let message: T = (self.deserializer)(Bytes::from(buf))?;
        (self.handler)(message, metadata, projection).await?;
        Ok(None)
    }

    fn box_clone(&self) -> Box<dyn SubscriptionHandle<P, M, W>> {
        Box::new(SubscriptionVoid::clone(&self))
    }
}
