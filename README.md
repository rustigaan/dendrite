# Rustic Dendrite

A [Rust](https://www.rust-lang.org) library to connect to [AxonServer](https://axoniq.io/product-overview/axon-server).

This project is a sibling of [dendrite2go](https://github.com/dendrite2go/dendrite) and [archetype-go-axon](https://github.com/dendrite2go/archetype-go-axon), but for the Rust programming language.

See the GitHub project [dendrite2go/archetype-rust-axon](https://github.com/dendrite2go/archetype-rust-axon) for an example of how to use this code.

## Core concepts

* [Command / Query Responsibility Segregation](http://codebetter.com/gregyoung/2010/02/16/cqrs-task-based-uis-event-sourcing-agh/) (CQRS)
* [Domain-driven Design](https://dddcommunity.org/learning-ddd/what_is_ddd/) (DDD)
* [Event Sourcing](https://axoniq.io/resources/event-sourcing)
* [Futures and async/await](https://rust-lang.github.io/async-book)
* [gRPC](https://grpc.io/)
* [Microservices](https://en.wikipedia.org/wiki/Microservices)
* [Service Mesh](https://buoyant.io/2017/04/25/whats-a-service-mesh-and-why-do-i-need-one/) (video: [talk by Jeroen Reijn](https://2019.jfall.nl/sessions/whats-a-service-mesh-and-why-do-i-need-one/))

## More information

* [dendrite on crates.io](https://crates.io/crates/dendrite)
* [dendrite on docs.rs](https://docs.rs/dendrite)

## Status

This project has now reached the level of Minimal Viable Deliverable in the sense that the first phase is completed: the current application communicates with AxonServer properly.
Like [archetype-go-axon](https://github.com/dendrite2go/archetype-go-axon) it can do the following:
1. ☑ Set up a session with AxonServer
   * ☑ Enable React app to call a RPC endpoint on the example-command-api service through grpc-web
2. ☑ Issue commands
3. ☑ Register a command handler and handle commands
4. ☑ Submit events
   * ☑ Stream events to UI
5. ☑ Retrieve the events for an aggregate and build a projection
   * ☑ Validate commands against the projection
6. ☑ Register a tracking event processor and handle events
7. ☑ Store records in a query model: Elastic Search
   * ☑ Store tracking token in Elastic Search
8. ☑ Register a query handler and handle queries
   * ☑ Show query results in UI

As well as:

* In-memory caching of aggregate projections

Now it would be nice to:

* Add macros to make the definition of handlers more ergonomic
* Add support for storing snapshots of aggregate projections in AxonServer.
* Add support for segmentation to distribute the load on tracking event processors.
* Add support for sagas.
* ...
