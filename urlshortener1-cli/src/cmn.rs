use clap::arg_enum;
use mime::Mime;
use serde_json::{self as json, value::Value};
use strsim;
use yup_oauth2::{ApplicationSecret, ConsoleApplicationSecret, Token, TokenStorage};

use std::{
    default::Default,
    env,
    error::Error as StdError,
    fmt, fs, io,
    io::{stdout, Write},
    path::{Path, PathBuf},
    str::FromStr,
    string::ToString,
};

const FIELD_SEP: char = '.';

#[allow(dead_code)]
#[derive(Clone)]
pub enum ComplexType {
    Pod,
    Vec,
    Map,
}

// Null,
// Bool(bool),
// I64(i64),
// U64(u64),
// F64(f64),
// String(String),

#[allow(dead_code)]
#[derive(Clone)]
pub enum JsonType {
    Boolean,
    Int,
    Uint,
    Float,
    String,
}

#[derive(Clone)]
pub struct JsonTypeInfo {
    pub jtype: JsonType,
    pub ctype: ComplexType,
}

// Based on @erickt user comment. Thanks for the idea !
// Remove all keys whose values are null from given value (changed in place)
#[allow(dead_code)]
pub fn remove_json_null_values(value: &mut Value) {
    match *value {
        Value::Object(ref mut map) => {
            let mut for_removal = Vec::new();

            for (key, mut value) in map.iter_mut() {
                if value.is_null() {
                    for_removal.push(key.clone());
                } else {
                    remove_json_null_values(&mut value);
                }
            }

            for key in &for_removal {
                map.remove(key);
            }
        }
        json::value::Value::Array(ref mut arr) => {
            let mut i = 0;
            while i < arr.len() {
                if arr[i].is_null() {
                    arr.remove(i);
                } else {
                    remove_json_null_values(&mut arr[i]);
                    i += 1;
                }
            }
        }
        _ => {}
    }
}

pub fn object_from_kvargs(
    opts: impl IntoIterator<Item = impl AsRef<str>>,
    err: &mut InvalidOptionsError,
    fields: &[(&'static str, JsonTypeInfo)],
) -> json::value::Value {
    let mut field_cursor = FieldCursor::default();
    let mut object = json::value::Value::Object(Default::default());
    for kvarg in opts.into_iter() {
        let kvarg = kvarg.as_ref();
        let last_errc = err.issues.len();
        let (key, value) = parse_kv_arg(&*kvarg, err, false);
        let mut temp_cursor = field_cursor.clone();
        if let Err(field_err) = temp_cursor.set(&*key) {
            err.issues.push(field_err);
        }
        if value.is_none() {
            field_cursor = temp_cursor.clone();
            if err.issues.len() > last_errc {
                err.issues.remove(last_errc);
            }
            continue;
        }

        let cursor_string = temp_cursor.to_string();
        let type_info = fields
            .iter()
            .find(|(field, _)| *field == cursor_string)
            .cloned()
            .or_else(|| {
                let suggestion = FieldCursor::did_you_mean(key, fields.iter().map(|s| s.0));
                err.issues.push(CLIError::Field(FieldError::Unknown(
                    temp_cursor.to_string(),
                    suggestion,
                    value.map(|v| v.to_string()),
                )));
                None
            });
        if let Some((field_cursor_str, type_info)) = type_info {
            FieldCursor::from(field_cursor_str).set_json_value(
                &mut object,
                value.unwrap(),
                type_info,
                err,
                &temp_cursor,
            );
        }
    }
    object
}

fn did_you_mean(
    v: &str,
    possible_values: impl IntoIterator<Item = impl AsRef<str>>,
) -> Option<String> {
    let mut candidate: Option<(f64, String)> = None;
    for pv in possible_values.into_iter() {
        let pv = pv.as_ref();
        let confidence = strsim::jaro_winkler(v, pv);
        if confidence > 0.8 && (candidate.is_none() || (candidate.as_ref().unwrap().0 < confidence))
        {
            candidate = Some((confidence, pv.to_owned()));
        }
    }
    match candidate {
        None => None,
        Some((_, candidate)) => Some(candidate),
    }
}

#[allow(dead_code)]
pub enum CallType {
    Upload(UploadProtocol),
    Standard,
}

arg_enum! {
    pub enum UploadProtocol {
        Simple,
        Resumable
    }
}

impl AsRef<str> for UploadProtocol {
    fn as_ref(&self) -> &str {
        match *self {
            UploadProtocol::Simple => "simple",
            UploadProtocol::Resumable => "resumable",
        }
    }
}

impl AsRef<str> for CallType {
    fn as_ref(&self) -> &str {
        match *self {
            CallType::Upload(ref proto) => proto.as_ref(),
            CallType::Standard => "standard-request",
        }
    }
}

#[derive(Clone, Default)]
pub struct FieldCursor(Vec<String>);

impl ToString for FieldCursor {
    fn to_string(&self) -> String {
        self.0.join(".")
    }
}

impl From<&'static str> for FieldCursor {
    fn from(value: &'static str) -> FieldCursor {
        let mut res = FieldCursor::default();
        res.set(value).unwrap();
        res
    }
}

// TODO: use the 'entry()' method instead
fn assure_entry<'a, 'b>(m: &'a mut json::Map<String, Value>, k: &'b str) -> &'a mut Value {
    if m.contains_key(k) {
        return m.get_mut(k).expect("value to exist");
    }
    m.insert(k.to_owned(), Value::Object(Default::default()));
    m.get_mut(k).expect("value to exist")
}

