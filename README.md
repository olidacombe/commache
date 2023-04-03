# commache

A general command output cache.

This is a work in progress, and in its current form
leaves orphan/zombie processes behing on every invokation.

### TODO

Transition to a daemon implementation, where the clien
queries the daemon, and the daemon is responsible for the
lazy after-return fetching.

License: MIT OR Apache-2.0
