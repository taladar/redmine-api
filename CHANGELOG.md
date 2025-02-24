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

update depedencies

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
add IssueStatusEssentials field is_closed which seems to be included in recent Redmine versions

## 0.1.6

Fix missing #[builder(default)] on GetUser id field

## 0.1.5

Add From implementations for shared references for all existing From implementations

## 0.1.4

Add From implementations for the Essentials types from the respective full types where possible

## 0.1.3

Fix typo TimeEntryActvity -> TimeEntryActivity
Fix typos in comments

## 0.1.2

## Fixed

Make fields in TrackerEssentials public

## 0.1.1

## Fixed

SortByColumn in descending direction for issues should use :desc instead of :rev keyword

Add some docs Errors sections

Add some must_use attributes to methods where that made sense (mostly the builder() methods)

## 0.1.0

Initial Release
