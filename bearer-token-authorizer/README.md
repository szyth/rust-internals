# bearer-token-authorizer

A bearer-token authorization chain — header lookup, prefix stripping, session lookup, expiry check, permission check — built as a single `Result`/`Option` combinator chain instead of a nested `match`/`if let` pyramid.

## What's in here

`authorize()` threads a `&Session` through five independently-failable steps, each tagged with its own `AuthError` variant (`MissingHeader`, `MalformedHeader`, `UnknownToken`, `SessionExpired`, `InsufficientPermissions`) via `.ok_or()`/`.ok_or_else()` and `.and_then()` — never collapsing two different failure reasons into a single `None`/generic error. The two real traps hit while building it: reaching for `.map()` where `.and_then()` was needed (turns a `bool`/an always-`Ok` transform where a branching `Ok`/`Err` was required), and getting the expiry comparison's branches backwards (`now >= expires_at` means expired, not valid).

## Why not just `Option<Session>`

Collapsing all five failure modes down to a bare `None` would be shorter, but it throws away exactly the information worth keeping — "was this rejected for a missing header, an unknown token, or an expired session?" all look identical. Keeping a specific `AuthError` per failure, even inside a terse combinator chain, keeps that diagnostic information for free.
