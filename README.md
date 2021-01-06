# opsgenie-rs

This is an unofficial Rust client for [OpsGenie API v2](https://docs.opsgenie.com/docs/api-overview).

The client is autogenerated from the OpenAPI definition found in the [OpsGenie Python SDK repository](https://github.com/opsgenie/opsgenie-python-sdk/blob/master/opsgenie-oas.yml).

This library is still in its early stages. The interface *could* change at any time.

It currently uses `reqwest` for HTTP calls, but I'm looking into converting this to bring-your-own-http-client. 

OpsGenie also provide a repository for the OpenAPI specification, but it doesn't compile.

* Repository: https://github.com/opsgenie/opsgenie-oas
* GitHub issue related to the problems: https://github.com/opsgenie/opsgenie-oas/issues/40

## Generation

The client is generated with [OpenAPI Generator](https://github.com/OpenAPITools/openapi-generator) v5.0.

```
java -jar openapi-generator-cli-5.0.0.jar generate \
  --input-spec opsgenie-oas.yml \
  --generator-name rust \
  --config generator.json \
  --type-mappings=DateTime="chrono::DateTime<chrono::offset::Utc>",Date="chrono::Date<chrono::offset::Utc>" \
  --import-mappings="chrono::DateTime<chrono::offset::Utc>",Date="chrono::Date<chrono::offset::Utc>"
```

## Documentation for API Endpoints

See [DOCS.md](DOCS.md) for the autogenerated API documentation.

The [official OpsGenie API documentation](https://docs.opsgenie.com/docs/api-overview) may come in handy, too.


## License

This project is distributed under the terms of the Apache License 2.0. Please see [LICENSE](LICENSE) for the full text.
