# opsgenie-rs

This is an unofficial Rust client for [OpsGenie API v2](https://docs.opsgenie.com/docs/api-overview).

The client is autogenerated from the OpenAPI definition found in the [OpsGenie Python SDK repository](https://github.com/opsgenie/opsgenie-python-sdk/blob/master/opsgenie-oas.yml).

OpsGenie also provide a repository for the OpenAPI specification, but it doesn't compile.

* Repository: https://github.com/opsgenie/opsgenie-oas
* GitHub issue related to the problems: https://github.com/opsgenie/opsgenie-oas/issues/40

## Generation

The client is generated with [OpenAPI Generator](https://github.com/OpenAPITools/openapi-generator).

```
docker run --rm \   
  -v ${PWD}:/local openapitools/openapi-generator-cli generate \
  -i https://raw.githubusercontent.com/opsgenie/opsgenie-python-sdk/master/opsgenie-oas.yml \
  -g rust \
  -o /local/ \
  --additional-properties=packageName=opsgenie-rs,packageVersion=0.1.0,library=reqwest,hideGenerationTimestamp=false
```

## Documentation for API Endpoints

See [DOCS.md](DOCS.md) for the autogenerated API documentation.

The [official OpsGenie API documentation](https://docs.opsgenie.com/docs/api-overview) may come in handy, too.