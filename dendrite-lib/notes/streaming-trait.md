Hi, I'm using Tonic to talk to a (Java based) system called [Axon Server](https://axoniq.io/product-overview/axon-server) using their gRPC API.

To improve testability I'd like to expose the API operations as a trait, like this:

When I try to implement it, however, it won't compile. It seems that type `StaticCommandProviderOutboundStream` isn't `'static` enough.

(You can guess the `ServerHandleTrait` from the `impl` ;-) )
```rust
type StaticCommandProviderOutboundStream =
    dyn Stream<Item = CommandProviderOutbound> + Send + 'static;
type CommandProviderOutboundStreamBox =
    Pin<Box<StaticCommandProviderOutboundStream>>;

struct ServerHandle {
    conn: Channel
}

// #[tonic::async_trait]
// trait ServerHandleTrait { ... }

#[tonic::async_trait]
impl ServerHandleTrait for ServerHandle
{
    async fn open_command_provider_inbound_stream(
        &self, request: CommandProviderOutboundStreamBox
    ) -> Result<
        tonic::Response<Streaming<CommandProviderInbound>>,
        tonic::Status
    > {
        let request = Request::new(request);
        let mut client = CommandServiceClient::new(self.conn.clone());
        client.open_stream(request).await
    }
}
```

When I remove the Box and make it an `impl` parameter, the compiler complains that `ServerHandleTrait` can't be made into an object. When I dereference the request, like this:
```rust
        client.open_stream(*request).await
```
the compiler complains that the size of `*request` isn't known at compile time. What can I do?

----

It still bothers me that I expect to be able to expose a method on an async trait, but the compiler won't let me. I struggle with this before. Is there some rule that says "trait, async, and impl: pick two"?

Let me try to explain more clearly. So the signature of the Tonic generated `open_stream` is
```rust
pub struct CommandServiceClient<T> {
    inner: tonic::client::Grpc<T>,
}
impl<T> CommandServiceClient<T> {
    pub async fn open_stream(
        &mut self,
        request: impl tonic::IntoStreamingRequest<
            Message=super::CommandProviderOutbound,
        >,
    ) -> Result<...> {}
}
```
I understand this is just another way of saying
```rust
impl<T> CommandServiceClient<T> {
    pub async fn open_stream<
        R: tonic::IntoStreamingRequest<
          Message=super::CommandProviderOutbound,
        >
    >(
        &mut self,
        request: R,
    ) -> Result<...> {}
}
```
Now if I create a stream, like so
```rust
async fn test() {
    let my_stream = stream! {...};
    let _out_stream = client.open_stream(stream).await.unwrap();
}
```
Then this compiles, because `stream!` creates an instance of an anonymous type that fits the bound for `<R>`. Now I want to refactor it like this:
```rust
async fn test() {
    let my_stream = stream! {...};
    exec(my_stream).await;
}
async fn exec(stream: impl tonic::IntoStreamingRequest<
    Message=super::CommandProviderOutbound,
>) {
    // Enrich the stream with metadata
    let _out_stream = client.open_stream(stream).await.unwrap();
    // Post-process the _out_stream
}
```
And I want to make `exec` somebody else's problem by defining it on a trait and let them implement it:
```rust
#[tonic::async_trait]
trait StreamExecutor {
    async fn exec(stream: impl tonic::IntoStreamingRequest<
        Message=super::CommandProviderOutbound,
    >);
}
```
Now if I try to make a struct implement this trait and pass it around, the compiler complains that `StreamExecutor` cannot be made into an object because of the (implicit) generic parameter (remember `<R>`?). So, as the documentation states that the compiler can simply but non-trivially derive an `impl` from a `dyn`, I tried to turn the argument in a `dyn`. However, a `dyn` is not `Size`, so I had to wrap it up in a `Box`. And because of async I had to `Pin` it and adorn it with `Send + 'static`, so:
```rust
#[tonic::async_trait]
trait StreamExecutor {
    async fn exec(stream: Pin<Box<dyn tonic::IntoStreamingRequest<
        Message=super::CommandProviderOutbound,
    > + Send + 'static>>);
}
```
Now the trait compiles, I can call it with a `stream!` argument, and I can pass implementations of it around. Yeay!

Alas, I can't call `open_stream` anymore in the implementation
```rust
#[tonic::async_trait]
impl StreamExecutor for TestStruct {
    async fn exec(stream: Pin<Box<dyn tonic::IntoStreamingRequest<
        Message=super::CommandProviderOutbound,
    > + Send + 'static>>) {
        // Enrich the stream with metadata
        let _out_stream = client.open_stream(stream).await.unwrap();
        // Post-process the _out_stream
    }
}
```
because the compiler complains that
```
the type `Pin<Box<dyn Stream<Item = CommandProviderOutbound> + Send>>` does not fulfill the required lifetime
...
= note: type must satisfy the static lifetime
```
Note that the `'static` that I put after the `Send` was completely ignored. Also, the stream lived long enough when it was still an `impl`. Wrapping it up in a `Box` shouldn't make it any less `'static`, right? Why won't the compiler let me do this?