mod request;

pub fn main (secret_key: minisign::SecretKey, files: Vec<std::path::PathBuf>, url: &'_ str) {
    if files.is_empty() {
        eprintln!("At least one file must be provided");
        std::process::exit(1);
    }

    // Check is all files exist
    for file in &files {
        if !file.exists() {
            eprintln!("File not found: {}", file.display());
            std::process::exit(1);
        }
    }

    let read_file = |path| -> Result<_, std::io::Error> {
        let mut file = std::fs::File::open(path)?;
        let mut buf = Vec::new();

        use std::io::Read;
        file.read_to_end(&mut buf)?;
        Ok(buf)
    };

    let url = url.parse().unwrap_or_else(|_| {
        eprintln!("Invalid url: {}", url);
        std::process::exit(1);
    });

    let mut request = request::Request::new(&secret_key, url).unwrap_or_else(|_| {
        eprintln!("Failed to create multipart request");
        std::process::exit(1);
    });

    // Sign all files and send them
    for file in &files {
        let data = read_file(file).unwrap_or_else(|_| {
            eprintln!("Could not open file: {}", file.display());
            std::process::exit(1);
        });

        request.sign_and_add(data, file).unwrap_or_else(|msg| {
            eprintln!("{}: {}", msg, file.display());
            std::process::exit(1);
        });
    }

    request.send().unwrap_or_else(|_| {
        eprintln!("Request failed");
        std::process::exit(1);
    });
}