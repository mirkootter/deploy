pub struct SignedFile {
    pub file: Vec<u8>,
    pub filename: String,
    pub signature: String
}

impl SignedFile {
    pub fn new(file: Vec<u8>, signature: Vec<u8>, filename: String) -> Result<Self, ()> {
        let signature = String::from_utf8(signature).or(Err(()))?;
        let signed_file = SignedFile {
            file,
            filename,
            signature
        };

        Ok(signed_file)
    }

    pub fn create_and_sign(file: Vec<u8>, filename: String, secret_key: &minisign::SecretKey) -> Result<Self, ()> {
        let mut cursor = std::io::Cursor::new(&file);
        let signature = minisign::sign(None, &secret_key, &mut cursor, false, None, None).or(Err(()))?;
        let signed_file = SignedFile {
            file,
            filename,
            signature: signature.into_string()
        };

        Ok(signed_file)
    }

    pub fn verify(&self, public_key: &minisign::PublicKey) -> Result<(), ()> {
        let mut cursor = std::io::Cursor::new(&self.file);
        let signature = minisign::SignatureBox::from_string(&self.signature).or(Err(()))?;
        minisign::verify(public_key, &signature, &mut cursor, true, false).or(Err(()))
    }

    pub fn deploy(&self) -> Result<(), ()> {
        let current_dir = std::env::current_dir().or(Err(()))?;

        use std::io::Write;

        let path = std::path::Path::new(&self.filename);
        let path = path.canonicalize().or(Err(()))?;
        let path = path.strip_prefix(current_dir).or(Err(()))?;

        let mut f = std::fs::File::create(path).or(Err(()))?;
        f.write_all(&self.file).or(Err(()))?;
        Ok(())
    }
}
