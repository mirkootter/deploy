mod request;

extern crate hyper;
extern crate multipart;

use hyper::server::{Handler, Server, Request, Response};
use hyper::status::StatusCode;
use hyper::server::response::Response as HyperResponse;
use multipart::server::hyper::{Switch, MultipartHandler, HyperRequest};
use multipart::server::{Multipart};

struct NonMultipart;
impl Handler for NonMultipart {
    fn handle(&self, _: Request, mut res: Response) {
        *res.status_mut() = StatusCode::ImATeapot;
        res.send(b"Please send a multipart req :(\n").unwrap();
    }
}

struct UploadHandler {
    public_key: minisign::PublicKey
}

impl UploadHandler {
    fn process_multipart(&self, mut multipart: Multipart<HyperRequest>) -> Result<(), ()> {
        println!("Processing...");
        let mut request_builder = request::RequestBuilder::new();
        let mut failed = false;
    
        let result = multipart.foreach_entry(|field| {
            if failed {
                return;
            }

            let filename = field.headers.filename;
    
            use std::io::Read;
            let mut buf = Vec::new();
    
            let ok = field.data.take(5 * 1024 * 1024).read_to_end(&mut buf).is_ok();
            if !ok {
                failed = true;
                return;
            }
    
            let ok = request_builder.add(&field.headers.name, filename, buf).is_ok();
            if !ok {
                failed = true;
                return;
            }
        });
    
        if failed || result.is_err() {
            return Err(());
        }
    
        let request = request_builder.build()?;
        request.verify(&self.public_key)?;
    
        request.save()?;
        
        Ok(())
    }
}

impl MultipartHandler for UploadHandler {
    fn handle_multipart(&self, multipart: Multipart<HyperRequest>, mut res: HyperResponse) {
        match self.process_multipart(multipart) {
            Ok(_) => {
                res.send("Worked".as_bytes()).unwrap();
            },
            Err(_) => {
                *res.status_mut() = StatusCode::BadRequest;
                res.send("An error occurred".as_bytes()).unwrap();
            }
        };
    }
}

pub fn main(public_key: minisign::PublicKey) {
    println!("Listening on 0.0.0.0:3333");
    Server::http("0.0.0.0:3333").unwrap().handle(
        Switch::new(
            NonMultipart,
            UploadHandler { public_key }
        )).unwrap();
}