# Changelog

## 0.9.0 - 2025-11-06 13:28:39Z

### 🚀 Features

- *(wiki)* Add read-only parts of wiki page API
- *(wiki)* Add create/update and delete endpoints for wiki pages

### 🐛 Bug Fixes

- *(http)* Actually return Err on Http error status codes
- *(traits)* Fix the unimplemented_on message to take ignore_response_body into
  account
- *(test)* Fix the update project membership test

### 💼 Other

- *(release)* Add shell script to create a release

### ⚙️ Miscellaneous Tasks

- *(changelog)* Add git-cliff config file for CHANGELOG.md

## 0.8.0

change Rust edition to 2024
add Iterator/Stream interface
to avoid loading large results completely into memory
update dependencies

## 0.7.6

update dependencies

## 0.7.5

update dependencies

## 0.7.4

update dependencies
fix test locking and some other minor test issues

## 0.7.3

forgot to make thumbnail\_url public

## 0.7.2

update dependencies
fix clippy lint uninlined\_format\_args
fix some completeness test failures
thumbnail\_url in attachments
editable in custom\_fields
updated\_on and updated\_by in journal entries
skip serializing total\_estimated\_hours if it is None

## 0.7.1

re-export reqwest so users can construct clients with the correct reqwest
version

## 0.7.0

update dependencies
add extra parameter to new/from\_env to allow users to pass in a reqwest::Client
or reqwest::blocking::Client configured according to their needs, remove
rustls-tls feature since the user can just use their own dependency on reqwest
to configure the features

## 0.6.0

add new trait NoPagination that guards the use of the unpaginated API
so the user is forced to use the paginated API functions for endpoints that
do require pagination to return all results

update dependencies

## 0.5.6

minimize reqwest features to allow user of the library to decide if they want
to use openssl, rustls, http2,... without this library forcing a choice via
its use of the default features

## 0.5.5

update dependencies

## 0.5.4

update dependencies

## 0.5.3

update dependencies

## 0.5.2

update dependencies

## 0.5.1

update dependencies

## 0.5.0

add missing values for ProjectIncludes and ProjectsIncludes
some more fixes to missing fields in other data types

## 0.4.2

fix visibility on redmine\_api::api::issues::CustomField fields

## 0.4.1

update dependencies

## 0.4.0

add missing Clone instances all over
update dependencies

## 0.3.0

add async support (not very well tested yet)
replace parking\_lot lock in tests  with tokio one since the parking\_lot one
does not work with the async tests

## 0.2.9

update dependencies
fix visibility in RoleEssentials (fields were private)

## 0.2.8

update dependencies

## 0.2.7

update dependencies
replace derivative (unmaintained) with derive\_more

## 0.2.6

update dependencies

## 0.2.5

update dependencies

## 0.2.4

upgrade dependencies
update deny.toml to new format

## 0.2.3

upgrade dependencies

## 0.2.2

upgrade dependencies

## 0.2.1

upgrade dependencies

## 0.2.0

upgrade dependencies including some incompatible ones

## 0.1.10

Some extra Clone implementations
Custom fields in time entries

## 0.1.9

Fix SPDX license expression
Replace dotenv (unmaintained) with dotenvy
Update dependencies

## 0.1.8

make issues custom_fields optional

## 0.1.7

optional rustls support contributed by Alexander Yekimov <av.yekimov@yandex.ru>
fix clippy lints about unused lifetimes in impl blocks
add IssueStatusEssentials field is_closed which seems to be included in recent
Redmine versions

## 0.1.6

Fix missing `#[builder(default)]` on GetUser id field

## 0.1.5

Add From implementations for shared references for all existing From
implementations

## 0.1.4

Add From implementations for the Essentials types from the respective full types
where possible

## 0.1.3

Fix typo in TimeEntryActivity
Fix typos in comments

## 0.1.2

Make fields in TrackerEssentials public

## 0.1.1

SortByColumn in descending direction for issues should use :desc instead of
:rev keyword

Add some docs Errors sections

Add some must_use attributes to methods where that made sense (mostly the
builder() methods)

## 0.1.0

Initial Release
