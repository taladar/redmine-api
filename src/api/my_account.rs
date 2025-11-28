//! My Account Rest API Endpoint definitions
//!
//! [Redmine Documentation](https://www.redmine.org/projects/redmine/wiki/Rest_MyAccount)
//!
//! - [x] my account endpoint

use derive_builder::Builder;
use reqwest::Method;
use std::borrow::Cow;

use crate::api::custom_fields::CustomFieldEssentialsWithValue;
use crate::api::{Endpoint, NoPagination, ReturnsJsonResponse};
use serde_json::json;

/// a type for my account to use as an API return type
///
/// alternatively you can use your own type limited to the fields you need
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MyAccount {
    /// numeric id
    pub id: u64,
    /// login name
    pub login: String,
    /// is this user an admin
    pub admin: bool,
    /// user's firstname
    pub firstname: String,
    /// user's lastname
    pub lastname: String,
    /// primary email of the user
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mail: Option<String>,
    /// The time when this user was created
    #[serde(
        serialize_with = "crate::api::serialize_rfc3339",
        deserialize_with = "crate::api::deserialize_rfc3339"
    )]
    pub created_on: time::OffsetDateTime,
    /// the time when this user last logged in
    #[serde(
        serialize_with = "crate::api::serialize_optional_rfc3339",
        deserialize_with = "crate::api::deserialize_optional_rfc3339"
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_login_on: Option<time::OffsetDateTime>,
    /// the user's API key
    pub api_key: String,
    /// two-factor authentication scheme
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub twofa_scheme: Option<String>,
    /// authentication source id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_source_id: Option<u64>,
    /// whether the user must change password
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub must_change_passwd: Option<bool>,
    /// the time when the password was last changed
    #[serde(
        default,
        serialize_with = "crate::api::serialize_optional_rfc3339",
        deserialize_with = "crate::api::deserialize_optional_rfc3339"
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passwd_changed_on: Option<time::OffsetDateTime>,
    /// custom fields with values
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldEssentialsWithValue>>,
}

/// Mail notification options for a user.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MailNotificationOption {
    /// All events.
    All,
    /// Only for selected projects.
    Selected,
    /// Only for my events.
    OnlyMyEvents,
    /// Only for assigned issues.
    OnlyAssigned,
    /// Only for issues I own.
    OnlyOwner,
    /// No notifications.
    None,
}

/// Comments sorting order.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommentsSorting {
    /// Ascending order.
    Asc,
    /// Descending order.
    Desc,
}

/// Textarea font options.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TextareaFont {
    /// Monospace font.
    Monospace,
    /// Proportional font.
    Proportional,
}

/// Toolbar language options.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolbarLanguage {
    /// C programming language.
    C,
    /// C++ programming language.
    Cpp,
    /// C# programming language.
    Csharp,
    /// CSS stylesheet language.
    Css,
    /// Diff format.
    Diff,
    /// Go programming language.
    Go,
    /// Groovy programming language.
    Groovy,
    /// HTML markup language.
    Html,
    /// Java programming language.
    Java,
    /// Javascript programming language.
    Javascript,
    /// Objective-C programming language.
    Objc,
    /// Perl programming language.
    Perl,
    /// PHP programming language.
    Php,
    /// Python programming language.
    Python,
    /// R programming language.
    R,
    /// Ruby programming language.
    Ruby,
    /// Sass stylesheet language.
    Sass,
    /// Scala programming language.
    Scala,
    /// Shell scripting language.
    Shell,
    /// SQL query language.
    Sql,
    /// Swift programming language.
    Swift,
    /// XML markup language.
    Xml,
    /// YAML data serialization language.
    Yaml,
}

impl std::fmt::Display for ToolbarLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToolbarLanguage::C => write!(f, "c"),
            ToolbarLanguage::Cpp => write!(f, "cpp"),
            ToolbarLanguage::Csharp => write!(f, "csharp"),
            ToolbarLanguage::Css => write!(f, "css"),
            ToolbarLanguage::Diff => write!(f, "diff"),
            ToolbarLanguage::Go => write!(f, "go"),
            ToolbarLanguage::Groovy => write!(f, "groovy"),
            ToolbarLanguage::Html => write!(f, "html"),
            ToolbarLanguage::Java => write!(f, "java"),
            ToolbarLanguage::Javascript => write!(f, "javascript"),
            ToolbarLanguage::Objc => write!(f, "objc"),
            ToolbarLanguage::Perl => write!(f, "perl"),
            ToolbarLanguage::Php => write!(f, "php"),
            ToolbarLanguage::Python => write!(f, "python"),
            ToolbarLanguage::R => write!(f, "r"),
            ToolbarLanguage::Ruby => write!(f, "ruby"),
            ToolbarLanguage::Sass => write!(f, "sass"),
            ToolbarLanguage::Scala => write!(f, "scala"),
            ToolbarLanguage::Shell => write!(f, "shell"),
            ToolbarLanguage::Sql => write!(f, "sql"),
            ToolbarLanguage::Swift => write!(f, "swift"),
            ToolbarLanguage::Xml => write!(f, "xml"),
            ToolbarLanguage::Yaml => write!(f, "yaml"),
        }
    }
}

