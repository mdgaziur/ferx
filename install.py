from subprocess import Popen
from os import mkdir, remove, getuid
from os.path import exists
from shutil import copytree, rmtree, copy
from platform import system

if getuid() != 0:
    print("Run this script as root and add env variables if needed")
    exit(1)

print("Compiling Ferx...")

if system() != "Linux":
    print("Ah boomer, it's only for Linux")
    exit(1)

process = Popen(["/bin/sh", "-c", "cargo build --release"])
process.wait()

if process.returncode != 0:
    print("Failed to compile!")
    exit(1)

print("Compilation done... Installing Ferx")

# check if /opt/ferx exists
if exists("/opt/ferx"):
    print("Detected previous ferx installation")
    ans = input("Want to remove it?[y/N] ")
    if ans in "Yy":
        rmtree("/opt/ferx") # binary will get replaced automatically :3
    else:
        exit(1)

# try to make /opt/ferx
print("Making ferx directory in /opt...")
try:
    mkdir("/opt/ferx")
except Exception as e:
    print(f"Failed to install Ferx. Reason: {e}")
    exit(1)

# now copy arts to ferx/arts
print("Copying arts...")
try:
    copytree("arts", "/opt/ferx/arts")
except Exception as e:
    print(f"Failed to install Ferx. Reason: {e}")
    exit(1)

# now copy ferx binary to /usr/bin
print("Copying ferx binary to /usr/bin")
try:
    copy("target/release/ferx", "/usr/bin")

except Exception as e:
    print(f"Failed to install Ferx. Reason: {e}")
    exit(1)

print("Installation done!")
