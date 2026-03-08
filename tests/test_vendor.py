import usb.core
import usb.util
import sys

# Testing vendor communication

VID = 0x303A
PID = 0x4029

dev = usb.core.find(idVendor=VID, idProduct=PID)

if dev is None:
    print("Device not found!")
    sys.exit()

# dev.set_configuration()

if dev is None:
    print("Device not found!")
    exit()

# If on Linux, detach the kernel driver for the Vendor interface (Interface 4)
if sys.platform.startswith('linux'):
    if dev.is_kernel_driver_active(4):
        dev.detach_kernel_driver(4)
        print("Detached kernel driver")

# Claim the vendor interface
usb.util.claim_interface(dev, 4)

EP_OUT = 0x04
EP_IN  = 0x84

def read_all_available(dev, endpoint, chunk_size=64, timeout=200):
    full_data = []
    while True:
        try:
            # Read a chunk
            data = dev.read(endpoint, chunk_size, timeout=timeout)
            full_data.extend(data)
            
            # If the packet is 'short' (less than 64), it's the end of the transfer
            if len(data) < chunk_size:
                break
        except usb.core.USBError as e:
            if e.errno == 110 or e.backend_error_code == -7:
                break
            else:
                raise e
            
    return ''.join([chr(x) for x in full_data])

try:
    while True:
        msg = input("MSG: ")
        if msg == "break":
            break
        dev.write(EP_OUT, f"{msg}\n")

        print(f"Received: {read_all_available(dev, EP_IN)}")
finally:
    usb.util.release_interface(dev, 4)

    if sys.platform.startswith("linux"):
        try:
            dev.attach_kernel_driver(4)
        except:
            pass
