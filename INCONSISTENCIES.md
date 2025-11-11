# Redmine API Inconsistencies

**Checked API Endpoints:** `issues` , `projects` , `users` , `time_entries` ,
`attachments` , `custom_fields` , `enumerations` , `files` , `groups` ,
`issue_categories` , `issue_relations` , `issue_statuses` , `my_account` ,
`news` , `project_memberships` , `queries` , `roles` , `trackers` , `uploads` ,
`versions` , `wiki_pages`

This document lists the inconsistencies found between the Redmine server
implementation and the `redmine-api` Rust client. The primary source of truth
for the server-side implementation is the Redmine codebase itself (e.g.,
`app/models/issue_query.rb` ).

## `issues` Endpoint

The `issues` endpoint in the client is missing a significant number of filters.

### Missing Filters

The following filters are available on the Redmine server but are not
implemented in the `redmine-api` client.

#### Time Tracking

- `spent_time`
- `closed_on`
- `estimated_hours` (noted as TODO in client)
- `done_ratio` (noted as TODO in client)

#### User Attributes

- `author.group`
- `author.role`
- `member_of_group` (filter by assignee's group)
- `assigned_to_role` (filter by assignee's role)

#### Version Attributes

- `fixed_version.due_date`
- `fixed_version.status`

#### Other Properties

- `notes` (journal content)
- `watcher_id`
- `updated_by`
- `last_updated_by`
- `attachment`
- `attachment_description`
- `project.status`

#### Special Filters

- `any_searchable` (free-text search across multiple fields)

#### Custom Fields

- The client has a TODO for a generic mechanism to filter on custom fields,
  which is fully supported by the server.

### `include` Parameter

- **Get a single issue (`GetIssue`)** : The client correctly implements all
  available `include` options ( `children` , `attachments` , `relations` ,
  `changesets` , `journals` , `watchers` , `allowed_statuses` ).
- **List issues (`ListIssues`)** : The client only supports `attachments` and
  `relations` . The server-side implementation for the list view also seems to
  have a limited scope for includes.

---

## `projects` Endpoint

### `include` Parameter

- The client correctly implements all `include` parameters documented by the
  official Redmine API. While the `Project` model has more associations, they do
  not appear to be exposed via the `include` parameter in the API.

---

## `users` Endpoint

### Missing Filters

The `ListUsers` endpoint is missing numerous filters supported by the server:

- `auth_source_id`: Filter by authentication source.
- `twofa_scheme`: Filter by the two-factor authentication scheme.
- `admin`: A boolean filter to find only administrators.
- `created_on`, `last_login_on`: Date-based filters.
- `login` , `firstname` , `lastname` , `mail` : While the generic `name` filter
  exists, the API also supports exact matches on these individual fields.
- Custom Fields: Filtering by custom fields (e.g., `cf_x`) is not implemented.

---

## `time_entries` Endpoint

### Missing Filters

The `ListTimeEntries` endpoint is missing a large number of filters supported by
the server. The client only supports basic filtering, while the server allows
filtering on attributes of the time entry, the associated issue, user, project,
and custom fields.

#### Key Missing Filters

- **Time Entry Attributes** : `author_id` , `comments` (text search), `hours`
  (float comparison).
- **Associated Issue Attributes** : `issue.tracker_id` , `issue.status_id` ,
  `issue.fixed_version_id` , `issue.subject` , etc.
- **Associated User Attributes**: `user.group`, `user.role`.
- **Project Attributes**: `project.status`, `subproject_id`.
- **Custom Fields** : A generic mechanism to filter on custom fields for time
  entries and their associated objects.

---

## `attachments` Endpoint

### Missing Endpoint

- The client is missing an `UpdateAttachment` endpoint corresponding to
  `PUT /attachments/:id.json` . The server supports updating an attachment's
  `filename` and `description` .

### Incomplete Data Model

The `Attachment` struct is missing the following fields returned by the API:

- `digest` : A string containing a hash of the file content (e.g., SHA256 or
  MD5).
- `downloads`: An integer representing the download count.

---

## `custom_fields` Endpoint

### Incomplete Data Model

The `CustomField` struct is missing numerous fields returned by the API:

- `is_for_all` (boolean)
- `position` (integer)
- `url_pattern` (string)
- `text_formatting` (string)
- `edit_tag_style` (string)
- `user_role`
- `version_status`
- `extensions_allowed` (string)
- `full_width_layout` (boolean)
- `thousands_delimiter` (boolean)
- `ratio_interval` (float)

---

## `enumerations` Endpoint

### Missing `active` Field in Client Models

The Rust data structures ( `IssuePriority` , `TimeEntryActivity` ,
`DocumentCategory` ) are all missing the `active: bool` field, which is
explicitly included in the server's API response.

### Divergent Endpoint Implementation Strategy

The client uses separate, hardcoded endpoint structs for each enumeration type (
`ListIssuePriorities` , etc.), while the server offers a more generic
`GET /enumerations.json?type=IssuePriority` approach. This is a design choice,
but a more flexible and maintainable client could be achieved by refactoring to
use a single `ListEnumerations` endpoint builder that takes a type parameter.
---

## `files` Endpoint

**Summary:** The Rust Redmine API client's `files` endpoint ( `src/api/files.rs`
) is currently a placeholder with no functional implementation. In contrast, the
Redmine server provides robust API endpoints for listing and uploading
attachments. This represents a significant gap in the client's capabilities.

**Details of Redmine Server's Files API
  (based on `files_controller.rb` and `Attachment` model):**

### 1. List Project Files (GET)

- **Endpoint:** `GET /projects/:project_id/files.json`
- **Description:** Retrieves a list of attachments associated with a given
  project and its versions.
- **Authentication:** Required.
- **Parameters:**
  - `project_id`: (Required) The ID of the project.
  - `sort` : (Optional) String indicating the sort order. Supported fields:
    `filename` , `created_on` , `size` (maps to `Attachment.filesize` ),
    `downloads` . Example: `filename:asc` , `created_on:desc` .
- **Expected Response (200 OK):** A JSON array of attachment objects. Each
    object would typically include:
  - `id` (Integer)
  - `filename` (String)
  - `description` (String, optional)
  - `content_type` (String)
  - `filesize` (Integer, in bytes)
  - `downloads` (Integer)
  - `author` (User object, including `id`, `name`, etc.)
  - `created_on` (DateTime)
  - `container_id` (Integer)
  - `container_type` (String, e.g., "Project", "Version", "Issue", "Document")
  - `digest` (String, SHA256 hash of file content)
  - `token` (String, for direct download links, e.g., `id.digest`)

### 2. Upload File (POST)

- **Endpoint:** `POST /projects/:project_id/files.json`
- **Description:** Uploads one or more files to a project, optionally
  associating them with a specific version.
- **Authentication:** Required.
- **Parameters (in request body):**
  - `project_id`: (Required, from URL) The ID of the target project.
  - `version_id` : (Optional) The ID of a specific version within the project to
    link the attachment to.
  - `attachments` : (Required for multiple files) An array of file uploads
    (expected as `multipart/form-data` ).
  - `file` : (Required for single file) The file content itself, or potentially
    a `token` if using a pre-uploaded temporary file mechanism (as suggested by
    `Attachment.find_by_token` and controller logic).
- **Request Body:** `multipart/form-data` for file content.
- **Expected Response (200 OK):** A minimal success response (e.g.,
  `{"status": "ok"}` ) or details of the newly created attachment(s).
- **Expected Response (400 Bad Request):** An error message detailing the reason
  for failure (e.g., file size limits, invalid extensions, missing parameters).

**Recommendation:** The Rust client's `files.rs` needs to be implemented to
provide functionality for both listing and uploading files, aligning with the
Redmine server's existing API. This would involve defining appropriate Rust
structs for request parameters and response bodies, and implementing HTTP client
calls to the identified endpoints.
---

## `groups` Endpoint

### `ListGroups` Endpoint

- **Missing Pagination** : The client's `ListGroups` endpoint does not support
  pagination, while the server does.
- **Missing `builtin` Filter** : The client is missing a filter for `builtin`
  groups.

### `CreateGroup` and `UpdateGroup` Endpoints

- **Missing `twofa_required` Attribute** : The client does not support the
  `twofa_required` attribute.
- **Missing Custom Fields** : The client does not support custom fields for
  groups.

### `Group` Model

- The server's `Group` model has `lastname` aliased as `name` . The client's
  `Group` struct only has `name` . This might not be an inconsistency if the API
  always returns `name` , but it's worth noting.

---

## `issue_categories` Endpoint

### Inconsistency in Delete Operation

- **Server-side** : The `destroy` action ( `DELETE /issue_categories/:id.json` )
  supports optional parameters `reassign_to_id` and `todo` to reassign issues
  linked to the category being deleted. If these are not provided, issues'
  `category_id` is nullified.
- **Client-side** : The Rust client's `DeleteIssueCategory` endpoint only
  accepts the `id` of the category. It does not expose `reassign_to_id` or
  `todo` parameters.
- **Impact** : The Rust client cannot explicitly reassign issues during category
  deletion, only nullify their category.

---

## `issue_relations` Endpoint

### Inconsistency in `CreateIssueRelation` Endpoint

- **Server-side** : The Redmine server's API for creating issue relations (
  `POST /issues/:issue_id/relations.json` ) supports creating *multiple*
  relations in a single request by allowing the `issue_to_id` parameter to be a
  comma-separated list of issue IDs.
