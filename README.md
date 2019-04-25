RAD tools for making high-performance web applications with rust while avoiding
a definite vendor lock-in.

Commands:

- create application of given architecture and parameters
  - available: ddd, crud
- create plugin
- create aggregate root
- create value object
- create new command

Plugins:

- web
- cli
- logging
- appendlog
- projector

Each of the plugins has options and configurations:

Logging:

- logging adapters and filters (mem, ...)

Appendlog:

- storage (mem, sql, redis, ...)

Projector:

- storage (mem, sql, mongodb, ...)
