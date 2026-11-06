# TODO
## General
- Improve comments
- Provide documentation
## Design
- Support ForkServer
- Support InProcess Fuzzing
- Support TinyInst instrumentation
- Support Frida instrumentation
- Seed scheduling / energy: worker re-runs every input in node_pool exactly once per pass and the pool only grows, so promising inputs get diluted instead of prioritized => wire in scheduler.rs (priority queue) and assign energy per seed
## New Features
- Derive Dictionnaries from inputs
- Support trimming
- Support Seed selection
- Support Power Schedule