- **Client-side** : The Rust client's `CreateIssueRelation` struct and
  associated builder only accept a *single* `u64` for `issue_to_id` .
- **Impact** : The Rust client cannot leverage the batch creation capability of
  the server.

---

## `issue_statuses` Endpoint

No inconsistencies were found between the Rust Redmine API client's
`issue_statuses` endpoint and the Redmine server's API implementation.
---

## `my_account` Endpoint

### Missing `UpdateMyAccount` Endpoint

- The client only implements `GetMyAccount` . The server supports
  `PUT /my/account.json` to update the current user's details.

### Incomplete `MyAccount` Struct

The `MyAccount` struct in the client is missing several fields that are
available in the `User` model and can be returned by the API:

- `twofa_scheme`: (string)
- `auth_source_id`: (u64)
- `language`: (string)
- `mail_notification`: (enum/string)
- `must_change_passwd`: (bool)
- `passwd_changed_on`: (datetime)

---

## `news` Endpoint

### Missing API Endpoints

The Rust client only implements listing news items. It is missing:

- `GET /news/:id.json` (Get Single News Item)
- `POST /projects/:project_id/news.json` (Create News Item)
- `PUT /news/:id.json` (Update News Item)
- `DELETE /news/:id.json` (Delete News Item)

### Inconsistencies in Returned Fields

