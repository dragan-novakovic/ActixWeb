use actix_protobuf::*;
use actix_web::*;

#[derive(Clone, PartialEq, Message)]
pub struct MyObj {
    #[prost(int32, tag = "1")]
    pub number: i32,
    #[prost(string, tag = "2")]
    pub name: String,
}

pub fn index(msg: ProtoBuf<MyObj>) -> Result<HttpResponse> {
    println!("model: {:?}", msg);
    HttpResponse::Ok().protobuf(msg.0) // <- send response
}
