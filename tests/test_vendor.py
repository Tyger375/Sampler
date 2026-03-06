import usb.core
import usb.util

# Use the IDs from your lsusb output
VID = 0x303a
PID = 0x4029

# Find the device
dev = usb.core.find(idVendor=VID, idProduct=PID)

if dev is None:
    print("Device not found! Check your USB cable/connection.")
    exit()

# If on Linux, detach the kernel driver for the Vendor interface (Interface 4)
if dev.is_kernel_driver_active(4):
    try:
        dev.detach_kernel_driver(4)
        print("Detached kernel driver from Interface 4")
    except usb.core.USBError as e:
        print(f"Could not detach: {e}")

# Claim the vendor interface
usb.util.claim_interface(dev, 4)

# Endpoint addresses based on your C++ Enum
# EPNUM_VENDOR_OUT = 5 (0x05)
# EPNUM_VENDOR_IN = 0x80 | 6 (0x86)
EP_OUT = 0x04
EP_IN  = 0x84

# Send some data
msg = "ECHO"
print(f"Sending: {msg}")
dev.write(EP_OUT, f"{msg}\n")

# Read the echo back (if your C++ code does an echo)
try:
    ret = dev.read(EP_IN, 64, timeout=1000)
    print(f"Received: {''.join([chr(x) for x in ret])}")
except Exception as e:
    print(f"Read failed: {e}")

# Clean up
usb.util.release_interface(dev, 4)