impl FieldCursor {
    pub fn set(&mut self, value: &str) -> Result<(), CLIError> {
        if value.is_empty() {
            return Err(CLIError::Field(FieldError::Empty));
        }

        let mut first_is_field_sep = false;
        let mut char_count: usize = 0;
        let mut last_c = FIELD_SEP;
        let mut num_conscutive_field_seps = 0;

        let mut field = String::new();
        let mut fields = self.0.clone();

        let push_field = |fs: &mut Vec<String>, f: &mut String| {
            if !f.is_empty() {
                fs.push(f.clone());
                f.truncate(0);
            }
        };

        for (cid, c) in value.chars().enumerate() {
            char_count += 1;

            if c == FIELD_SEP {
                if cid == 0 {
                    first_is_field_sep = true;
                }
                num_conscutive_field_seps += 1;
                if cid > 0 && last_c == FIELD_SEP {
                    if fields.pop().is_none() {
                        return Err(CLIError::Field(FieldError::PopOnEmpty(value.to_string())));
                    }
                } else {
                    push_field(&mut fields, &mut field);
                }
            } else {
                num_conscutive_field_seps = 0;
                if cid == 1 && first_is_field_sep {
                    fields.truncate(0);
                }
                field.push(c);
            }

            last_c = c;
        }

        push_field(&mut fields, &mut field);

        if char_count == 1 && first_is_field_sep {
            fields.truncate(0);
        }
        if char_count > 1 && num_conscutive_field_seps == 1 {
            return Err(CLIError::Field(FieldError::TrailingFieldSep(
                value.to_string(),
            )));
        }

        self.0 = fields;
        Ok(())
    }

    pub fn did_you_mean(
        value: &str,
        possible_values: impl IntoIterator<Item = &'static str>,
    ) -> Option<String> {
        if value.is_empty() {
            return None;
        }

        let mut last_c = FIELD_SEP;

        let mut field = String::new();
        let mut output = String::new();

        let possible_values: Vec<_> = possible_values.into_iter().collect();
        let push_field = |fs: &mut String, f: &mut String| {
            if !f.is_empty() {
                match did_you_mean(&f, &possible_values) {
                    Some(candidate) => fs.push_str(&candidate),
                    None => fs.push_str(f),
                };
                f.truncate(0);
            }
        };

        for c in value.chars() {
            if c == FIELD_SEP {
                if last_c != FIELD_SEP {
                    push_field(&mut output, &mut field);
                }
                output.push(c);
            } else {
                field.push(c);
            }

            last_c = c;
        }

        push_field(&mut output, &mut field);

        if output == value {
            None
        } else {
            Some(output)
        }
    }

