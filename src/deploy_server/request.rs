use crate::shared::SignedFile;

pub struct Request {
    files: Vec<SignedFile>
}

impl Request {
    pub fn verify(&self, pk: &minisign::PublicKey) -> Result<(), ()> {
        for signed_file in &self.files {
            signed_file.verify(pk)?;
        }

        Ok(())
    }

    pub fn save(&self) -> Result<(), ()> {
        for file in &self.files {
            match file.deploy() {
                Ok(_) => println!("  Deployed file: {}", file.filename),
                Err(_) => {
                    eprintln!("  Failed to deploy file: {}", file.filename);
                    return Err(());
                }
            }
        }
        Ok(())
    }
}

pub struct RequestBuilder {
    files: Vec<(String, Vec<u8>)>,
    signatures: Vec<Vec<u8>>
}

impl RequestBuilder {
    pub fn new() -> Self {
        RequestBuilder {
            files: Vec::new(),
            signatures: Vec::new()
        }
    }

    pub fn add(&mut self, name: &'_ str, filename: Option<String>, data: Vec<u8>) -> Result<bool, ()> {
        match name {
            "signature" => self.signatures.push(data),
            "file" => {
                let filename = filename.ok_or(())?;
                self.files.push((filename, data));
            },
            _ => return Ok(false)
        }
        Ok(true)
    }

    pub fn build(self) -> Result<Request, ()> {
        if self.signatures.len() != self.files.len() {
            return Err(());
        }
        if self.files.is_empty() {
            return Err(());
        }

        let mut signed_files = Vec::new();
        for ((filename, file), signature) in self.files.into_iter().zip(self.signatures.into_iter()) {
            signed_files.push(SignedFile::new(file, signature, filename)?);
        }

        let request = Request {
            files: signed_files
        };

        Ok(request)
    }
}
