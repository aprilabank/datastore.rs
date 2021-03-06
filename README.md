Cloud Datastore Rust Client
===========================

`datastore.rs` is a hand-rolled Rust client for [Google Cloud Datastore][].

It uses the `v1` API defined [here][] and exposes both a low-level API mapping
to the REST-API operations and some slightly higher-level convenience functions.

Not all operations and types are implemented yet.

Serialisation to and from entities is performed via [serde]().

# Rationale

Several half-finished Datastore clients already exist for Rust, however they seem to have been generated from either the
Google API description service and not work at all or from outdated gRPC definitions.

In order to provide a nicely usable client and integration with the Rust ecosystem, e.g. Serde, hand-rolling a client
seemed like the better option. Plus it's fun!

# Authentication

Ideally this would support all forms of authentication that the Google Cloud SDK
in other languages supports, but I"m not sure if there is a Rust library that has
implemented that already.

Required for now are:

* [ ] local `application-default` credentials
* [ ] token via [metadata endpoint][]
 
# Completeness overview
 
## Types

* [x] [Entity](https://cloud.google.com/datastore/docs/reference/rest/v1/Entity)
* [ ] [EntityResult](https://cloud.google.com/datastore/docs/reference/rest/v1/EntityResult)
* [x] [Key](https://cloud.google.com/datastore/docs/reference/rest/v1/Key)
* [x] [Value](https://cloud.google.com/datastore/docs/reference/rest/v1/projects/runQuery#Value)
* [x] [PartitionId](https://cloud.google.com/datastore/docs/reference/rest/v1/PartitionId)
* [ ] [ReadOptions](https://cloud.google.com/datastore/docs/reference/rest/v1/ReadOptions)
* [ ] [CommonMetadata](https://cloud.google.com/datastore/docs/reference/rest/Shared.Types/CommonMetadata)
* [ ] [EntityFilter](https://cloud.google.com/datastore/docs/reference/rest/Shared.Types/EntityFilter)
* [x] [LatLng](https://cloud.google.com/datastore/docs/reference/rest/Shared.Types/LatLng)
* [ ] [Operation](https://cloud.google.com/datastore/docs/reference/rest/Shared.Types/Operation)
* [ ] [OperationType](https://cloud.google.com/datastore/docs/reference/rest/Shared.Types/OperationType)
* [ ] [Progress](https://cloud.google.com/datastore/docs/reference/rest/Shared.Types/Progress)
* [ ] [State](https://cloud.google.com/datastore/docs/reference/rest/Shared.Types/State)

## Methods on entities

* [ ] [allocateIds](https://cloud.google.com/datastore/docs/reference/rest/v1/projects/allocateIds)
* [ ] [beginTransaction](https://cloud.google.com/datastore/docs/reference/rest/v1/projects/beginTransaction)
* [ ] [commit](https://cloud.google.com/datastore/docs/reference/rest/v1/projects/commit)
* [ ] [lookup](https://cloud.google.com/datastore/docs/reference/rest/v1/projects/lookup)
* [ ] [rollback](https://cloud.google.com/datastore/docs/reference/rest/v1/projects/rollback)
* [ ] [runQuery](https://cloud.google.com/datastore/docs/reference/rest/v1/projects/runQuery)

## Methods on operations

[Google Cloud Datastore]: https://cloud.google.com/datastore/
[here]: https://cloud.google.com/datastore/docs/reference/rest/
[serde]: https://serde.rs/
[metadata-endpoint]: https://cloud.google.com/compute/docs/storing-retrieving-metadata
