# A rust cli to build and assemble serverless unikernels 

A simple command line utility that takes a yaml config , compiles and builds serverless unikernels from given serverless templates

## Setup

Clone this repo 

```
git clone https://github.com/lmzuccarelli/rust-serverless-interface-cli.git

```

Create an executable

```
cd rust-serverless-interface-cli

make clean-all

# creates a release (optimized) executable
# in the ./target/release directory
make build

```

## Usage

Create a config yaml (refer to the one in the config directory)

This assumes you have already created your serverless function - 
refer to the example template https://github.com/lmzuccarelli/rust-serverless-interface-template 


```
apiVersion: unikernel.serverless.io/v1alpha1
kind: ServerlessConfig
spec:
  services:
    - name: serverless-test
      serverlessTemplate: path:///home/lzuccarelli/Projects/rust-serverless-interface-template
      version: 0.1.1
      authors: 
        - lmzuccarelli luzuccar@redhat.com
        - mtroisi hello@marcotroisi.com
      description: "Simple demo of a unikernel serverless webservice"
      env:
        - name: LOG_LEVEL
          value: TRACE
        - name: SERVER_PORT
          value: 3030
        - name: IP
          value: "0.0.0.0"

```

The field ``` serverlessTemplate ``` can reference a local path or a git repo url

For local development while debugging, use the local path, once you are satisfied that the serverless function
and unikernel are working then commit the template to git (or similar)

As an example 

```
apiVersion: unikernel.serverless.io/v1alpha1
kind: ServerlessConfig
spec:
  services:
    - name: serverless-test
      serverlessTemplate: https://github.com/lmzuccarelli/rust-serverless-interface-template

```

The ``` env ``` block is used to create a unikernel config

Execute the cli to compile and create a u serverless unikernel

```
./target/release/serverless-interface-cli --loglevel debug generate --config-file config/serverless.yaml --working-dir . --no-cleanup
```

## Notes

This project depends on the NanoVM unikernel framework (massive shout out to the creators of this project) 

Refer to  https://github.com/nanovms/ops on how to install

At this point in time only Rust serverless is supported

We will be adding Golang and Nodejs support in a future release



