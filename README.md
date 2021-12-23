# deploy
Lightweight tool for simple deployment (server+client)

## Usage
You first need a key value pair:

`deploy generate-keys`

* Public-Key: Used on the server to verify the signature
* Private-Key: Necessary to sign files for deployment. Only signed files can be deployed, i.e. only people knowing the private key can upload to the server.

## Usage server
`deploy serve <publickey>`

Starts the server on port 3333 and deploys files in the current working directory.

Currently, you cannot change the port or the interface because the server is designed to work in docker.

## Usage client
`deploy deploy --private-key <privatekey> <url> <files>...`

The files are signed and transferred to the supplied url. If a server listen on this url, the files will be deployed.
