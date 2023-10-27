# Roadmap

## Milestones

 * [ ] d-bus replacement
 * [ ] `#![no_std]` for all crates on the unix platform

## Language Bindings

 * [ ] C / C++
 * [ ] Python
 * [ ] Lua
 * [ ] Zig

## Gateways

 * [ ] zbus
 * [ ] sommr
 * [ ] data-rs
 * [ ] rustdds
 * [ ] zenoh
 * [ ] rumqtt

## Microservices

 * [ ] Service Discovery
 * [ ] Health Monitor
 * [ ] Introspection Service
 * [ ] System Monitor (show CPU load etc. like top)

## Communication

 * [x] publish subscribe
 * [x] events
 * [ ] Publish Subscribe with history
 * [ ] Request Response Messaging Pattern
 * [ ] Blackboard Messaging Pattern
 * [ ] Pipeline Messaging Pattern
 * [ ] PubSub, ReqRes, Pipeline variant that works with copies (poor mans mixed criticality)
 * [ ] Zero-copy GPU communication with Cuda, NvSci, Vulkan
 * [ ] Zero-copy across hypervisor partitions
 * [ ] Zero-copy via QEmu ivshmem: https://www.qemu.org/docs/master/system/devices/ivshmem.html

## Platform support

 * [ ] Android
 * [x] Linux
 * [x] Windows
 * [ ] Mac Os
 * [ ] iOS
 * [ ] WatchOS
 * [x] FreeBSD
 * [ ] FreeRTOS

## Framework Integration

 * [ ] ROS2 rmw binding
 * [ ] dora-rs integration

## Tooling

 * [ ] Basic command line introspection tooling
 * [ ] Tooling for advanced introspection, cool WebGUI
 * [ ] command line client as interface to microservices

## Safety & Security

 * [ ] Mixed Criticallity setup, e.g. applications do not interfer with each other
 * [ ] Sample Tracking when application crashes
 * [ ] Identity and Access Management, e.g. service that create additional services
 * [ ] Use Kani, Loom and Miri for tests of lock-free constructs

## Development

 * [ ] Tracing integration for advanced performance measurements, see google chrome chrome://tracing/ and flame graphs
       See lttng, add trace points to the source code on the important functions
