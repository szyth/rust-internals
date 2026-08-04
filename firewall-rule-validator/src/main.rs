// 2.7 — Deref, DerefMut, From/Into, TryFrom/TryInto, newtype pattern
// Exercise: Firewall Rule Config Validator
// Spec: see §4 of "2.7 Deref, DerefMut, From, Into, TryFrom, TryInto, newtype pattern.md" in the vault.
// Steps 1-5 complete.

struct Port(u16);

#[derive(Debug, PartialEq)]
enum PortError {
    OutOfRange(u32), // carries the original value, useful for a clear error message
    Zero,
}

impl TryFrom<u32> for Port {
    type Error = PortError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value == 0 {
            return Err(PortError::Zero);
        }

        match u16::try_from(value) {
            Ok(port) => Ok(Self(port)),
            Err(_e) => Err(PortError::OutOfRange(value)),
        }
    }
}

impl Port {
    fn value(&self) -> u16 {
        self.0
    }
}
// use std::ops::Deref;
// impl Deref for Port {
//     type Target = u16;
//
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

struct Hostname(String);

#[derive(Debug, PartialEq)]
enum HostnameError {
    EmptyHostname,
}
impl TryFrom<String> for Hostname {
    type Error = HostnameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(HostnameError::EmptyHostname);
        }

        Ok(Hostname(value))
    }
}

struct RawRuleConfig {
    host: String,
    port: u32,
}

struct FirewallRule {
    host: Hostname,
    port: Port,
}

#[derive(Debug, PartialEq)]
enum FirewallRuleError {
    Host(HostnameError),
    Port(PortError),
}

impl From<PortError> for FirewallRuleError {
    fn from(value: PortError) -> Self {
        FirewallRuleError::Port(value)
    }
}
impl From<HostnameError> for FirewallRuleError {
    fn from(value: HostnameError) -> Self {
        FirewallRuleError::Host(value)
    }
}

impl TryFrom<RawRuleConfig> for FirewallRule {
    type Error = FirewallRuleError;

    fn try_from(value: RawRuleConfig) -> Result<Self, Self::Error> {
        let port = Port::try_from(value.port)?;
        let host = Hostname::try_from(value.host)?;

        Ok(Self { host, port })
    }
}

fn main() {
    // let port = Port(8080);
    // compiles. "Parse, dont validate" invalidated
    // For untrusted inputs, Wrapper should not leak raw inner value using Deref.
    // Here, caller has access to full inner value
    // let _large_arthematic_on_port = *port * u16::MAX;

    // Wrapper should allow access to raw inner value via ONLY THE PROVIDED METHODS
    // Here, the implementor controls the inner value using methods.
    // let _large_arthematic_on_port = port.value() * u16::MAX;

    let raw = RawRuleConfig {
        host: "internal.example.com".to_string(),
        port: 8080,
    };

    let bad_port = RawRuleConfig {
        host: "internal.example.com".to_string(),
        port: 0, // triggers PortError::Zero
    };

    let bad_host = RawRuleConfig {
        host: String::new(), // triggers HostnameError::EmptyHostname
        port: 8080,
    };

    let rule = FirewallRule::try_from(raw);
    let rule_bad_port = FirewallRule::try_from(bad_port);
    let rule_bad_host = FirewallRule::try_from(bad_host);

    assert!(rule.is_ok());
    assert!(rule_bad_port.is_err_and(|err| err == FirewallRuleError::Port(PortError::Zero)));
    assert!(
        rule_bad_host
            .is_err_and(|err| err == FirewallRuleError::Host(HostnameError::EmptyHostname))
    );

    // error[E0308]: expected Port found u32
    // even though Port's fallible constructor accepts a u32, Port itself wraps a u16 --
    // the compiler treats Port and u32 as two entirely unrelated types
    // fn scan_port(port: Port) {
    //     let _ = port;
    // };
    // let _ = scan_port(8080u32);
}