    pub fn set_json_value(
        &self,
        mut object: &mut Value,
        value: &str,
        type_info: JsonTypeInfo,
        err: &mut InvalidOptionsError,
        orig_cursor: &FieldCursor,
    ) {
        assert!(!self.0.is_empty());

        for field in &self.0[..self.0.len() - 1] {
            let tmp = object;
            object = match *tmp {
                Value::Object(ref mut mapping) => assure_entry(mapping, &field),
                _ => panic!("We don't expect non-object Values here ..."),
            };
        }

        match *object {
            Value::Object(ref mut mapping) => {
                let field = &self.0[self.0.len() - 1];
                let to_jval =
                    |value: &str, jtype: JsonType, err: &mut InvalidOptionsError| -> Value {
                        match jtype {
                            JsonType::Boolean => {
                                Value::Bool(arg_from_str(value, err, &field, "boolean"))
                            }
                            JsonType::Int => Value::Number(
                                json::Number::from_f64(arg_from_str(value, err, &field, "int"))
                                    .expect("valid f64"),
                            ),
                            JsonType::Uint => Value::Number(
                                json::Number::from_f64(arg_from_str(value, err, &field, "uint"))
                                    .expect("valid f64"),
                            ),
                            JsonType::Float => Value::Number(
                                json::Number::from_f64(arg_from_str(value, err, &field, "float"))
                                    .expect("valid f64"),
                            ),
                            JsonType::String => Value::String(value.to_owned()),
                        }
                    };

                match type_info.ctype {
                    ComplexType::Pod => {
                        if mapping
                            .insert(field.to_owned(), to_jval(value, type_info.jtype, err))
                            .is_some()
                        {
                            err.issues.push(CLIError::Field(FieldError::Duplicate(
                                orig_cursor.to_string(),
                            )));
                        }
                    }
                    ComplexType::Vec => match *assure_entry(mapping, field) {
                        Value::Array(ref mut values) => {
                            values.push(to_jval(value, type_info.jtype, err))
                        }
                        _ => unreachable!(),
                    },
                    ComplexType::Map => {
                        let (key, value) = parse_kv_arg(value, err, true);
                        let jval = to_jval(value.unwrap_or(""), type_info.jtype, err);

                        match *assure_entry(mapping, &field) {
                            Value::Object(ref mut value_map) => {
                                if value_map.insert(key.to_owned(), jval).is_some() {
                                    err.issues.push(CLIError::Field(FieldError::Duplicate(
                                        orig_cursor.to_string(),
                                    )));
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub fn num_fields(&self) -> usize {
        self.0.len()
    }
}

pub fn parse_kv_arg<'a>(
    kv: &'a str,
    err: &mut InvalidOptionsError,
    for_hashmap: bool,
) -> (&'a str, Option<&'a str>) {
    let mut add_err = || {
        err.issues
            .push(CLIError::InvalidKeyValueSyntax(kv.to_string(), for_hashmap))
    };
    match kv.find('=') {
        None => {
            add_err();
            (kv, None)
        }
        Some(pos) => {
            let key = &kv[..pos];
            if kv.len() <= pos + 1 {
                add_err();
                return (key, Some(""));
            }
            (key, Some(&kv[pos + 1..]))
        }
    }
}

#[allow(dead_code)]
pub fn calltype_from_str(
    name: &str,
    valid_protocols: Vec<String>,
    err: &mut InvalidOptionsError,
) -> CallType {
    CallType::Upload(match UploadProtocol::from_str(name) {
        Ok(up) => up,
        Err(_msg) => {
            err.issues.push(CLIError::InvalidUploadProtocol(
                name.to_string(),
                valid_protocols,
            ));
            UploadProtocol::Simple
        }
    })
}

#[allow(dead_code)]
pub fn input_file_from_opts(file_path: &str, err: &mut InvalidOptionsError) -> Option<fs::File> {
    match fs::File::open(file_path) {
        Ok(f) => Some(f),
        Err(io_err) => {
            err.issues.push(CLIError::Input(InputError::Io((
                file_path.to_string(),
                io_err,
            ))));
            None
        }
    }
}

#[allow(dead_code)]
pub fn input_mime_from_opts(mime: &str, err: &mut InvalidOptionsError) -> Option<Mime> {
    match mime.parse() {
        Ok(m) => Some(m),
        Err(_) => {
            err.issues
                .push(CLIError::Input(InputError::Mime(mime.to_string())));
            None
        }
    }
}

pub fn writer_from_opts(arg: Option<&str>) -> Result<Box<dyn Write>, io::Error> {
    let f = arg.unwrap_or("-");
    match f {
        "-" => Ok(Box::new(stdout())),
        _ => match fs::OpenOptions::new().create(true).write(true).open(f) {
            Ok(f) => Ok(Box::new(f)),
            Err(io_err) => Err(io_err),
        },
    }
}

pub fn arg_from_str<'a, T>(
    arg: &str,
    err: &mut InvalidOptionsError,
    arg_name: &'a str,
    arg_type: &'a str,
) -> T
where
    T: FromStr + Default,
    <T as FromStr>::Err: fmt::Display,
{
    match FromStr::from_str(arg) {
        Err(perr) => {
            err.issues.push(CLIError::ParseError(
                arg_name.to_owned(),
                arg_type.to_owned(),
                arg.to_string(),
                format!("{}", perr),
            ));
            Default::default()
        }
        Ok(v) => v,
    }
}

pub struct JsonTokenStorage {
    pub program_name: &'static str,
    pub db_dir: String,
}

impl JsonTokenStorage {
    fn path(&self, scope_hash: u64) -> PathBuf {
        Path::new(&self.db_dir).join(&format!("{}-token-{}.json", self.program_name, scope_hash))
    }
}

#[derive(Debug)]
pub enum TokenStorageError {
    Json(json::Error),
    Io(io::Error),
}

impl fmt::Display for TokenStorageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            TokenStorageError::Json(ref err) => writeln!(f, "Could not serialize secrets: {}", err),
            TokenStorageError::Io(ref err) => writeln!(f, "Failed to write secret token: {}", err),
        }
    }
}

impl StdError for TokenStorageError {
    fn description(&self) -> &str {
        "Failure when getting or setting the token storage"
    }
}

impl TokenStorage for JsonTokenStorage {
    type Error = TokenStorageError;

    // NOTE: logging might be interesting, currently we swallow all errors
    fn set(
        &mut self,
        scope_hash: u64,
        _: &Vec<&str>,
        token: Option<Token>,
    ) -> Result<(), TokenStorageError> {
        match token {
            None => match fs::remove_file(self.path(scope_hash)) {
                Err(err) => match err.kind() {
                    io::ErrorKind::NotFound => Ok(()),
                    _ => Err(TokenStorageError::Io(err)),
                },
                Ok(_) => Ok(()),
            },
            Some(token) => {
                match fs::OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open(&self.path(scope_hash))
                {
                    Ok(mut f) => match json::to_writer_pretty(&mut f, &token) {
                        Ok(_) => Ok(()),
                        Err(serde_err) => Err(TokenStorageError::Json(serde_err)),
                    },
                    Err(io_err) => Err(TokenStorageError::Io(io_err)),
                }
            }
        }
    }

    fn get(&self, scope_hash: u64, _: &Vec<&str>) -> Result<Option<Token>, TokenStorageError> {
        match fs::File::open(&self.path(scope_hash)) {
            Ok(f) => match json::de::from_reader(f) {
                Ok(token) => Ok(Some(token)),
                Err(err) => Err(TokenStorageError::Json(err)),
            },
            Err(io_err) => match io_err.kind() {
                io::ErrorKind::NotFound => Ok(None),
                _ => Err(TokenStorageError::Io(io_err)),
            },
        }
    }
}

#[derive(Debug)]
pub enum ApplicationSecretError {
    DecoderError((String, json::Error)),
    FormatError(String),
}

impl fmt::Display for ApplicationSecretError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            ApplicationSecretError::DecoderError((ref path, ref err)) => writeln!(
                f,
                "Could not decode file at '{}' with error: {}.",
                path, err
            ),
            ApplicationSecretError::FormatError(ref path) => writeln!(
                f,
                "'installed' field is unset in secret file at '{}'.",
                path
            ),
        }
    }
}

