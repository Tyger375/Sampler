import usb.core
import usb.util
import sys
import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation
import collections
import re
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
    if msg.count(':') > 1:
        return None
    try:
        if ":" in msg:
            parts = msg.split(":")
            return int(parts[0].strip()), int(parts[1].strip())
    except ValueError:
        pass
    return None

EP_OUT = 0x04
EP_IN  = 0x84

def read_line(dev, endpoint):
    full_data = []
    while True:
        try:
            data = dev.read(endpoint, 64, timeout=200)
            print(full_data)
        except usb.core.USBError as e:
            if e.errno == 110 or e.backend_error_code == -7:
                break
            else:
                raise e

def read_all_available(dev, endpoint, chunk_size=64, timeout=200):
    full_data = []
    while True:
        try:
            # Read a chunk
            data = dev.read(endpoint, chunk_size, timeout=timeout)
            full_data.extend(data)

            print(data)
            
            # If the packet is 'short' (less than 64), it's the end of the transfer
            if len(data) < chunk_size:
                break
        except usb.core.USBError as e:
            if e.errno == 110 or e.backend_error_code == -7:
                break
            else:
                raise e
            
    return ''.join([chr(x) for x in full_data])

filter = int(input("filter: "))

TUPLE_REGEX = r"\(([0-9]+)[,]?[ ]?([0-9]+)\)"
def parse_tuple(s: str) -> tuple[int, int]:
    try:
        res = re.fullmatch(TUPLE_REGEX, s)
        if res == None:
            return None
        
        first = res.group(1)
        second = res.group(2)
        return (int(first), int(second))
    except:
        return None
    
def parse_msg(raw: str) -> list[tuple[int, tuple[int, int]]]:
    try:
        values = []
        for x in raw.split(";"):
            s = x.split(":")
            if len(s) != 2:
                continue
            
            value = parse_tuple(s[1].strip())
            if value == None:
                continue
            values.append((int(s[0]), value))
        return values
    except:
        return []

def append_data_store(idx, value, trigger: bool):
    suffix = " - Trigger" if trigger else ""
    if idx not in data_store:
        data_store[idx] = collections.deque([0]*MAX_POINTS, maxlen=MAX_POINTS)
        ln, = ax.plot([], [], label=f"Index {idx}{suffix}")
        line_objects[idx] = ln
        ax.legend()

    data_store[idx].append(value)
    line_objects[idx].set_data(range(len(data_store[idx])), list(data_store[idx]))

def update(frame):
    try:
        chunk = dev.read(EP_IN, 64, timeout=200)
    except:
        return line_objects.values()
    raw_str = ''.join([chr(x) for x in chunk])
    values = parse_msg(raw_str)
    # print(values)

    for (idx, item) in values:
        if idx != filter:
            continue

        val = item[0]
        midi_type = item[1]

        append_data_store(idx, val, False)
        append_data_store(idx + 16, midi_type * 500, True)

    return line_objects.values()
    """
    raw_str = read_all_available(dev, EP_IN, timeout=200)
    if not raw_str:
        return line_objects.values()
    """
    
    messages = raw_str.strip().split("\n")
    updated = False

    for msg in messages:
        print(msg)
        parsed = parse_message(msg)
        if parsed and parsed[0] == filter: # Filter for Index
            print(parsed, msg)
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
