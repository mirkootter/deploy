use crate::shared::SignedFile;

type HyperStream = hyper::client::Request<hyper::net::Streaming>;

pub struct Request<'sk> {
    secret_key: &'sk minisign::SecretKey,
    multipart: multipart::client::Multipart<HyperStream>
}

impl<'sk> Request<'sk> {
    pub fn new(secret_key: &'sk minisign::SecretKey, url: hyper::Url) -> Result<Self, ()> {
        let ssl = hyper_native_tls::NativeTlsClient::new().unwrap_or_else(|err| {
            eprintln!("Error: {:?}", err);
            std::process::exit(1);
        });
        let connector = hyper::net::HttpsConnector::new(ssl);
        let hyper_request = hyper::client::Request::with_connector(hyper::method::Method::Post, url, &connector).unwrap_or_else(|err| {
            eprintln!("Error: {:?}", err);
            std::process::exit(1);
        });
        let multipart = multipart::client::Multipart::from_request(hyper_request).or(Err(()))?;

        let req = Request {
            secret_key,
            multipart
        };

        Ok(req)
    }

    pub fn sign_and_add(&mut self, file: Vec<u8>, path: &std::path::Path) -> Result<(), &'static str> {
        let filename = path.file_name()
            .and_then(|filename| filename.to_str())
            .ok_or("Invalid file name")?;

        let signed_file = SignedFile::create_and_sign(file, filename.to_owned(), self.secret_key).or(Err("Could not sign the file"))?;
        let mut data = &signed_file.file as &[u8];
        
        self.multipart.write_stream("file", &mut data, Some(filename), None).or(Err(())).or(Err("Could not write the file"))?;
        self.multipart.write_text("signature", &signed_file.signature).or(Err(())).or(Err("Could not write the signature"))?;

        Ok(())
    }

    pub fn send(self) -> Result<(), ()> {
        let response = self.multipart.send().or(Err(()))?;
        println!("Server responded with status code {}", response.status);
        Ok(())
    }
}