No inconsistencies were found in the fields returned by the `ListNews` and
`ListProjectNews` endpoints compared to the Redmine API documentation. The
`News` struct in the Rust client accurately reflects the documented fields (
`id` , `project` , `author` , `title` , `summary` , `description` , `created_on`
).

### Inconsistencies in Parameters/Filters

No inconsistencies were found in the parameters or filters for the implemented
listing endpoints. The Rust client correctly handles project-specific news
listing and implicitly supports pagination as per the Redmine API.

### Impact and Recommendations

The current Rust client for news is read-only for listing operations. To achieve
full parity with the Redmine API, the following should be implemented:

1. **`GetNews` Endpoint**: To retrieve a single news item by ID.
2. **`CreateNews` Endpoint** : To allow creation of new news items, including
   support for title, summary, description, and attachments.
3. **`UpdateNews` Endpoint** : To allow modification of existing news items,
   including support for title, summary, description, and attachments.
4. **`DeleteNews` Endpoint**: To allow removal of news items.

---

## `project_memberships` Endpoint

### Inconsistency in `CreateProjectMembership` Endpoint

- **Server-side** : The Redmine server's API supports creating *multiple*
  project memberships in a single request by accepting
  `params[:membership][:user_ids]` as an array of user IDs.
- **Client-side** : The Rust client's `CreateProjectMembership` struct is
  designed to create only a *single* membership at a time, accepting a single
  `user_id` .
- **Impact** : The Rust client cannot efficiently perform batch creation of
  project memberships.

---

## `queries` Endpoint

### Scope of Operations

- **Rust Client**: Only supports listing queries (`GET /queries.json`).
- **Redmine Server** : Supports full CRUD operations ( `new` , `create` , `edit`
  , `update` , `destroy` ) and a `filter` endpoint to retrieve available filter
  values.

### Query Fields and Data Structure

- **Rust Client's `Query` struct** : Incomplete, missing fields like
  `description` , `user_id` , `filters` , `column_names` , `sort_criteria` ,
  `options` , and the full `visibility` integer.
- **Redmine Server's `Query` model**: Provides a rich set of attributes.

### Visibility Representation

