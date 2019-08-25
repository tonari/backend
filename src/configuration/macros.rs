//! Provides a macro for configuration variables.

/// Derives the name of the configuration environment variable.
#[macro_export]
macro_rules! env_var_name {
    // NOTE: This must be a macro, because it has to work with literals
    ($name:ident) => {
        concat!("TONARI_", stringify!($name))
    };
}

/// Configure a program variable that can optionally be set by an environment variable.
///
/// A configuration variable has a name, a type, and a specified default value.
/// The code for the default value is executed the first time it is used, but only if
/// no environment variable of the same name exists with a valid value for the type.
/// If it does, that value is used.
#[macro_export]
macro_rules! configuration_variable {
    ($($(#[$meta:meta])* pub static ref $name:ident: $($type:ty)* = $val:expr;)*) => {
        $(
            configuration_variable_internal!($(#[$meta])* pub static ref $name: $($type)* = $val);
        )*
    };
}

macro_rules! configuration_variable_internal {
    ($(#[$meta:meta])* pub static ref $name:ident: $type:ty = $val:expr) => {
        lazy_static! {
            $(
                #[$meta]
            )*
            // Configuration variables are always public.
            pub static ref $name: $type = {
                const VARNAME: &str = env_var_name!($name);

                std::env::var(VARNAME)
                    .ok()
                    .and_then(|val|
                        val.parse::<$type>()
                        .map_err(|err|
                            eprintln!("Could not parse environment variable {}: {}. Using default value ({}).", VARNAME, err, $val)
                        ).ok())
                    .unwrap_or_else(|| $val)
            };
        }
    };
    ($(#[$meta:meta])* pub static ref $name:ident: = $val:expr) => {
        lazy_static! {
            $(
                #[$meta]
            )*
            // Configuration variables are always public.
            pub static ref $name: String = {
                const VARNAME: &str = env_var_name!($name);

                std::env::var(VARNAME)
                    .ok()
                    .map(|val| val.into())
                    .unwrap_or_else(|| $val.into())
            };
        }
    };
}
