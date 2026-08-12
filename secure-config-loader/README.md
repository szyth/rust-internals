# secure-config-loader

A config loader chaining three independently-failable steps — read a file, parse `key=value` lines, validate a port — into one `ConfigError` enum via `?` and three `From` impls.

## What's in here

`load_and_validate_port()` is nothing but three `?`s: `read_config_file(path)?`, `parse_config(&raw)?`, `validate_port(&config)?`. Each step has its own error type (`std::io::Error`, `ParseError`, `ValidationError`); `ConfigError` wraps all three and gets a `From` impl per variant, so `?` converts automatically at each step with zero manual `match`/wrapping in `load_and_validate_port` itself.

## Real bugs hit and fixed while building it

- An inverted port-range check (`port < 1 || port > 1024`) that accepted privileged ports and rejected valid ones — the exact opposite of the intended `> 1024` rule.
- An off-by-one at the boundary (`port < 1024` let `1024` itself through, when the rule is strictly `> 1024`).
- Empty lines in the config initially treated as malformed instead of skipped, which would break on any file with a blank line.
- `assert_eq!` doesn't work against `ConfigError::Io` — `std::io::Error` doesn't implement `PartialEq`, so `matches!(result, Err(ConfigError::Io(_)))` is the tool for checking "this failed the way I expect" without needing full equality.