- **Rust Client**: Uses a boolean `is_public`.
- **Redmine Server** : Uses an integer `visibility` (0=private, 1=roles,
  2=public).

### Query Types and Polymorphism

- **Rust Client**: Uses a generic `Query` struct.
- **Redmine Server** : Employs polymorphism ( `IssueQuery` , `TimeEntryQuery` ,
  etc.).

### Filters and Parameters

- **Rust Client**: Does not expose or utilize the rich filtering capabilities.
- **Redmine Server**: Offers extensive filtering options.

### Associated Data

- **Redmine Server** : API response might include associated data like `roles`
  or `user` who created the query.
- **Rust Client**: These are not captured.

### Impact and Recommendations

The current Rust client for `queries` is very basic and only suitable for
retrieving a minimal set of information about public queries. To achieve feature
parity or enable more advanced interactions, the following changes are
necessary:

- **Expand `Query` struct:** Add fields like `description` , `user_id` ,
  `filters` (potentially as a more complex data structure or a raw JSON string),
  `column_names` , `sort_criteria` , `options` , and `visibility` (as an integer
  or an enum).
- **Handle `visibility`:** Update the Rust client to correctly interpret the
  integer `visibility` field from the server.
- **Implement CRUD operations:** Add endpoints and logic for creating, updating,
  and deleting queries.
- **Support query filtering:** Implement mechanisms to construct and send filter
  parameters to the Redmine API, and to parse the `available_filters_as_json`
  response from the server. This would involve a significant expansion of the
  client's capabilities.
- **Consider query types:** If the client needs to interact with specific query
  types (e.g., `IssueQuery` ), the `Query` struct might need to be made more
  flexible or polymorphic on the client side.
- **Parse associated data:** If `roles` or `user` details are needed, the client
  should be updated to parse them.

---

## `roles` Endpoint

### `ListRoles` Endpoint Data Structure Mismatch

- **Client Expectation (`RoleEssentials`):** `id`, `name`, optional `inherited`.
- **Server Response:** Full `Role` objects including `position` , `assignable` ,
  `builtin` , various `_visibility` fields, `permissions` , `settings` , etc.
- **Specific Issue: `inherited` field:** This field in `RoleEssentials` does not
  directly correspond to a Redmine `Role` model attribute or API response for
  `GET /roles.json` .

### `ListRoles` Endpoint Implicit Filtering

- **Client:** No parameters for filtering.
- **Server:** Implicitly filters to only return `givable` roles (non-built-in).
- **Inconsistency:** Client doesn't explicitly state this limitation or provide
  options to list all roles.

### `GetRole` Endpoint Data Structure Incompleteness (Client Side)

- **Client (`Role` struct):** Missing `position` , `builtin` , `settings` , and
  `default_time_entry_activity_id` .
- **Server:** Returns a full `Role` object.

---

## `trackers` Endpoint

### API Parameters and Filters

- **Inconsistency:** None. Both client and server implementations, as analyzed,
  fetch all trackers without specific filtering or pagination parameters.

### Returned Fields

- **Inconsistency:** Minor Type Mismatch for `enabled_standard_fields`.
  - The Redmine server consistently returns `enabled_standard_fields` as a JSON
    array (e.g., `[]` or `["field1", "field2"]` ).
  - The Rust client models this as `Option<Vec<String>>` . While `Option`
    correctly handles `null` values, the server's current implementation
    guarantees an array (even if empty), meaning `null` is not expected for this
    field.

  - **Impact:** This is not a breaking change and deserialization will still
    succeed (an empty array will be `Some([])` ). However, it introduces a
    slight semantic difference where the Rust type implies `null` is a
    possibility, which the server does not currently provide for this field.
  - **Potential Refinement (Client-side):** The Rust type could potentially be
    `Vec<String>` if it's guaranteed that the server will always return an array
    for `enabled_standard_fields` and never `null` . This would make the type
    more precise to the server's behavior.

---

## `uploads` Endpoint

### Missing `content_type` in upload request

- **Server** : The Redmine server's `attachments_controller#upload` expects a
  `content_type` parameter in the request.
- **Client** : The Rust client's `UploadFile` endpoint does not currently send
  this parameter.
- **Impact** : Redmine might guess the content type incorrectly, leading to
  issues.

### Extra `id` field in upload response

- **Server** : The Redmine server's JSON response for a successful upload
  includes both an `id` and a `token` within the `upload` object.
