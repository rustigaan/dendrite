use std::error::Error;
use prost::{EncodeError,Message};
use tonic::{Request, Response, Status};
use crate::axon_utils::{InitCommandSender, CommandSink, CommandSinkHandle, VecU8Message};
use crate::grpc_example::greeter_service_server::GreeterService;
use crate::grpc_example::{Acknowledgement, Greeting};

#[derive(Debug)]
pub struct GreeterServer {
    command_sink: CommandSinkHandle,
}

impl VecU8Message for Greeting
where
    Self: Sized
{
    fn encode_u8(&self, buf: &mut Vec<u8>) -> Result<(),EncodeError> {
        self.encode(buf)
    }
}

#[tonic::async_trait]
impl GreeterService for GreeterServer {
    async fn greet(
        &self,
        request: Request<Greeting>,
    ) -> Result<Response<Acknowledgement>, Status> {
        println!("Got a request: {:?}", request);
        let inner_request = request.into_inner();
        let result_message = inner_request.message.clone();

        self.command_sink.send_command("GreetCommand", Box::new(inner_request)).unwrap();

        let reply = Acknowledgement {
            message: format!("Hello {}!", result_message).into(),
        };

        Ok(Response::new(reply))
    }
}

pub async fn init() -> Result<GreeterServer, Box<dyn Error>> {
    InitCommandSender().await.map(|command_sink| {GreeterServer{ command_sink }})
}