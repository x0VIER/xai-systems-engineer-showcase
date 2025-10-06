import os
import socket

print("Python script running...")

# Attempt to access the network
try:
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.connect(("www.google.com", 80))
    print("Network connection successful!")
except Exception as e:
    print(f"Network connection failed: {e}")

# Attempt to write to a restricted file
try:
    with open("/etc/passwd", "a") as f:
        f.write("test:x:1001:1001::/home/test:/bin/bash\n")
    print("File write successful!")
except Exception as e:
    print(f"File write failed: {e}")

print("Python script finished.")