#[derive(Debug)]
pub enum ConfigurationError {
    DirectoryCreationFailed((String, io::Error)),
    DirectoryUnset,
    HomeExpansionFailed(String),
    Secret(ApplicationSecretError),
    Io((String, io::Error)),
}

impl fmt::Display for ConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            ConfigurationError::DirectoryCreationFailed((ref dir, ref err)) => writeln!(
                f,
                "Directory '{}' could not be created with error: {}.",
                dir, err
            ),
            ConfigurationError::DirectoryUnset => writeln!(f, "--config-dir was unset or empty."),
            ConfigurationError::HomeExpansionFailed(ref dir) => writeln!(
                f,
                "Couldn't find HOME directory of current user, failed to expand '{}'.",
                dir
            ),
            ConfigurationError::Secret(ref err) => writeln!(f, "Secret -> {}", err),
            ConfigurationError::Io((ref path, ref err)) => writeln!(
                f,
                "IO operation failed on path '{}' with error: {}.",
                path, err
            ),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum InputError {
    Io((String, io::Error)),
    Mime(String),
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            InputError::Io((ref file_path, ref io_err)) => writeln!(
                f,
                "Failed to open '{}' for reading with error: {}.",
                file_path, io_err
            ),
            InputError::Mime(ref mime) => writeln!(f, "'{}' is not a known mime-type.", mime),
        }
    }
}

