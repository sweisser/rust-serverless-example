# AWS Lambda with AWS API Gateway invocation in Rust

Uses the https://github.com/awslabs/aws-lambda-rust-runtime.

# Installation

Build the ZIP file for upload to AWS using:

```` 
make bundle
````

Upload it to AWS Lambda and connect the API Gateway (REST API) to it.

Then call it using POST request and with Content-Type specified (won't work without).

````
curl -X POST -H 'Content-Type: application/json' -d '{"year": 2020}' https://[insert_your_uri_here].execute-api.us-east-2.amazonaws.com/default/holiday_lambda
````

Output should be like this:

````
{"holidays":{"easter_monday":"2020-04-13","easter_sunday":"2020-04-12","epiphany":"2020-01-06","good_friday":"2020-04-10","new_years_day":"2020-01-01","womens_day":"2020-03-08"},"version":"1.0"}
````
