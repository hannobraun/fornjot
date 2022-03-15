# Release Operator

Detect when a release pull-request is merged.

## Logging

The log level is set on the environment using `RUST_LOG` _(see [docs.rs/env_logger])

- The default log-level is set to `info` and yields the minimum amount of messages
- Use the `debug` level to see the reasons for the tools decisions
- Use the `trace` level to see details about every API call in the background
  - You will also see how long the program took to execute

[docs.rs/env_logger]: https://docs.rs/env_logger/latest/env_logger/#enabling-logging

## Related Issues

- https://github.com/hannobraun/Fornjot/issues/104
