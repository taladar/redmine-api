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

There are four main ways to call API endpoints:

* ignoring the response body
* returning the JSON response body
* returning a single page from a query that supports pagination
* returning all pages from a query that supports pagination

### The Endpoint trait

Each API endpoint is represented by an object implementing the [Endpoint](api::Endpoint) trait.

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
even something like [serde_json::Value]. This allows you to extract the fields
you need directly into your own types, e.g. just the Id.

### Essentials types

In the API responses we often find lists of other entities in a minimal form, e.g.
just the id and name. I have named these types after the main entity followed by
Essentials, e.g. [IssueEssentials](api::issues::IssueEssentials).

### Ignoring the response body

This is mainly useful in practice for queries that do not have a response body
(e.g. delete queries) or for queries that have a side-effect (like creating an
issue) where we do not care about the response body.

For illustrative purposes I am using the [ListIssues](api::issues::ListIssues)
endpoint here mainly to avoid accidental data loss from running
a delete example copied from here on a production Redmine instance.

```
use redmine_api::api::Redmine;
use redmine_api::api::issues::{Issue, IssuesWrapper, ListIssues};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv()?;
    let redmine = Redmine::from_env()?;
    let endpoint = ListIssues::builder().build()?;
    redmine.ignore_response_body::<_>(&endpoint)?;
    Ok(())
}
```

### Returning a JSON response

This is used for most of the API endpoints, it requires the endpoint to
implement the [ReturnsJsonResponse](api::ReturnsJsonResponse) trait
so it can not be accidentally used on an endpoint which always returns an empty
response body.

If it is used on an endpoint supporting pagination it will just return the first
page. This is a Redmine behaviour, not implemented by this crate.

```
use redmine_api::api::Redmine;
use redmine_api::api::issues::{Issue, IssuesWrapper, ListIssues};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv()?;
    let redmine = Redmine::from_env()?;
    let endpoint = ListIssues::builder().build()?;
    let IssuesWrapper { issues } =
        redmine.json_response_body::<_, IssuesWrapper<Issue>>(&endpoint)?;
    for issue in issues {
        println!("Issue found:\n{:#?}", issue);
    }
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

```
use redmine_api::api::{Redmine,ResponsePage};
use redmine_api::api::issues::{Issue, ListIssues};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv()?;
    let redmine = Redmine::from_env()?;
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

```
use redmine_api::api::Redmine;
use redmine_api::api::issues::{Issue, ListIssues};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv()?;
    let redmine = Redmine::from_env()?;
    let endpoint = ListIssues::builder().build()?;
    let issues =
        redmine.json_response_body_all_pages::<_, Issue>(&endpoint)?;
    for issue in issues {
        println!("Issue found:\n{:#?}", issue);
    }
    Ok(())
}
```
