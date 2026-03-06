import usb.core
import usb.util

# Testing vendor communication

VID = 0x303A
PID = 0x4029

dev = usb.core.find(idVendor=VID, idProduct=PID)

if dev is None:
    print("Device not found!")
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

EP_OUT = 0x04
EP_IN  = 0x84

msg = "ECHO"
print(f"Sending: {msg}")
dev.write(EP_OUT, f"{msg}\n")

try:
    ret = dev.read(EP_IN, 64, timeout=1000)
    print(f"Received: {''.join([chr(x) for x in ret])}")
except Exception as e:
    print(f"Read failed: {e}")

# Clean up
usb.util.release_interface(dev, 4)
