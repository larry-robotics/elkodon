# Frequently Asked Questions

## How to use `log` or `tracing` as default log backend

 * **log**, add the feature flag `logger_log` to the dependency in `Cargo.toml`

  ```toml
  elkodon = { version = "0.1.0", features = ["logger_log"]}
  ```

 * **tracing**, add the feature flag `logger_tracing` to the dependency in `Cargo.toml`

 ```toml
  elkodon = { version = "0.1.0", features = ["logger_tracing"]}
 ```

## A crash leads to the failure `PublishSubscribeOpenError(UnableToOpenDynamicServiceInformation)`

**Note:** A command line tool and internal service is already planned to cleanup
resources from crashed applications, see issue #65.

When an application crashes some resources may remain in the system and must be
cleaned up manually. If this occurs, stop all services and remove manually all
shared memory segments and static service config files.

```sh
rm -rf /dev/shm/*
rm -rf /tmp/elkodon/*
```

If you cannot stop all running services, you can look up the `uuid` of the service
in question and remove the files manually.
Assume, the service `My/Funk/ServiceName` is corrupted. You can identify the static
config by grepping the service name in the `/tmp/elkodon/service` folder.

So the command
```sh
cd /tmp/elkodon/service
grep -RIne "My/Funk/ServiceName"
```
provides us with the output
```
25b25afeb7557886e9f69408151e018e268e5917.service:2:service_name = "My/Funk/ServiceName"
```

The file name corresponds with the `uuid` of the service. So removing the dynamic and
static service config with the following commands, removes the service completely from
the system.
```sh
# static service config
rm /tmp/elkodon/service/25b25afeb7557886e9f69408151e018e268e5917.service

# dynamic service config
rm /dev/shm/25b25afeb7557886e9f69408151e018e268e5917.dynamic
```

Be aware, that if an application with a publisher crashes, the data segment of the
publisher must be cleaned up as well.
