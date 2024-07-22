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
