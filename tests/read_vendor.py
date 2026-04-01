import usb.core
import usb.util
import sys
import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation
import collections
# Testing vendor communication

VID = 0x303A
PID = 0x4029
INTERFACE = 4

dev = usb.core.find(idVendor=VID, idProduct=PID)

if dev is None:
    print("Device not found!")
    sys.exit()

# If on Linux, detach the kernel driver for the Vendor interface (Interface 4)
if sys.platform.startswith('linux'):
    if dev.is_kernel_driver_active(INTERFACE):
        dev.detach_kernel_driver(INTERFACE)
        print("Detached kernel driver")

# Claim the vendor interface
usb.util.claim_interface(dev, INTERFACE)

MAX_POINTS = 100
data_store = {}
line_objects = {}

fig, ax = plt.subplots()
ax.set_title("ESP32-S3 Real-time Vendor Data")
ax.set_xlabel("Time (Samples)")
ax.set_ylabel("Value")
ax.set_ylim(0, 4096)
ax.set_xlim(0, MAX_POINTS)

def parse_message(msg):
    # Parses '{index}: {value}' into (int, int)
    try:
        if ":" in msg:
            parts = msg.split(":")
            return int(parts[0].strip()), int(parts[1].strip())
    except ValueError:
        pass
    return None

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

def update(frame):
    raw_str = read_all_available(dev, EP_IN, timeout=10)
    if not raw_str:
        return line_objects.values()
    
    messages = raw_str.strip().split("\n")
    updated = False

    for msg in messages:
        parsed = parse_message(msg)
        if parsed and parsed[0] == 4: # Filter for Index 4
            idx, val = parsed
            updated = True

            if idx not in data_store:
                data_store[idx] = collections.deque([0]*MAX_POINTS, maxlen=MAX_POINTS)
                ln, = ax.plot([], [], label=f"Index {idx}")
                line_objects[idx] = ln
                ax.legend()

            data_store[idx].append(val)
            line_objects[idx].set_data(range(len(data_store[idx])), list(data_store[idx]))

    return line_objects.values()

ani = FuncAnimation(fig, update, interval=20, blit=True, cache_frame_data=False)
plt.show()

usb.util.release_interface(dev, 4)

if sys.platform.startswith("linux"):
    try:
        dev.attach_kernel_driver(4)
    except:
        pass
