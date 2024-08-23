# rust-lightning-node-rankings

## Requirements
* This template was tested with Rust v1.66.0 and above.

## Deploy the sample application

To deploy the application, you need the folllowing tools:

* SAM CLI - [Install the SAM CLI](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-sam-cli-install.html)
* Docker - [Install Docker community edition](https://hub.docker.com/search/?type=edition&offering=community)
* [Rust](https://www.rust-lang.org/) version 1.64.0 or newer
* [cargo-lambda](https://github.com/cargo-lambda/cargo-lambda) for cross-compilation

To build and deploy your application for the first time, run the following in your shell:

```bash
sam build
sam deploy
```

## Tests

Tests are defined alongside your lambda function code in the `rust_app/src` folder.

```bash
cargo test
```

## Cleanup

To delete the sample application that you created, use the AWS CLI. Assuming you used your project name for the stack name, you can run the following:

```bash
sam delete
```
