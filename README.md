Example tool for architecture in rust
=====================================

This is a task tracker. Beside doing its job, the code also shows an example
architecture in rust - loosely based on ideas from DDD.

Features
========

* works for teams
* tracks tasks
* aware of people working on tasks
* aware of projects
* aware of tags
* aware of contexts
* dependency among tasks
* nested tasks
* multiple interfaces: CLI, Web, GUI
* tracks time
* tracks interruptions
  * reason for interruption
  * assign interruption to people
  * assign interruption to organizations
* tasks nested in each other arbitrarily
* workflows: implements multiple workflows
  * simple - a simple list of tasks, one underneath the other
  * lanes - a grid of tasks
  * gtd - combine gtd with another workflow
  * kanban
  * scrum
* extensible with common scripting languages
  * python
  * javascript
* templates for tasks
* can generate gantt charts

Implementation Features
=======================

* shows how to protect your project from vendor lock-in in an ecosystem which
  is very much in flux
* multiple plugins for UIs, storage and logging
* based on event sourcing and cqrs
* pluggable architecture
* multiple data stores
* easy to recombine features
* plugins can register new commands
* the core is in rust, but many features are implemented via plugins
* if we notice features that solidify over the years, we integrate them into
  core

Things in core at first
=======================

* all entities and crud methods for them, recording changes to them
* event bus
* command dispatcher
* all command DTOs
* everything absolutely necessary for making the UIs
* all events

Things in plugins
=================

* command handlers


Features of a task
=================

Fundamental fields

* id
* uuid
* description
* tags `#<string>`
* properties `prop1=val`
* asignee `+<nickname>`
* creator `+<nickname>`

Relations to other tasks

* children = finishing this task is possible only when the children are
  finished
* blockingtasks = progress cannot be done on this task until the other tasks
  are finished (they are not children)
* siblingstart = starting this task automatically starts the other task
* siblingfinish = finishing this task automatically finishes the other task

Worklog

A worklog entry consists of a type, a start datetime, an end time, and various
other data based on the type:

* tangible - when ended, commit id and description are taken from git
* intangible - when started, you must enter a description; other properties can
  be attached
* interruption - when started, the currently running log is captured and
  stopped; when stopped, you are asked to enter a description (can include
  `+<nickname>` to tag people causing the interruption)

Numeric properties

* difficulty
* risk
* duration - must be entered before started first
* priority (L,,M,H)

They can be calculated via various heuristics by plugins. New properties can be
created by plugins, removed, values changed or names renamed.

This is an area of research and the heuristics can vary between organizations.

Features of people
==================

* fields:
  * nickname
  * Full name
  * email
  * id
* starting state as a skill matrix
* skill matrix in which tags are technologies, techniques, libraries, frameworks
* skill matrix includes experience level (1-5), years of experience and last
  used date
* skill matrix can be used by plugins to assess difficulty, risk and duration
  of tasks
* skill matrix is updated automatically by plugins

Entities in the system
======================

* task
* tasklist
* project - consists of tasks
* organization - consists of people
* context
* people