- **Client** : The Rust client's `FileUploadToken` struct only models the
  `token` field.
- **Impact** : While `serde` ignores unknown fields, it's a discrepancy in the
  data model. The `id` could be useful.

---

## `versions` Endpoint

### Missing Custom Fields in `Version` struct

- **Server** : The Redmine `Version` model supports custom fields (
  `acts_as_customizable` ).
- **Client** : The Rust client's `Version` struct does not include a field for
  custom fields.
- **Impact** : The client cannot retrieve or set custom field values for
  versions.

### Missing `close_completed` action

- **Server** : The `versions_controller.rb` has a `close_completed` action (
  `POST /versions/:id/close_completed.json` ) to close a version and move its
  open issues to the next open version.
- **Client**: The Rust client does not have an endpoint for this action.
- **Impact** : The client cannot programmatically trigger this useful Redmine
  feature.

### Missing `default_project_version` parameter

- **Server** : The Redmine `Version` model has a `default_project_version`
  method, and it's possible to set a version as default for a project.
- **Client** : The Rust client's `CreateVersion` and `UpdateVersion` endpoints
  do not expose a parameter to set a version as the default for a project.
- **Impact** : The client cannot create or update versions and set them as
  default for a project.

---

## `wiki_pages` Endpoint

### Missing `protected` field in `WikiPage` struct

- **Server**: The `WikiPage` model has a `protected` attribute (boolean).
- **Client** : The Rust client's `WikiPageEssentials` and `WikiPage` structs do
  not have a `protected` field.
- **Impact**: The client cannot determine if a wiki page is protected or not.

### No `redirect_existing_links` parameter in `CreateOrUpdateProjectWikiPage`

- **Server** : The `WikiPage` model has a `redirect_existing_links` attribute,
  used when renaming or moving a page.
- **Client** : The Rust client's `CreateOrUpdateProjectWikiPage` does not expose
  this parameter.
- **Impact** : The client cannot control whether existing links are redirected
  when renaming/moving a wiki page.

### Missing `is_start_page` parameter in `CreateOrUpdateProjectWikiPage`

- **Server**: The `WikiPage` model has an `is_start_page` attribute.
- **Client** : The Rust client's `CreateOrUpdateProjectWikiPage` does not expose
  this parameter.
- **Impact**: The client cannot set a wiki page as the start page for a project.

### Missing `destroy_version` action

- **Server** : The `wiki_controller.rb` has a `destroy_version` action (
  `DELETE /projects/:project_id/wiki/:id/:version/destroy_version` ).
- **Client**: The Rust client does not have an endpoint for this action.
- **Impact**: The client cannot delete specific versions of a wiki page.

### Missing `rename` action

- **Server** : The `wiki_controller.rb` has a `rename` action (
  `POST /projects/:project_id/wiki/:id/rename` ).
- **Client**: The Rust client does not have an endpoint for this action.
- **Impact**: The client cannot rename a wiki page.

### Missing `protect` action

- **Server** : The `wiki_controller.rb` has a `protect` action (
  `POST /projects/:project_id/wiki/:id/protect` ).
- **Client**: The Rust client does not have an endpoint for this action.
- **Impact**: The client cannot protect/unprotect a wiki page.

### Missing `history`, `diff`, `annotate` actions

- **Server** : The `wiki_controller.rb` has `history` , `diff` , `annotate`
  actions.
- **Client**: The Rust client does not have endpoints for these actions.
- **Impact** : The client cannot access the history, diff, or annotations of a
  wiki page.

### Missing `export` action

- **Server**: The `wiki_controller.rb` has an `export` action.
- **Client**: The Rust client does not have an endpoint for this action.
- **Impact**: The client cannot export wiki pages.

### Missing `add_attachment` action

- **Server**: The `wiki_controller.rb` has an `add_attachment` action.
- **Client**: The Rust client does not have an endpoint for this action.
- **Impact** : The client cannot add attachments to a wiki page directly via
  this action.

### `DeleteProjectWikiPage` `todo` and `reassign_to_id` parameters

- **Server** : The `destroy` action in `wiki_controller.rb` supports `todo` and
  `reassign_to_id` parameters when deleting a wiki page with descendants.
- **Client** : The Rust client's `DeleteProjectWikiPage` only accepts
  `project_id_or_name` and `title` .
- **Impact** : The client cannot control how descendant pages are handled when
  deleting a parent wiki page.