#[derive(Debug)]
pub enum FieldError {
    PopOnEmpty(String),
    TrailingFieldSep(String),
    Unknown(String, Option<String>, Option<String>),
    Duplicate(String),
    Empty,
}

impl fmt::Display for FieldError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            FieldError::PopOnEmpty(ref field) => {
                writeln!(f, "'{}': Cannot move up on empty field cursor.", field)
            }
            FieldError::TrailingFieldSep(ref field) => writeln!(
                f,
                "'{}': Single field separator may not be last character.",
                field
            ),
            FieldError::Unknown(ref field, ref suggestion, ref value) => {
                let suffix = match *suggestion {
                    Some(ref s) => {
                        let kv = match *value {
                            Some(ref v) => format!("{}={}", s, v),
                            None => s.clone(),
                        };
                        format!(" Did you mean '{}' ?", kv)
                    }
                    None => String::new(),
                };
                writeln!(f, "Field '{}' does not exist.{}", field, suffix)
            }
            FieldError::Duplicate(ref cursor) => {
                writeln!(f, "Value at '{}' was already set", cursor)
            }
            FieldError::Empty => writeln!(f, "Field names must not be empty."),
        }
    }
}

#[derive(Debug)]
pub enum CLIError {
    Configuration(ConfigurationError),
    ParseError(String, String, String, String),
    UnknownParameter(String, Vec<&'static str>),
    InvalidUploadProtocol(String, Vec<String>),
    InvalidKeyValueSyntax(String, bool),
    Input(InputError),
    Field(FieldError),
    MissingCommandError,
    MissingMethodError(String),
}

impl fmt::Display for CLIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            CLIError::Configuration(ref err) => write!(f, "Configuration -> {}", err),
            CLIError::Input(ref err) => write!(f, "Input -> {}", err),
            CLIError::Field(ref err) => write!(f, "Field -> {}", err),
            CLIError::InvalidUploadProtocol(ref proto_name, ref valid_names) => writeln!(
                f,
                "'{}' is not a valid upload protocol. Choose from one of {}.",
                proto_name,
                valid_names.join(", ")
            ),
            CLIError::ParseError(ref arg_name, ref type_name, ref value, ref err_desc) => writeln!(
                f,
                "Failed to parse argument '{}' with value '{}' as {} with error: {}.",
                arg_name, value, type_name, err_desc
            ),
            CLIError::UnknownParameter(ref param_name, ref possible_values) => {
                let suffix = match did_you_mean(param_name, possible_values) {
                    Some(v) => format!(" Did you mean '{}' ?", v),
                    None => String::new(),
                };
                writeln!(f, "Parameter '{}' is unknown.{}", param_name, suffix)
            }
            CLIError::InvalidKeyValueSyntax(ref kv, is_hashmap) => {
                let hashmap_info = if is_hashmap { "hashmap " } else { "" };
                writeln!(
                    f,
                    "'{}' does not match {}pattern <key>=<value>.",
                    kv, hashmap_info
                )
            }
            CLIError::MissingCommandError => writeln!(f, "Please specify the main sub-command."),
            CLIError::MissingMethodError(ref cmd) => writeln!(
                f,
                "Please specify the method to call on the '{}' command.",
                cmd
            ),
        }
    }
}

