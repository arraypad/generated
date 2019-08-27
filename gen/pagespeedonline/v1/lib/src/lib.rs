pub mod schemas {
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Result {
        #[doc = "The captcha verify result"]
        #[serde(rename = "captchaResult", default)]
        pub captcha_result: ::std::option::Option<String>,
        #[doc = "Localized PageSpeed results. Contains a ruleResults entry for each PageSpeed rule instantiated and run by the server."]
        #[serde(rename = "formattedResults", default)]
        pub formatted_results: ::std::option::Option<crate::schemas::ResultFormattedResults>,
        #[doc = "Canonicalized and final URL for the document, after following page redirects (if any)."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[doc = "List of rules that were specified in the request, but which the server did not know how to instantiate."]
        #[serde(rename = "invalidRules", default)]
        pub invalid_rules: ::std::option::Option<Vec<String>>,
        #[doc = "Kind of result."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "Summary statistics for the page, such as number of JavaScript bytes, number of HTML bytes, etc."]
        #[serde(rename = "pageStats", default)]
        pub page_stats: ::std::option::Option<crate::schemas::ResultPageStats>,
        #[doc = "Response code for the document. 200 indicates a normal page load. 4xx/5xx indicates an error."]
        #[serde(rename = "responseCode", default)]
        pub response_code: ::std::option::Option<i32>,
        #[doc = "The PageSpeed Score (0-100), which indicates how much faster a page could be. A high score indicates little room for improvement, while a lower score indicates more room for improvement."]
        #[serde(rename = "score", default)]
        pub score: ::std::option::Option<i32>,
        #[doc = "Base64-encoded screenshot of the page that was analyzed."]
        #[serde(rename = "screenshot", default)]
        pub screenshot: ::std::option::Option<crate::schemas::ResultScreenshot>,
        #[doc = "Title of the page, as displayed in the browser's title bar."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
        #[doc = "The version of PageSpeed used to generate these results."]
        #[serde(rename = "version", default)]
        pub version: ::std::option::Option<crate::schemas::ResultVersion>,
    }
    impl ::field_selector::FieldSelector for Result {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ResultFormattedResults {
        #[doc = "The locale of the formattedResults, e.g. \"en_US\"."]
        #[serde(rename = "locale", default)]
        pub locale: ::std::option::Option<String>,
        #[doc = "Dictionary of formatted rule results, with one entry for each PageSpeed rule instantiated and run by the server."]
        #[serde(rename = "ruleResults", default)]
        pub rule_results: ::std::option::Option<
            ::std::collections::BTreeMap<
                String,
                crate::schemas::ResultFormattedResultsRuleResultsAdditionalProperties,
            >,
        >,
    }
    impl ::field_selector::FieldSelector for ResultFormattedResults {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ResultFormattedResultsRuleResultsAdditionalProperties {
        #[doc = "Localized name of the rule, intended for presentation to a user."]
        #[serde(rename = "localizedRuleName", default)]
        pub localized_rule_name: ::std::option::Option<String>,
        #[doc = "The impact (unbounded floating point value) that implementing the suggestions for this rule would have on making the page faster. Impact is comparable between rules to determine which rule's suggestions would have a higher or lower impact on making a page faster. For instance, if enabling compression would save 1MB, while optimizing images would save 500kB, the enable compression rule would have 2x the impact of the image optimization rule, all other things being equal."]
        #[serde(rename = "ruleImpact", default)]
        pub rule_impact: ::std::option::Option<f64>,
        #[doc = "List of blocks of URLs. Each block may contain a heading and a list of URLs. Each URL may optionally include additional details."]
        #[serde(rename = "urlBlocks", default)]
        pub url_blocks: ::std::option::Option<
            Vec<
                crate::schemas::ResultFormattedResultsRuleResultsAdditionalPropertiesUrlBlocksItems,
            >,
        >,
    }
    impl ::field_selector::FieldSelector for ResultFormattedResultsRuleResultsAdditionalProperties {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ResultFormattedResultsRuleResultsAdditionalPropertiesUrlBlocksItems { # [ doc = "Heading to be displayed with the list of URLs." ] # [ serde ( rename = "header" , default ) ] pub header : :: std :: option :: Option < crate :: schemas :: ResultFormattedResultsRuleResultsAdditionalPropertiesUrlBlocksItemsHeader > , # [ doc = "List of entries that provide information about URLs in the url block. Optional." ] # [ serde ( rename = "urls" , default ) ] pub urls : :: std :: option :: Option < Vec < crate :: schemas :: ResultFormattedResultsRuleResultsAdditionalPropertiesUrlBlocksItemsUrlsItems > > , }
    impl ::field_selector::FieldSelector
        for ResultFormattedResultsRuleResultsAdditionalPropertiesUrlBlocksItems
    {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ResultFormattedResultsRuleResultsAdditionalPropertiesUrlBlocksItemsHeader { # [ doc = "List of arguments for the format string." ] # [ serde ( rename = "args" , default ) ] pub args : :: std :: option :: Option < Vec < crate :: schemas :: ResultFormattedResultsRuleResultsAdditionalPropertiesUrlBlocksItemsHeaderArgsItems > > , # [ doc = "A localized format string with $N placeholders, where N is the 1-indexed argument number, e.g. 'Minifying the following $1 resources would save a total of $2 bytes'." ] # [ serde ( rename = "format" , default ) ] pub format : :: std :: option :: Option < String > , }
    impl ::field_selector::FieldSelector
        for ResultFormattedResultsRuleResultsAdditionalPropertiesUrlBlocksItemsHeader
    {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ResultFormattedResultsRuleResultsAdditionalPropertiesUrlBlocksItemsHeaderArgsItems {
        #[doc = "Type of argument. One of URL, STRING_LITERAL, INT_LITERAL, BYTES, or DURATION."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<String>,
        #[doc = "Argument value, as a localized string."]
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector
        for ResultFormattedResultsRuleResultsAdditionalPropertiesUrlBlocksItemsHeaderArgsItems
    {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ResultFormattedResultsRuleResultsAdditionalPropertiesUrlBlocksItemsUrlsItems { # [ doc = "List of entries that provide additional details about a single URL. Optional." ] # [ serde ( rename = "details" , default ) ] pub details : :: std :: option :: Option < Vec < crate :: schemas :: ResultFormattedResultsRuleResultsAdditionalPropertiesUrlBlocksItemsUrlsItemsDetailsItems > > , # [ doc = "A format string that gives information about the URL, and a list of arguments for that format string." ] # [ serde ( rename = "result" , default ) ] pub result : :: std :: option :: Option < crate :: schemas :: ResultFormattedResultsRuleResultsAdditionalPropertiesUrlBlocksItemsUrlsItemsResult > , }
    impl ::field_selector::FieldSelector
        for ResultFormattedResultsRuleResultsAdditionalPropertiesUrlBlocksItemsUrlsItems
    {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ResultFormattedResultsRuleResultsAdditionalPropertiesUrlBlocksItemsUrlsItemsDetailsItems { # [ doc = "List of arguments for the format string." ] # [ serde ( rename = "args" , default ) ] pub args : :: std :: option :: Option < Vec < crate :: schemas :: ResultFormattedResultsRuleResultsAdditionalPropertiesUrlBlocksItemsUrlsItemsDetailsItemsArgsItems > > , # [ doc = "A localized format string with $N placeholders, where N is the 1-indexed argument number, e.g. 'Unnecessary metadata for this resource adds an additional $1 bytes to its download size'." ] # [ serde ( rename = "format" , default ) ] pub format : :: std :: option :: Option < String > , }
    impl ::field_selector::FieldSelector
        for ResultFormattedResultsRuleResultsAdditionalPropertiesUrlBlocksItemsUrlsItemsDetailsItems
    {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ResultFormattedResultsRuleResultsAdditionalPropertiesUrlBlocksItemsUrlsItemsDetailsItemsArgsItems
    {
        #[doc = "Type of argument. One of URL, STRING_LITERAL, INT_LITERAL, BYTES, or DURATION."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<String>,
        #[doc = "Argument value, as a localized string."]
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<String>,
    }
    impl :: field_selector :: FieldSelector for ResultFormattedResultsRuleResultsAdditionalPropertiesUrlBlocksItemsUrlsItemsDetailsItemsArgsItems { fn field_selector_with_ident ( ident : & str , selector : & mut String ) { match selector . chars ( ) . rev ( ) . nth ( 0 ) { Some ( ',' ) | None => { } , _ => selector . push_str ( "," ) , } selector . push_str ( ident ) ; } }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ResultFormattedResultsRuleResultsAdditionalPropertiesUrlBlocksItemsUrlsItemsResult { # [ doc = "List of arguments for the format string." ] # [ serde ( rename = "args" , default ) ] pub args : :: std :: option :: Option < Vec < crate :: schemas :: ResultFormattedResultsRuleResultsAdditionalPropertiesUrlBlocksItemsUrlsItemsResultArgsItems > > , # [ doc = "A localized format string with $N placeholders, where N is the 1-indexed argument number, e.g. 'Minifying the resource at URL $1 can save $2 bytes'." ] # [ serde ( rename = "format" , default ) ] pub format : :: std :: option :: Option < String > , }
    impl ::field_selector::FieldSelector
        for ResultFormattedResultsRuleResultsAdditionalPropertiesUrlBlocksItemsUrlsItemsResult
    {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ResultFormattedResultsRuleResultsAdditionalPropertiesUrlBlocksItemsUrlsItemsResultArgsItems
    {
        #[doc = "Type of argument. One of URL, STRING_LITERAL, INT_LITERAL, BYTES, or DURATION."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<String>,
        #[doc = "Argument value, as a localized string."]
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<String>,
    }
    impl :: field_selector :: FieldSelector for ResultFormattedResultsRuleResultsAdditionalPropertiesUrlBlocksItemsUrlsItemsResultArgsItems { fn field_selector_with_ident ( ident : & str , selector : & mut String ) { match selector . chars ( ) . rev ( ) . nth ( 0 ) { Some ( ',' ) | None => { } , _ => selector . push_str ( "," ) , } selector . push_str ( ident ) ; } }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ResultPageStats {
        #[doc = "Number of uncompressed response bytes for CSS resources on the page."]
        #[serde(rename = "cssResponseBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub css_response_bytes: ::std::option::Option<i64>,
        #[doc = "Number of response bytes for flash resources on the page."]
        #[serde(rename = "flashResponseBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub flash_response_bytes: ::std::option::Option<i64>,
        #[doc = "Number of uncompressed response bytes for the main HTML document and all iframes on the page."]
        #[serde(rename = "htmlResponseBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub html_response_bytes: ::std::option::Option<i64>,
        #[doc = "Number of response bytes for image resources on the page."]
        #[serde(rename = "imageResponseBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub image_response_bytes: ::std::option::Option<i64>,
        #[doc = "Number of uncompressed response bytes for JS resources on the page."]
        #[serde(rename = "javascriptResponseBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub javascript_response_bytes: ::std::option::Option<i64>,
        #[doc = "Number of CSS resources referenced by the page."]
        #[serde(rename = "numberCssResources", default)]
        pub number_css_resources: ::std::option::Option<i32>,
        #[doc = "Number of unique hosts referenced by the page."]
        #[serde(rename = "numberHosts", default)]
        pub number_hosts: ::std::option::Option<i32>,
        #[doc = "Number of JavaScript resources referenced by the page."]
        #[serde(rename = "numberJsResources", default)]
        pub number_js_resources: ::std::option::Option<i32>,
        #[doc = "Number of HTTP resources loaded by the page."]
        #[serde(rename = "numberResources", default)]
        pub number_resources: ::std::option::Option<i32>,
        #[doc = "Number of static (i.e. cacheable) resources on the page."]
        #[serde(rename = "numberStaticResources", default)]
        pub number_static_resources: ::std::option::Option<i32>,
        #[doc = "Number of response bytes for other resources on the page."]
        #[serde(rename = "otherResponseBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub other_response_bytes: ::std::option::Option<i64>,
        #[doc = "Number of uncompressed response bytes for text resources not covered by other statistics (i.e non-HTML, non-script, non-CSS resources) on the page."]
        #[serde(rename = "textResponseBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub text_response_bytes: ::std::option::Option<i64>,
        #[doc = "Total size of all request bytes sent by the page."]
        #[serde(rename = "totalRequestBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub total_request_bytes: ::std::option::Option<i64>,
    }
    impl ::field_selector::FieldSelector for ResultPageStats {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ResultScreenshot {
        #[doc = "Image data base64 encoded."]
        #[serde(rename = "data", default)]
        pub data: ::std::option::Option<crate::bytes::Bytes>,
        #[doc = "Height of screenshot in pixels."]
        #[serde(rename = "height", default)]
        pub height: ::std::option::Option<i32>,
        #[doc = "Mime type of image data. E.g. \"image/jpeg\"."]
        #[serde(rename = "mime_type", default)]
        pub mime_type: ::std::option::Option<String>,
        #[doc = "Width of screenshot in pixels."]
        #[serde(rename = "width", default)]
        pub width: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for ResultScreenshot {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ResultVersion {
        #[doc = "The major version number of PageSpeed used to generate these results."]
        #[serde(rename = "major", default)]
        pub major: ::std::option::Option<i32>,
        #[doc = "The minor version number of PageSpeed used to generate these results."]
        #[serde(rename = "minor", default)]
        pub minor: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for ResultVersion {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
}
pub mod params {
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Alt {
        #[doc = "Responses with Content-Type of application/json"]
        Json,
    }
    impl Alt {
        pub fn as_str(self) -> &'static str {
            match self {
                Alt::Json => "json",
            }
        }
    }
    impl ::std::fmt::Display for Alt {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Alt {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Alt {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "json" => Alt::Json,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for Alt {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
}
pub struct Client<A> {
    reqwest: ::reqwest::Client,
    auth: ::std::sync::Mutex<A>,
}
impl<A: yup_oauth2::GetToken> Client<A> {
    pub fn new(auth: A) -> Self {
        Client {
            reqwest: ::reqwest::Client::builder().timeout(None).build().unwrap(),
            auth: ::std::sync::Mutex::new(auth),
        }
    }
    #[doc = "Actions that can be performed on the pagespeedapi resource"]
    pub fn pagespeedapi(&self) -> crate::resources::pagespeedapi::PagespeedapiActions<A> {
        crate::resources::pagespeedapi::PagespeedapiActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
pub mod resources {
    pub mod pagespeedapi {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum RunpagespeedStrategy {
                #[doc = "Fetch and analyze the URL for desktop browsers"]
                Desktop,
                #[doc = "Fetch and analyze the URL for mobile devices"]
                Mobile,
            }
            impl RunpagespeedStrategy {
                pub fn as_str(self) -> &'static str {
                    match self {
                        RunpagespeedStrategy::Desktop => "desktop",
                        RunpagespeedStrategy::Mobile => "mobile",
                    }
                }
            }
            impl ::std::fmt::Display for RunpagespeedStrategy {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for RunpagespeedStrategy {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for RunpagespeedStrategy {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "desktop" => RunpagespeedStrategy::Desktop,
                        "mobile" => RunpagespeedStrategy::Mobile,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::field_selector::FieldSelector for RunpagespeedStrategy {
                fn field_selector_with_ident(ident: &str, selector: &mut String) {
                    match selector.chars().rev().nth(0) {
                        Some(',') | None => {}
                        _ => selector.push_str(","),
                    }
                    selector.push_str(ident);
                }
            }
        }
        pub struct PagespeedapiActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> PagespeedapiActions<'a, A> {
            #[doc = "Runs PageSpeed analysis on the page at the specified URL, and returns a PageSpeed score, a list of suggestions to make that page faster, and other information."]
            pub fn runpagespeed(&self, url: impl Into<String>) -> RunpagespeedRequestBuilder<A> {
                RunpagespeedRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    url: url.into(),
                    filter_third_party_resources: None,
                    locale: None,
                    rule: None,
                    screenshot: None,
                    strategy: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct RunpagespeedRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            url: String,
            filter_third_party_resources: Option<bool>,
            locale: Option<String>,
            rule: Option<Vec<String>>,
            screenshot: Option<bool>,
            strategy: Option<crate::resources::pagespeedapi::params::RunpagespeedStrategy>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> RunpagespeedRequestBuilder<'a, A> {
            #[doc = "Indicates if third party resources should be filtered out before PageSpeed analysis."]
            pub fn filter_third_party_resources(mut self, value: bool) -> Self {
                self.filter_third_party_resources = Some(value);
                self
            }
            #[doc = "The locale used to localize formatted results"]
            pub fn locale(mut self, value: impl Into<String>) -> Self {
                self.locale = Some(value.into());
                self
            }
            #[doc = "A PageSpeed rule to run; if none are given, all rules are run"]
            pub fn rule(mut self, value: impl Into<Vec<String>>) -> Self {
                self.rule = Some(value.into());
                self
            }
            #[doc = "Indicates if binary data containing a screenshot should be included"]
            pub fn screenshot(mut self, value: bool) -> Self {
                self.screenshot = Some(value);
                self
            }
            #[doc = "The analysis strategy to use"]
            pub fn strategy(
                mut self,
                value: crate::resources::pagespeedapi::params::RunpagespeedStrategy,
            ) -> Self {
                self.strategy = Some(value);
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(mut self, value: impl Into<String>) -> Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(mut self, value: bool) -> Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
                self
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let fields = T::field_selector();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_standard(
                self,
            ) -> Result<crate::schemas::Result, Box<dyn ::std::error::Error>> {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Result, Box<dyn ::std::error::Error>> {
                self.execute_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://www.googleapis.com/pagespeedonline/v1/".to_owned();
                output.push_str("runPagespeed");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("url", &self.url)]);
                let req = req.query(&[(
                    "filter_third_party_resources",
                    &self.filter_third_party_resources,
                )]);
                let req = req.query(&[("locale", &self.locale)]);
                let req = req.query(&[("rule", &self.rule)]);
                let req = req.query(&[("screenshot", &self.screenshot)]);
                let req = req.query(&[("strategy", &self.strategy)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                req
            }
        }
    }
}
#[allow(dead_code)]
const SIMPLE: &::percent_encoding::AsciiSet = &::percent_encoding::NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'.')
    .remove(b'_')
    .remove(b'~');

#[allow(dead_code)]
const RESERVED: &::percent_encoding::AsciiSet = &SIMPLE
    .remove(b'%')
    .remove(b':')
    .remove(b'/')
    .remove(b'?')
    .remove(b'#')
    .remove(b'[')
    .remove(b']')
    .remove(b'@')
    .remove(b'!')
    .remove(b'$')
    .remove(b'&')
    .remove(b'\'')
    .remove(b'(')
    .remove(b')')
    .remove(b'*')
    .remove(b'+')
    .remove(b',')
    .remove(b';')
    .remove(b'=');
#[allow(dead_code)]
mod multipart {
    pub(crate) struct RelatedMultiPart {
        parts: Vec<Part>,
        boundary: String,
    }

    impl RelatedMultiPart {
        pub(crate) fn new() -> Self {
            RelatedMultiPart {
                parts: Vec::new(),
                boundary: ::textnonce::TextNonce::sized(68).unwrap().0,
            }
        }

        pub(crate) fn new_part(&mut self, part: Part) {
            self.parts.push(part);
        }

        pub(crate) fn boundary(&self) -> &str {
            &self.boundary
        }

        pub(crate) fn into_reader(self) -> RelatedMultiPartReader {
            let boundary_marker = boundary_marker(&self.boundary);
            RelatedMultiPartReader {
                state: RelatedMultiPartReaderState::WriteBoundary {
                    start: 0,
                    boundary: format!("{}\r\n", &boundary_marker),
                },
                boundary: boundary_marker,
                next_body: None,
                parts: self.parts.into_iter(),
            }
        }
    }

    pub(crate) struct Part {
        content_type: ::mime::Mime,
        body: Box<dyn ::std::io::Read + Send>,
    }

    impl Part {
        pub(crate) fn new(
            content_type: ::mime::Mime,
            body: Box<dyn ::std::io::Read + Send>,
        ) -> Part {
            Part { content_type, body }
        }
    }

    pub(crate) struct RelatedMultiPartReader {
        state: RelatedMultiPartReaderState,
        boundary: String,
        next_body: Option<Box<dyn ::std::io::Read + Send>>,
        parts: std::vec::IntoIter<Part>,
    }

    enum RelatedMultiPartReaderState {
        WriteBoundary {
            start: usize,
            boundary: String,
        },
        WriteContentType {
            start: usize,
            content_type: Vec<u8>,
        },
        WriteBody {
            body: Box<dyn ::std::io::Read + Send>,
        },
    }

    impl ::std::io::Read for RelatedMultiPartReader {
        fn read(&mut self, buf: &mut [u8]) -> ::std::io::Result<usize> {
            use RelatedMultiPartReaderState::*;
            let mut bytes_written: usize = 0;
            loop {
                let rem_buf = &mut buf[bytes_written..];
                match &mut self.state {
                    WriteBoundary { start, boundary } => {
                        let bytes_to_copy = std::cmp::min(boundary.len() - *start, rem_buf.len());
                        rem_buf[..bytes_to_copy]
                            .copy_from_slice(&boundary.as_bytes()[*start..*start + bytes_to_copy]);
                        *start += bytes_to_copy;
                        bytes_written += bytes_to_copy;
                        if *start == boundary.len() {
                            let next_part = match self.parts.next() {
                                None => break,
                                Some(part) => part,
                            };
                            self.next_body = Some(next_part.body);
                            self.state = WriteContentType {
                                start: 0,
                                content_type: format!(
                                    "Content-Type: {}\r\n\r\n",
                                    next_part.content_type
                                )
                                .into_bytes(),
                            };
                        } else {
                            break;
                        }
                    }
                    WriteContentType {
                        start,
                        content_type,
                    } => {
                        let bytes_to_copy =
                            std::cmp::min(content_type.len() - *start, rem_buf.len());
                        rem_buf[..bytes_to_copy]
                            .copy_from_slice(&content_type[*start..*start + bytes_to_copy]);
                        *start += bytes_to_copy;
                        bytes_written += bytes_to_copy;
                        if *start == content_type.len() {
                            self.state = WriteBody {
                                body: self.next_body.take().unwrap(),
                            };
                        } else {
                            break;
                        }
                    }
                    WriteBody { body } => {
                        let written = body.read(rem_buf)?;
                        bytes_written += written;
                        if written == 0 {
                            self.state = WriteBoundary {
                                start: 0,
                                boundary: format!("\r\n{}\r\n", &self.boundary),
                            };
                        } else {
                            break;
                        }
                    }
                }
            }
            Ok(bytes_written)
        }
    }

    fn boundary_marker(boundary: &str) -> String {
        let mut marker = String::with_capacity(boundary.len() + 2);
        marker.push_str("--");
        marker.push_str(boundary);
        marker
    }
}
// A serde helper module that can be used with the `with` attribute
// to deserialize any string to a FromStr type and serialize any
// Display type to a String. Google API's encode i64, u64 values as
// strings.
#[allow(dead_code)]
mod parsed_string {
    pub fn serialize<T, S>(
        value: &Option<T>,
        serializer: S,
    ) -> ::std::result::Result<S::Ok, S::Error>
    where
        T: ::std::fmt::Display,
        S: ::serde::Serializer,
    {
        use ::serde::Serialize;
        value.as_ref().map(|x| x.to_string()).serialize(serializer)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> ::std::result::Result<Option<T>, D::Error>
    where
        T: ::std::str::FromStr,
        T::Err: ::std::fmt::Display,
        D: ::serde::de::Deserializer<'de>,
    {
        use ::serde::Deserialize;
        match Option::<String>::deserialize(deserializer)? {
            Some(x) => Ok(Some(x.parse().map_err(::serde::de::Error::custom)?)),
            None => Ok(None),
        }
    }
}
// Bytes in google apis are represented as urlsafe base64 encoded strings.
// This defines a Bytes type that is a simple wrapper around a Vec<u8> used
// internally to handle byte fields in google apis.
pub mod bytes {
    use radix64::URL_SAFE as BASE64_CFG;

    #[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub struct Bytes(Vec<u8>);

    impl ::std::convert::From<Vec<u8>> for Bytes {
        fn from(x: Vec<u8>) -> Bytes {
            Bytes(x)
        }
    }

    impl ::std::fmt::Display for Bytes {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> ::std::fmt::Result {
            ::radix64::Display::new(BASE64_CFG, &self.0).fmt(f)
        }
    }

    impl ::serde::Serialize for Bytes {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::Serializer,
        {
            let encoded = BASE64_CFG.encode(&self.0);
            encoded.serialize(serializer)
        }
    }

    impl<'de> ::serde::Deserialize<'de> for Bytes {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Bytes, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            let encoded = String::deserialize(deserializer)?;
            let decoded = BASE64_CFG
                .decode(&encoded)
                .map_err(|_| ::serde::de::Error::custom("invalid base64 input"))?;
            Ok(Bytes(decoded))
        }
    }
}
