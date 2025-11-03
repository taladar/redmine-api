# redmine-api

API for the Redmine issue tracker

## State of this crate

The crate supports both sync and async usage but the async code is
significantly newer so it might not be as well tested.

Most API endpoints are supported and each supported API endpoint is tested with
at least one unit test though at this early stage I do make no stability
guarantees with regards to parameters that might need to be changed slightly
to accommodate variants of the API endpoint I missed (e.g. different filters I
am not aware of, the documentation on the Redmine wiki is far from complete).

Any such changes should not take a lot of effort when changing a client when
updating this crate though, most likely the existing parameter value would just
have to be wrapped in a new Enum.

## How to use this crate

All the examples in this README file assume that REDMINE\_API\_KEY and
REDMINE\_URL are set in either the environment or a .env file (if you do not use
the .env file you can skip the dotenv line).

There are five main ways to call API endpoints:

* ignoring the response body
* returning the JSON response body
* returning a single page from a query that supports pagination
* returning all pages from a query that supports pagination
* using an Iterator (for the sync API) or a Stream (for the async API)

All the examples below use the blocking Api but it is also possible to use
the async API merely by creating a reqwest::Client instead of a
reqwest::blocking::Client and using RedmineAsync instead of Redmine (and of
course adding .await where appropriate). They do use the re-exported version
of reqwest to avoid version conflicts if the user has another version of the
reqwest crate in their dependency graph.

### The Endpoint trait

Each API endpoint is represented by an object implementing the
[Endpoint](api::Endpoint) trait.

Each of them has a Builder to set any potential parameters. For uniformity this
pattern is used even when there are no parameters.

The vast majority of endpoints are in a module under api that matches the
Redmine wiki page that documents the API endpoint.

The exception is the [FileUpload](api::uploads::UploadFile) endpoint
and tests related to that which lives in api::uploads.

The endpoint is the generic parameter we do not explicitly specify in the calls
below.

### Wrapper types

A lot of API responses return the actual response wrapped in or require the
request to be wrapped in an extra JSON object with one key named after singular
or plural of the entity the call handles. I have provided generic Wrapper types,
e.g. [IssueWrapper](api::issues::IssueWrapper) to allow the user
of this crate to discard them as soon as possible.

I decided against completely hiding them away behind the API for non-pagination
queries since that might have given the user less flexibility.

### Using your own return types

Instead of the supplied return types it is possible to use your own types or
even something like `serde_json::Value`. This allows you to extract the fields
you need directly into your own types, e.g. just the Id.

### Essentials types

In the API responses we often find lists of other entities in a minimal form,
e.g. just the id and name. I have named these types after the main entity
followed by Essentials, e.g. [IssueEssentials](api::issues::IssueEssentials).

### Ignoring the response body

This is mainly useful in practice for queries that do not have a response body
(e.g. delete queries) or for queries that have a side-effect (like creating an
issue) where we do not care about the response body.

For illustrative purposes I am using the [GetIssue](api::issues::GetIssue)
endpoint here mainly to avoid accidental data loss from running a delete example
copied from here on a production Redmine instance.

```rust
use redmine_api::api::Redmine;
use redmine_api::api::issues::{Issue, IssuesWrapper, GetIssue};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;
    let client =
      redmine_api::reqwest::blocking::Client::builder()
        .use_rustls_tls()
        .build()?;
    let redmine = Redmine::from_env(client)?;
    let endpoint = GetIssue::builder().id(1).build()?;
    redmine.ignore_response_body::<_>(&endpoint)?;
    Ok(())
}
```

### Returning a JSON response

This is used for most of the API endpoints, it requires the endpoint to
implement the [ReturnsJsonResponse](api::ReturnsJsonResponse) trait
so it can not be accidentally used on an endpoint which always returns an empty
response body.

If this is used on an endpoint that requires pagination a compile time error
will be emitted to avoid accidentally writing code that ignores all but the
first page. This is implemented by having all endpoints that do not implement
pagination implement the NoPagination trait.

```rust
use redmine_api::api::Redmine;
use redmine_api::api::issues::{Issue, IssueWrapper, GetIssue};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;
    let client =
      redmine_api::reqwest::blocking::Client::builder()
        .use_rustls_tls()
        .build()?;
    let redmine = Redmine::from_env(client)?;
    let endpoint = GetIssue::builder().id(1).build()?;
    let IssueWrapper { issue } =
        redmine.json_response_body::<_, IssueWrapper<Issue>>(&endpoint)?;
    println!("Issue found:\n{:#?}", issue);
    Ok(())
}
```

### Pagination (single page)

This is used for a few API endpoints that can return large numbers of results.

It requires the endpoint to implement the [Pageable](api::Pageable)
trait so it can not accidentally be used on endpoints which do not support
pagination.

Since pagination queries always have an outer wrapper object which contain
the pagination keys (total\_count, offset, limit) which have to be parsed
anyway the explicit use of a Wrapper object is not required on pagination
queries.

However the call does require the offset and limit parameters and the response
is wrapped in the [ResponsePage](api::ResponsePage) struct to
return these values.

```rust
use redmine_api::api::{Redmine,ResponsePage};
use redmine_api::api::issues::{Issue, ListIssues};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;
    let client =
      redmine_api::reqwest::blocking::Client::builder()
        .use_rustls_tls()
        .build()?;
    let redmine = Redmine::from_env(client)?;
    let endpoint = ListIssues::builder().build()?;
    let ResponsePage { values: issues, total_count, offset, limit} =
        redmine.json_response_body_page::<_, Issue>(&endpoint, 3, 25)?;
    println!("Total count: {}", total_count);
    println!("Offset: {}", offset);
    println!("Limit: {}", limit);
    for issue in issues {
        println!("Issue found:\n{:#?}", issue);
    }
    Ok(())
}
```

### Pagination (all pages)

Most of the things said in the previous section also apply here.

Since we request all pages we do not require an offset or limit parameter
nor are the results wrapped in an extra object.

The downside of this is that it needs to accumulate all results in memory
before returning them so it is more meant to be used with something like
projects or groups where common numbers of results might just be over one
page and not so much for those where hundreds of result pages are expected
like unfiltered issues.

```rust
use redmine_api::api::Redmine;
use redmine_api::api::issues::{Issue, ListIssues};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;
    let client =
      redmine_api::reqwest::blocking::Client::builder()
        .use_rustls_tls()
        .build()?;
    let redmine = Redmine::from_env(client)?;
    let endpoint = ListIssues::builder().build()?;
    let issues =
        redmine.json_response_body_all_pages::<_, Issue>(&endpoint)?;
    for issue in issues {
        println!("Issue found:\n{:#?}", issue);
    }
    Ok(())
}
```

### Iterator

This is available on endpoints that require pagination as it is obviously
useless if no results are returned and the result from a single call can
easily be turned into an Iterator as seen in our loop above.

The main advantage over the all pages version is that the results can be
streamed instead of having to keep them all in memory at the same time.

This means the main advantage is for e.g. getting all the issues in a
redmine instance, not for entities like projects where the number of pages
are in the low single digits on most instances.

```rust
use redmine_api::api::Redmine;
use redmine_api::api::issues::{Issue, ListIssues};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;
    let client =
      redmine_api::reqwest::blocking::Client::builder()
        .use_rustls_tls()
        .build()?;
    let redmine = Redmine::from_env(client)?;
    let endpoint = ListIssues::builder().build()?;
    let issues =
        redmine.json_response_body_all_pages_iter::<_, Issue>(&endpoint);
    for issue in issues {
        let issue = issue?;
        println!("Issue found:\n{:#?}", issue);
    }
    Ok(())
}
```
