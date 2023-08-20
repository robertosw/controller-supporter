/// Using linux' ConfigFS, create a new usb gadget
pub fn configure_gadget() {}

/*
    #!/bin/bash
    cd /sys/kernel/config/usb_gadget/
    mkdir -p raspi
    cd raspi
    echo 0x1d6b > idVendor # Linux Foundation
    echo 0x0104 > idProduct # Multifunction Composite Gadget
    echo 0x0100 > bcdDevice # v1.0.0
    echo 0x0200 > bcdUSB # USB2
    mkdir -p strings/0x409
    echo "fedcba9876543210" > strings/0x409/serialnumber
    echo "Robert Oswald" > strings/0x409/manufacturer
    echo "Raspberry Pi Zero 2 as USB Device" > strings/0x409/product
    mkdir -p configs/c.1/strings/0x409
    echo "Config 1: ECM network" > configs/c.1/strings/0x409/configuration
    echo 500 > configs/c.1/MaxPower

    # Add functions here
    mkdir -p functions/hid.usb0
    echo 0 > functions/hid.usb0/protocol
    echo 0 > functions/hid.usb0/subclass
    echo 64 > functions/hid.usb0/report_length
    echo -ne \\x05\\x01\\x09\\x06\\xa1\\x01\\x05\\x07\\x19\\xe0\\x29\\xe7\\x15\\x00\\x25\\x01\\x75\\x01\\x95\\x08\\x81\\x02\\x95\>
    ln -s functions/hid.usb0 configs/c.1/
    # End functions

    ls /sys/class/udc > UDC
*/

/*
   mkdir -p /sys/kernel/config/usb_gadget/g1
   echo "${bcd_usb}" > /sys/kernel/config/usb_gadget/g1/bcdUSB
   echo 0x0200 > /sys/kernel/config/usb_gadget/g1/bcdUSB
   echo "${device_class}" > /sys/kernel/config/usb_gadget/g1/bDeviceClass
   echo "${device_sub_class}" > /sys/kernel/config/usb_gadget/g1/bDeviceSubClass
   echo "${device_protocol}" > /sys/kernel/config/usb_gadget/g1/bDeviceProtocol
   echo 0x120 > /sys/kernel/config/usb_gadget/g1/idVendor
   echo "${product_id}" > /sys/kernel/config/usb_gadget/g1/idProduct
   echo "${bcd_device}" > /sys/kernel/config/usb_gadget/g1/bcdDevice
   mkdir -p /sys/kernel/config/usb_gadget/g1/strings/0x409
   echo "${manufacturer}" > /sys/kernel/config/usb_gadget/g1/strings/0x409/manufacturer
   echo "${product}" > /sys/kernel/config/usb_gadget/g1/strings/0x409/product
   echo "HID Keyboard" > /sys/kernel/config/usb_gadget/g1/strings/0x409/configuration
*/

/*
   mkdir -p /sys/kernel/config/usb_gadget/g1/functions/hid.usb0
   echo "1" > /sys/kernel/config/usb_gadget/g1/functions/hid.usb0/protocol
   echo "1" > /sys/kernel/config/usb_gadget/g1/functions/hid.usb0/subclass
   echo "${hid_report_length}" > /sys/kernel/config/usb_gadget/g1/functions/hid.usb0/report_length
   echo "${hid_report_descriptor}" > /sys/kernel/config/usb_gadget/g1/functions/hid.usb0/report_desc
   mkdir -p /sys/kernel/config/usb_gadget/g1/configs/c.1/strings/0x409
   echo "Config 1: HID Keyboard" > /sys/kernel/config/usb_gadget/g1/configs/c.1/strings/0x409/configuration
   echo 250 > /sys/kernel/config/usb_gadget/g1/configs/c.1/MaxPower
   ln -s /sys/kernel/config/usb_gadget/g1/functions/hid.usb0 /sys/kernel/config/usb_gadget/g1/configs/c.1/
 */