/// Auto watch on actions.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutoWatchAction {
    /// Watch issues when created.
    IssueCreated,
    /// Watch issues when contributed to.
    IssueContributedTo,
}

impl std::fmt::Display for AutoWatchAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AutoWatchAction::IssueCreated => write!(f, "issue_created"),
            AutoWatchAction::IssueContributedTo => write!(f, "issue_contributed_to"),
        }
    }
}

/// The endpoint to retrieve the current user's my account settings/data
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct GetMyAccount {}

impl ReturnsJsonResponse for GetMyAccount {}
impl NoPagination for GetMyAccount {}

impl GetMyAccount {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> GetMyAccountBuilder {
        GetMyAccountBuilder::default()
    }
}

impl Endpoint for GetMyAccount {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "my/account.json".into()
    }
}

/// The endpoint to update the current user's my account settings/data
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Builder, serde::Serialize)]
#[builder(setter(strip_option))]
pub struct UpdateMyAccount<'a> {
    /// user's firstname
    #[builder(setter(into), default)]
    firstname: Option<Cow<'a, str>>,
    /// user's lastname
    #[builder(setter(into), default)]
    lastname: Option<Cow<'a, str>>,
    /// primary email of the user
    #[builder(setter(into), default)]
    mail: Option<Cow<'a, str>>,
    /// mail notification option
    #[builder(default)]
    mail_notification: Option<MailNotificationOption>,
    /// project ids for which the user has explicitly turned mail notifications on
    #[builder(default)]
    notified_project_ids: Option<Vec<u64>>,
    /// user's language
    #[builder(setter(into), default)]
    language: Option<Cow<'a, str>>,
    /// hide mail address
    #[builder(default)]
    hide_mail: Option<bool>,
    /// user's time zone
    #[builder(setter(into), default)]
    time_zone: Option<Cow<'a, str>>,
    /// comments sorting order ('asc' or 'desc')
    #[builder(default)]
    comments_sorting: Option<CommentsSorting>,
    /// warn on leaving unsaved changes
    #[builder(default)]
    warn_on_leaving_unsaved: Option<bool>,
    /// no self notified
    #[builder(default)]
    no_self_notified: Option<bool>,
    /// notify about high priority issues
    #[builder(default)]
    notify_about_high_priority_issues: Option<bool>,
    /// textarea font ('monospace' or 'proportional')
    #[builder(default)]
    textarea_font: Option<TextareaFont>,
    /// recently used projects
    #[builder(default)]
    recently_used_projects: Option<u64>,
    /// history default tab
    #[builder(setter(into), default)]
    history_default_tab: Option<Cow<'a, str>>,
    /// default issue query
    #[builder(setter(into), default)]
    default_issue_query: Option<Cow<'a, str>>,
    /// default project query
    #[builder(setter(into), default)]
    default_project_query: Option<Cow<'a, str>>,
    /// toolbar language options (comma-separated list of languages)
    #[builder(default)]
    toolbar_language_options: Option<Vec<ToolbarLanguage>>,
    /// auto watch on (comma-separated list of actions)
    #[builder(default)]
    auto_watch_on: Option<Vec<AutoWatchAction>>,
}

impl<'a> UpdateMyAccount<'a> {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> UpdateMyAccountBuilder<'a> {
        UpdateMyAccountBuilder::default()
    }
}

