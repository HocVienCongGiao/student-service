# AWS Lambda functions for Terraform API

A project with the structure that supports multi function [Rustlang AWS Lambda](https://github.com/awslabs/aws-lambda-rust-runtime/) applications with ⚡ serverless framework ⚡ using [Cargo workspaces](https://doc.rust-lang.org/1.30.0/book/second-edition/ch14-03-cargo-workspaces.html)

## ✨ features

* 🦀 Auto-approve API to approve automatically TF workspaces that need human attention
- 🛵 Continuous integration testing with GitHub Actions
- 🚀 Continuous deployment with GitHub Actions
* 🧪 How to retrieve logs from EFS

## 📦 install

Install the [serverless framework](https://www.serverless.com/framework/docs/getting-started/) cli.

Then then run the following in your terminal
```bash
$ npm ci
```
`npm ci` will make sure npm dependencies are installed based directly on your package-lock.json file. This only needs run once.

## 🧙 how to package locally

```bash
$ npx serverless package
```
The first time you run `npx serverless package` it will pull down and compile the base set
of dependencies and your application. Unless the dependencies change afterwards,
this should only happen once, resulting in an out of the box rapid deployment
cycle.

Assuming you have [aws credentials with appropriate deployment permissions configured](https://serverless.com/framework/docs/providers/aws/guide/credentials/) (if you already use any existing AWS tooling installed you likely already have this configured), you could impress your friends by creating a project
that is _born_ in production and want to deploy the lambda functions using Serverless framework instead of using Terraform.

```bash
$ npx serverless deploy
```

## 🛵 continuous integration and deployment

This template includes an example [GitHub actions](https://travis-ci.org/) [configuration file](.github/workflows/build-push.yml) which can unlock a virtuous cycle of continuous integration and deployment
( i.e all tests are run on prs and every push to master results in a deployment ).

Store a `AWS_ACCESS_KEY_ID` `AWS_SECRET_ACCESS_KEY` used for aws deployment in your repositories secrets https://github.com/{username}/{my-new-service}/settings/secrets

Add your changes to git and push them to GitHub.

Finally, open https://github.com/{username}/{my-new-service}/actions in your browser and grab a bucket of popcorn 🍿

## 🔫 function triggering using Serverless framework
With your function deployed using Serverless framework, you can now start triggering it using `serverless` framework directly or
the AWS integration you've configured to trigger it on your behalf

```sh
$ npx serverless invoke -f hello -d '{"foo":"bar"}'
```

locally
```sh
$ npx serverless invoke local -f hello -d '{"hello":"world"}'
```
## 🔫 function triggering using AWS CLI
```sh
$ aws lambda invoke --region us-west-2 --function-name hellotfc  --payload 'eyJmb28iOiJiYXIifQo=' output.json
```

## 🔬 logs when deployed using the Serverless framework

With your function deployed you can now tail it's logs right from your project

```sh
$ npx serverless logs -f hello
```

```sh
$ npx serverless logs -f world
```

## 👴 retiring

Good code should be easily replaceable. Good code is should also be easily disposable. Retiring applications should be as easy as creating and deploying them them. The dual of `serverless deploy` is `serverless remove`. Use this for retiring services and cleaning up resources.

```bash
$ npx serverless remove
```

## ℹ️  additional information
* Configure your CLion-Rust to use rustfmt https://github.com/intellij-rust/intellij-rust/pull/3490
  
* See the [serverless-rust plugin's documentation](https://github.com/softprops/serverless-rust) for more information on plugin usage.

* See the [aws rust runtime's documentation](https://github.com/awslabs/aws-lambda-rust-runtime) for more information on writing Rustlang lambda functions

## 👯 Contributing
***

set SLS_DOCKER_ARGS=-v  D:\workspace\binhle\hvcg\saint-service\saint-app:/saint-app
https://www.jfrog.com/jira/browse/RTFACT-25926
set CARGO_HOME= D:\workspace\binhle\hvcg\saint-service\saint-lambda\.cargo
