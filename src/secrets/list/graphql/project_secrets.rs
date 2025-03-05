#![allow(clippy::all, warnings)]
pub struct ProjectSecrets;
pub mod project_secrets {
    #![allow(dead_code)]
    use std::result::Result;
    use std::fmt;
    pub const OPERATION_NAME: &str = "ProjectSecrets";
    pub const QUERY : & str = "query ProjectSecrets($id: uuid!) {\n  project_by_pk(id: $id) {\n    id\n    name\n\n    userSecrets(limit: 1000) {\n      id\n      name\n      slug\n      updatedAt\n      vendor\n    }\n  }\n}\n" ;
    use super::*;
    use ::uuid::Uuid;
    use chrono::offset::Utc;
    use chrono::DateTime;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    type timestamptz = DateTime<Utc>;
    type uuid = Uuid;
    #[derive()]
    pub enum vendorEnum_enum {
        agora,
        ansible,
        auth0,
        aws,
        azure,
        bitbucket,
        braintree,
        cloudflare,
        datadog,
        digitalOcean,
        discord,
        docker,
        dropbox,
        facebook,
        figma,
        gemini,
        gitHub,
        gitLab,
        google,
        googleCloud,
        jenkins,
        jira,
        kubernetes,
        linear,
        linkedIn,
        mailchimp,
        mixpanel,
        netlify,
        openAI,
        other,
        paypal,
        pulumi,
        reddit,
        salesforce,
        segment,
        sendgrid,
        shopify,
        slack,
        stripe,
        terraform,
        trello,
        twilio,
        twitter,
        vercel,
        zoom,
        Other(String),
    }
    impl fmt::Display for vendorEnum_enum {
        fn fmt(&self, value: &mut fmt::Formatter) -> fmt::Result {
            match self {
                vendorEnum_enum::agora => write!(value, "Agora"),
                vendorEnum_enum::ansible => write!(value, "Ansible"),
                vendorEnum_enum::auth0 => write!(value, "Auth0"),
                vendorEnum_enum::aws => write!(value, "AWS"),
                vendorEnum_enum::azure => write!(value, "Azure"),
                vendorEnum_enum::bitbucket => write!(value, "Bitbucket"),
                vendorEnum_enum::braintree => write!(value, "Braintree"),
                vendorEnum_enum::cloudflare => write!(value, "Cloudflare"),
                vendorEnum_enum::datadog => write!(value, "Datadog"),
                vendorEnum_enum::digitalOcean => write!(value, "DigitalOcean"),
                vendorEnum_enum::discord => write!(value, "Discord"),
                vendorEnum_enum::docker => write!(value, "Docker"),
                vendorEnum_enum::dropbox => write!(value, "Dropbox"),
                vendorEnum_enum::facebook => write!(value, "Facebook"),
                vendorEnum_enum::figma => write!(value, "Figma"),
                vendorEnum_enum::gemini => write!(value, "Gemini"),
                vendorEnum_enum::gitHub => write!(value, "GitHub"),
                vendorEnum_enum::gitLab => write!(value, "GitLab"),
                vendorEnum_enum::google => write!(value, "Google"),
                vendorEnum_enum::googleCloud => write!(value, "GoogleCloud"),
                vendorEnum_enum::jenkins => write!(value, "Jenkins"),
                vendorEnum_enum::jira => write!(value, "Jira"),
                vendorEnum_enum::kubernetes => write!(value, "Kubernetes"),
                vendorEnum_enum::linear => write!(value, "Linear"),
                vendorEnum_enum::linkedIn => write!(value, "LinkedIn"),
                vendorEnum_enum::mailchimp => write!(value, "Mailchimp"),
                vendorEnum_enum::mixpanel => write!(value, "Mixpanel"),
                vendorEnum_enum::netlify => write!(value, "Netlify"),
                vendorEnum_enum::openAI => write!(value, "OpenAI"),
                vendorEnum_enum::other => write!(value, "Other"),
                vendorEnum_enum::other => write!(value, "Other"),
                vendorEnum_enum::Other(other) => write!(value, "{}", other),
                vendorEnum_enum::paypal => write!(value, "Paypal"),
                vendorEnum_enum::paypal => write!(value, "Paypal"),
                vendorEnum_enum::pulumi => write!(value, "Pulumi"),
                vendorEnum_enum::pulumi => write!(value, "Pulumi"),
                vendorEnum_enum::reddit => write!(value, "Reddit"),
                vendorEnum_enum::salesforce => write!(value, "Salesforce"),
                vendorEnum_enum::salesforce => write!(value, "Salesforce"),
                vendorEnum_enum::segment => write!(value, "Segment"),
                vendorEnum_enum::sendgrid => write!(value, "Sendgrid"),
                vendorEnum_enum::shopify => write!(value, "Shopify"),
                vendorEnum_enum::slack => write!(value, "Slack"),
                vendorEnum_enum::stripe => write!(value, "Stripe"),
                vendorEnum_enum::terraform => write!(value, "Terraform"),
                vendorEnum_enum::trello => write!(value, "Trello"),
                vendorEnum_enum::twilio => write!(value, "Twilio"),
                vendorEnum_enum::twitter => write!(value, "Twitter"),
                vendorEnum_enum::vercel => write!(value, "Vercel"),
                vendorEnum_enum::zoom => write!(value, "Zoom"),
            }
        }
    }
    impl ::serde::Serialize for vendorEnum_enum {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                vendorEnum_enum::agora => "agora",
                vendorEnum_enum::ansible => "ansible",
                vendorEnum_enum::auth0 => "auth0",
                vendorEnum_enum::aws => "aws",
                vendorEnum_enum::azure => "azure",
                vendorEnum_enum::bitbucket => "bitbucket",
                vendorEnum_enum::braintree => "braintree",
                vendorEnum_enum::cloudflare => "cloudflare",
                vendorEnum_enum::datadog => "datadog",
                vendorEnum_enum::digitalOcean => "digitalOcean",
                vendorEnum_enum::discord => "discord",
                vendorEnum_enum::docker => "docker",
                vendorEnum_enum::dropbox => "dropbox",
                vendorEnum_enum::facebook => "facebook",
                vendorEnum_enum::figma => "figma",
                vendorEnum_enum::gemini => "gemini",
                vendorEnum_enum::gitHub => "gitHub",
                vendorEnum_enum::gitLab => "gitLab",
                vendorEnum_enum::google => "google",
                vendorEnum_enum::googleCloud => "googleCloud",
                vendorEnum_enum::jenkins => "jenkins",
                vendorEnum_enum::jira => "jira",
                vendorEnum_enum::kubernetes => "kubernetes",
                vendorEnum_enum::linear => "linear",
                vendorEnum_enum::linkedIn => "linkedIn",
                vendorEnum_enum::mailchimp => "mailchimp",
                vendorEnum_enum::mixpanel => "mixpanel",
                vendorEnum_enum::netlify => "netlify",
                vendorEnum_enum::openAI => "openAI",
                vendorEnum_enum::other => "other",
                vendorEnum_enum::Other(ref s) => &s,
                vendorEnum_enum::paypal => "paypal",
                vendorEnum_enum::pulumi => "pulumi",
                vendorEnum_enum::reddit => "reddit",
                vendorEnum_enum::salesforce => "salesforce",
                vendorEnum_enum::segment => "segment",
                vendorEnum_enum::sendgrid => "sendgrid",
                vendorEnum_enum::shopify => "shopify",
                vendorEnum_enum::slack => "slack",
                vendorEnum_enum::stripe => "stripe",
                vendorEnum_enum::terraform => "terraform",
                vendorEnum_enum::trello => "trello",
                vendorEnum_enum::twilio => "twilio",
                vendorEnum_enum::twitter => "twitter",
                vendorEnum_enum::vercel => "vercel",
                vendorEnum_enum::zoom => "zoom",
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for vendorEnum_enum {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s: String = ::serde::Deserialize::deserialize(deserializer)?;
            match s.as_str() {
                "agora" => Ok(vendorEnum_enum::agora),
                "ansible" => Ok(vendorEnum_enum::ansible),
                "auth0" => Ok(vendorEnum_enum::auth0),
                "aws" => Ok(vendorEnum_enum::aws),
                "azure" => Ok(vendorEnum_enum::azure),
                "bitbucket" => Ok(vendorEnum_enum::bitbucket),
                "braintree" => Ok(vendorEnum_enum::braintree),
                "cloudflare" => Ok(vendorEnum_enum::cloudflare),
                "datadog" => Ok(vendorEnum_enum::datadog),
                "digitalOcean" => Ok(vendorEnum_enum::digitalOcean),
                "discord" => Ok(vendorEnum_enum::discord),
                "docker" => Ok(vendorEnum_enum::docker),
                "dropbox" => Ok(vendorEnum_enum::dropbox),
                "facebook" => Ok(vendorEnum_enum::facebook),
                "figma" => Ok(vendorEnum_enum::figma),
                "gemini" => Ok(vendorEnum_enum::gemini),
                "gitHub" => Ok(vendorEnum_enum::gitHub),
                "gitLab" => Ok(vendorEnum_enum::gitLab),
                "google" => Ok(vendorEnum_enum::google),
                "googleCloud" => Ok(vendorEnum_enum::googleCloud),
                "jenkins" => Ok(vendorEnum_enum::jenkins),
                "jira" => Ok(vendorEnum_enum::jira),
                "kubernetes" => Ok(vendorEnum_enum::kubernetes),
                "linear" => Ok(vendorEnum_enum::linear),
                "linkedIn" => Ok(vendorEnum_enum::linkedIn),
                "mailchimp" => Ok(vendorEnum_enum::mailchimp),
                "mixpanel" => Ok(vendorEnum_enum::mixpanel),
                "netlify" => Ok(vendorEnum_enum::netlify),
                "openAI" => Ok(vendorEnum_enum::openAI),
                "other" => Ok(vendorEnum_enum::other),
                "paypal" => Ok(vendorEnum_enum::paypal),
                "pulumi" => Ok(vendorEnum_enum::pulumi),
                "reddit" => Ok(vendorEnum_enum::reddit),
                "salesforce" => Ok(vendorEnum_enum::salesforce),
                "segment" => Ok(vendorEnum_enum::segment),
                "sendgrid" => Ok(vendorEnum_enum::sendgrid),
                "shopify" => Ok(vendorEnum_enum::shopify),
                "slack" => Ok(vendorEnum_enum::slack),
                "stripe" => Ok(vendorEnum_enum::stripe),
                "terraform" => Ok(vendorEnum_enum::terraform),
                "trello" => Ok(vendorEnum_enum::trello),
                "twilio" => Ok(vendorEnum_enum::twilio),
                "twitter" => Ok(vendorEnum_enum::twitter),
                "vercel" => Ok(vendorEnum_enum::vercel),
                "zoom" => Ok(vendorEnum_enum::zoom),
                _ => Ok(vendorEnum_enum::Other(s)),
            }
        }
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub id: uuid,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub project_by_pk: Option<ProjectSecretsProjectByPk>,
    }
    #[derive(Deserialize)]
    pub struct ProjectSecretsProjectByPk {
        pub id: uuid,
        pub name: String,
        #[serde(rename = "userSecrets")]
        pub user_secrets: Vec<ProjectSecretsProjectByPkUserSecrets>,
    }
    #[derive(Deserialize)]
    pub struct ProjectSecretsProjectByPkUserSecrets {
        pub id: uuid,
        pub name: String,
        pub slug: String,
        #[serde(rename = "updatedAt")]
        pub updated_at: timestamptz,
        pub vendor: vendorEnum_enum,
    }
}
impl graphql_client::GraphQLQuery for ProjectSecrets {
    type Variables = project_secrets::Variables;
    type ResponseData = project_secrets::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: project_secrets::QUERY,
            operation_name: project_secrets::OPERATION_NAME,
        }
    }
}