impl Endpoint for UpdateMyAccount<'_> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "my/account.json".into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::Error> {
        use serde_json::json;
        let mut user_params = serde_json::Map::new();
        if let Some(ref firstname) = self.firstname {
            user_params.insert("firstname".to_string(), json!(firstname));
        }
        if let Some(ref lastname) = self.lastname {
            user_params.insert("lastname".to_string(), json!(lastname));
        }
        if let Some(ref mail) = self.mail {
            user_params.insert("mail".to_string(), json!(mail));
        }
        if let Some(ref mail_notification) = self.mail_notification {
            user_params.insert("mail_notification".to_string(), json!(mail_notification));
        }
        if let Some(ref notified_project_ids) = self.notified_project_ids {
            user_params.insert(
                "notified_project_ids".to_string(),
                json!(notified_project_ids),
            );
        }
        if let Some(ref language) = self.language {
            user_params.insert("language".to_string(), json!(language));
        }

        let mut pref_params = serde_json::Map::new();
        if let Some(hide_mail) = self.hide_mail {
            pref_params.insert("hide_mail".to_string(), json!(hide_mail));
        }
        if let Some(ref time_zone) = self.time_zone {
            pref_params.insert("time_zone".to_string(), json!(time_zone));
        }
        if let Some(ref comments_sorting) = self.comments_sorting {
            pref_params.insert("comments_sorting".to_string(), json!(comments_sorting));
        }
        if let Some(warn_on_leaving_unsaved) = self.warn_on_leaving_unsaved {
            pref_params.insert(
                "warn_on_leaving_unsaved".to_string(),
                json!(warn_on_leaving_unsaved),
            );
        }
        if let Some(no_self_notified) = self.no_self_notified {
            pref_params.insert("no_self_notified".to_string(), json!(no_self_notified));
        }
        if let Some(notify_about_high_priority_issues) = self.notify_about_high_priority_issues {
            pref_params.insert(
                "notify_about_high_priority_issues".to_string(),
                json!(notify_about_high_priority_issues),
            );
        }
        if let Some(ref textarea_font) = self.textarea_font {
            pref_params.insert("textarea_font".to_string(), json!(textarea_font));
        }
        if let Some(recently_used_projects) = self.recently_used_projects {
            pref_params.insert(
                "recently_used_projects".to_string(),
                json!(recently_used_projects),
            );
        }
        if let Some(ref history_default_tab) = self.history_default_tab {
            pref_params.insert(
                "history_default_tab".to_string(),
                json!(history_default_tab),
            );
        }
        if let Some(ref default_issue_query) = self.default_issue_query {
            pref_params.insert(
                "default_issue_query".to_string(),
                json!(default_issue_query),
            );
        }
        if let Some(ref default_project_query) = self.default_project_query {
            pref_params.insert(
                "default_project_query".to_string(),
                json!(default_project_query),
            );
        }
        if let Some(ref toolbar_language_options) = self.toolbar_language_options {
            pref_params.insert(
                "toolbar_language_options".to_string(),
                json!(
                    toolbar_language_options
                        .iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<String>>()
                        .join(",")
                ),
            );
        }
        if let Some(ref auto_watch_on) = self.auto_watch_on {
            pref_params.insert(
                "auto_watch_on".to_string(),
                json!(
                    auto_watch_on
                        .iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<String>>()
                        .join(",")
                ),
            );
        }

        let mut root_map = serde_json::Map::new();
        if !user_params.is_empty() {
            root_map.insert("user".to_string(), serde_json::Value::Object(user_params));
        }
        if !pref_params.is_empty() {
            root_map.insert("pref".to_string(), serde_json::Value::Object(pref_params));
        }

        if root_map.is_empty() {
            Ok(None)
        } else {
            Ok(Some((
                "application/json",
                serde_json::to_vec(&serde_json::Value::Object(root_map))?,
            )))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::api::test_helpers::with_redmine;
    use crate::api::users::UserWrapper;
    use pretty_assertions::assert_eq;
    use std::error::Error;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_get_my_account() -> Result<(), Box<dyn Error>> {
        let current_span = tracing::Span::current();
        with_redmine(current_span, |redmine| {
            let endpoint = GetMyAccount::builder().build()?;
            redmine.json_response_body::<_, UserWrapper<MyAccount>>(&endpoint)?;
            Ok(())
        })
    }

    #[traced_test]
    #[test]
    fn test_update_my_account() -> Result<(), Box<dyn Error>> {
        let current_span = tracing::Span::current();
        with_redmine(current_span, |redmine| {
            let get_endpoint = GetMyAccount::builder().build()?;
            let original_account: UserWrapper<MyAccount> =
                redmine.json_response_body(&get_endpoint)?;
            let update_endpoint = UpdateMyAccount::builder()
                .firstname("NewFirstName")
                .build()?;
            redmine.ignore_response_body(&update_endpoint)?;
            let updated_account: UserWrapper<MyAccount> =
                redmine.json_response_body(&get_endpoint)?;
            assert_eq!(updated_account.user.firstname, "NewFirstName");
            let restore_endpoint = UpdateMyAccount::builder()
                .firstname(original_account.user.firstname.as_str())
                .build()?;
            redmine.ignore_response_body(&restore_endpoint)?;
            let restored_account: UserWrapper<MyAccount> =
                redmine.json_response_body(&get_endpoint)?;
            assert_eq!(
                restored_account.user.firstname,
                original_account.user.firstname
            );
            Ok(())
        })
    }

    /// this tests if any of the results contain a field we are not deserializing
    ///
    /// this will only catch fields we missed if they are part of the response but
    /// it is better than nothing
    #[traced_test]
    #[test]
    fn test_completeness_my_account_type() -> Result<(), Box<dyn Error>> {
        let current_span = tracing::Span::current();
        with_redmine(current_span, |redmine| {
            let endpoint = GetMyAccount::builder().build()?;
            let UserWrapper { user: value } =
                redmine.json_response_body::<_, UserWrapper<serde_json::Value>>(&endpoint)?;
            let o: MyAccount = serde_json::from_value(value.clone())?;
            let reserialized = serde_json::to_value(o)?;
            assert_eq!(value, reserialized);
            Ok(())
        })
    }
}
