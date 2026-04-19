# Builder Development Guide
Only one builder should exist for each programming lanugae. The cannonical builder is
[base9-builder](https://github.com/base9-theme/base9-builder) in Rust.

All other builder should behave the same as the cannonical one.

However, since it's in different language, not everything is going to be
exactly the same. Each builder should come up with a good interface to adapt
its own lanuguage best practices.
