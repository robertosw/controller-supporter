# Usage

- `Controller  <--USB-->  Raspberry Pi  <--USB-->  Platform`
- Try to 1:1 Copy what both Controller and Platform send
  - Capture Handshake to be able to fake a connected controller afterwards
  - Capture changes to how both ends respond after knowing what has been connected

<br>

# Commands

- `sudo cat /sys/kernel/debug/usb/devices` shows all connected usb devices with all available information
> Example for PS5 Controller connected via usb:
> ```Rust
> T:  Bus=03 Lev=01 Prnt=01 Port=03 Cnt=02 Dev#=  5 Spd=480  MxCh= 0
> D:  Ver= 2.00 Cls=00(>ifc ) Sub=00 Prot=00 MxPS=64 #Cfgs=  1
> P:  Vendor=054c ProdID=0ce6 Rev= 1.00
> S:  Manufacturer=Sony Interactive Entertainment
> S:  Product=Wireless Controller
> C:* #Ifs= 4 Cfg#= 1 Atr=c0 MxPwr=500mA
>
> I:* If#= 0 Alt= 0 #EPs= 0 Cls=01(audio) Sub=01 Prot=00 Driver=snd-usb-audio
> I:* If#= 1 Alt= 0 #EPs= 0 Cls=01(audio) Sub=02 Prot=00 Driver=snd-usb-audio
> I:  If#= 1 Alt= 1 #EPs= 1 Cls=01(audio) Sub=02 Prot=00 Driver=snd-usb-audio
> E:  Ad=01(O) Atr=09(Isoc) MxPS= 392 Ivl=1ms
>
> I:* If#= 2 Alt= 0 #EPs= 0 Cls=01(audio) Sub=02 Prot=00 Driver=snd-usb-audio
> I:  If#= 2 Alt= 1 #EPs= 1 Cls=01(audio) Sub=02 Prot=00 Driver=snd-usb-audio
> E:  Ad=82(I) Atr=05(Isoc) MxPS= 196 Ivl=1ms
>
> I:* If#= 3 Alt= 0 #EPs= 2 Cls=03(HID  ) Sub=00 Prot=00 Driver=usbhid
> E:  Ad=84(I) Atr=03(Int.) MxPS=  64 Ivl=4ms
> E:  Ad=03(O) Atr=03(Int.) MxPS=  64 Ivl=4ms
> ```
> - `Bus=03` USB bus number
> - `Prnt=01` Parent device
> - `Cnt=02` How many devices are on this usb port
> - `Dev#=  5` device number
> - `Spd=480` speed in Mbit/s (so USB 2.0)
> - `Ver= 2.00` usb version
> - `Cls=00(>ifc )` device class
> - `MxPS=64` maximum package size
> - `#Cfgs=  1` number of configurations
> - `Vendor=054c` might be nececarry for connections to PlayStation 4 / 5
> - `#Ifs= 4` number of interfaces
> - `Atr=c0` attributes
> - `MxPwr=500mA` max power draw
> - `Ivl=4ms` interval
> - `Ad=84(I)` address IN (or O for OUT)

<br>

# Possible HowTo
The goal is, that the Raspberry Pi is recognized as an simple device using the HID protocol to communicate, since this is what controllers do.
To be useful as a middleman, reading raw HID inputs from the actual controller has to be possible.

The HID protocol consists of a host and client. Usually the controller is the client and the platform you are playing on is the host. 
In this case the raspi has to support both roles. It has to be the host for the actual controller and has to be a client, pretending to be a controller, for the platform.

Possible rust crates for HID:
- [hid-io-protocol](https://crates.io/crates/hid-io-protocol/0.1.5) Seems to support both roles
- [usbd-hid](https://crates.io/crates/usbd-hid) might be too high-level
- [rusb](https://crates.io/crates/rusb) only for reading (being host)
- [hidapi](https://crates.io/crates/hidapi)
- [hidg](https://lib.rs/crates/hidg) might only be useful for sending data as client

Other possibly useful things:
- [USB Gadget Drivers in Linux](https://www.kernel.org/doc/html/v4.19/driver-api/usb/gadget.html)
- [usbmon](https://docs.kernel.org/usb/usbmon.html)