#[derive(Debug)]
pub struct InvalidOptionsError {
    pub issues: Vec<CLIError>,
    pub exit_code: i32,
}

impl fmt::Display for InvalidOptionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for issue in &self.issues {
            issue.fmt(f)?;
        }
        Ok(())
    }
}

impl InvalidOptionsError {
    pub fn single(err: CLIError, exit_code: i32) -> InvalidOptionsError {
        InvalidOptionsError {
            issues: vec![err],
            exit_code,
        }
    }

    pub fn new() -> InvalidOptionsError {
        InvalidOptionsError {
            issues: Vec::new(),
            exit_code: 1,
        }
    }
}

pub fn assure_config_dir_exists(dir: &str) -> Result<String, CLIError> {
    let trdir = dir.trim();
    if trdir.is_empty() {
        return Err(CLIError::Configuration(ConfigurationError::DirectoryUnset));
    }

    let expanded_config_dir = if trdir.as_bytes()[0] == b'~' {
        match env::var("HOME")
            .ok()
            .or_else(|| env::var("UserProfile").ok())
        {
            None => {
                return Err(CLIError::Configuration(
                    ConfigurationError::HomeExpansionFailed(trdir.to_string()),
                ))
            }
            Some(mut user) => {
                user.push_str(&trdir[1..]);
                user
            }
        }
    } else {
        trdir.to_string()
    };

    if let Err(err) = fs::create_dir(&expanded_config_dir) {
        if err.kind() != io::ErrorKind::AlreadyExists {
            return Err(CLIError::Configuration(
                ConfigurationError::DirectoryCreationFailed((expanded_config_dir, err)),
            ));
        }
    }

    Ok(expanded_config_dir)
}

pub fn application_secret_from_directory(
    dir: &str,
    secret_basename: &str,
    json_console_secret: &str,
) -> Result<ApplicationSecret, CLIError> {
    let secret_path = Path::new(dir).join(secret_basename);
    let secret_str = || secret_path.as_path().to_str().unwrap().to_string();
    let secret_io_error = |io_err: io::Error| {
        Err(CLIError::Configuration(ConfigurationError::Io((
            secret_str(),
            io_err,
        ))))
    };

    for _ in 0..2 {
        match fs::File::open(&secret_path) {
            Err(mut err) => {
                if err.kind() == io::ErrorKind::NotFound {
                    // Write our built-in one - user may adjust the written file at will

                    err = match fs::OpenOptions::new()
                        .create(true)
                        .write(true)
                        .open(&secret_path)
                    {
                        Err(cfe) => cfe,
                        Ok(mut f) => {
                            // Assure we convert 'ugly' json string into pretty one
                            let console_secret: ConsoleApplicationSecret =
                                json::from_str(json_console_secret).unwrap();
                            match json::to_writer_pretty(&mut f, &console_secret) {
                                Err(serde_err) => {
                                    panic!("Unexpected serde error: {:#?}", serde_err)
                                }
                                Ok(_) => continue,
                            }
                        }
                    };
                    // fall through to IO error handling
                }
                return secret_io_error(err);
            }
            Ok(f) => match json::de::from_reader::<_, ConsoleApplicationSecret>(f) {
                Err(json_err) => {
                    return Err(CLIError::Configuration(ConfigurationError::Secret(
                        ApplicationSecretError::DecoderError((secret_str(), json_err)),
                    )))
                }
                Ok(console_secret) => match console_secret.installed {
                    Some(secret) => return Ok(secret),
                    None => {
                        return Err(CLIError::Configuration(ConfigurationError::Secret(
                            ApplicationSecretError::FormatError(secret_str()),
                        )))
                    }
                },
            },
        }
    }
    unreachable!();
}
