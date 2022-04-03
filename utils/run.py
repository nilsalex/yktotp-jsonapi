#!/usr/bin/env python3

import os
import platform
import subprocess
import sys

if len(sys.argv) < 2:
    print("no message provided")
    print("usage: run.py [message]")
    exit(1)

message = sys.argv[1]
message_raw = bytes(message, "UTF-8")
length = len(message_raw)
length_raw = length.to_bytes(4, byteorder=sys.byteorder, signed=False)

encoded_message = length_raw + message_raw

print(f"message: {message}")
print(f"encoded message: {encoded_message}")

exe_name = "yktotp-jsonapi.exe" if platform.system() == "Windows" else "yktotp-jsonapi"
exe_path = os.path.join(os.path.dirname(sys.argv[0]), "../target/release/", exe_name)
process = subprocess.run(os.path.abspath(exe_path),
                         text=True,
                         input=encoded_message.decode(),
                         capture_output=True)
response_raw = bytes(process.stdout, "UTF-8")

print(f"encoded response: {response_raw}")

response_length = int.from_bytes(response_raw[0:4], byteorder=sys.byteorder, signed=False)
response = response_raw[4:4 + response_length].decode()

print(f"response: {response}")
