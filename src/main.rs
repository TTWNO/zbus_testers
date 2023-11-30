use zbus_macros::DBusError;
use std::collections::HashMap;
use zbus::{zvariant::Value, dbus_proxy, dbus_interface, Result, SignalContext, proxy, interface};

struct Greeter;

#[dbus_interface(name = "org.zbus.MyGreeter1")]
impl Greeter {
    async fn say_hello(&self, name: &str) -> String {
        format!("Hello {}!", name)
    }
    #[dbus_interface(signal)]
    async fn greeted_everyone(ctxt: &SignalContext<'_>) -> Result<()>;
}

struct Greeter2;

#[interface(name = "org.zbus.MyGreeter1")]
impl Greeter2 {
    async fn say_hello(&self, name: &str) -> String {
        format!("Hello {}!", name)
    }
    #[zbus(signal)]
    async fn greeted_everyone(ctxt: &SignalContext<'_>) -> Result<()>;
}

#[dbus_proxy(
    default_service = "org.freedesktop.Notifications",
    default_path = "/org/freedesktop/Notifications"
)]
trait Notifications {
    /// Call the org.freedesktop.Notifications.Notify D-Bus method
    fn notify(&self,
              app_name: &str,
              replaces_id: u32,
              app_icon: &str,
              summary: &str,
              body: &str,
              actions: &[&str],
              hints: HashMap<&str, &Value<'_>>,
              expire_timeout: i32) -> zbus::Result<u32>;
}

#[proxy(
    default_service = "org.freedesktop.Notifications",
    default_path = "/org/freedesktop/Notifications"
)]
trait Notifications2 {
    /// Call the org.freedesktop.Notifications.Notify D-Bus method
    fn notify(&self,
              app_name: &str,
              replaces_id: u32,
              app_icon: &str,
              summary: &str,
              body: &str,
              actions: &[&str],
              hints: HashMap<&str, &Value<'_>>,
              expire_timeout: i32) -> zbus::Result<u32>;
}

#[derive(DBusError, Debug)]
#[dbus_error(prefix = "org.myservice.App")]
enum Error {
    #[dbus_error(zbus)]
    ZBus(zbus::Error),
    FileNotFound(String),
    OutOfMemory,
}

#[derive(DBusError, Debug)]
#[zbus(prefix = "org.myservice.App")]
enum Error2 {
    #[zbus(zbus)]
    ZBus(zbus::Error),
    FileNotFound(String),
    OutOfMemory,
}

fn main() {
    println!("Hello, world!");
}
