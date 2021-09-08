from subprocess import Popen
from os import mkdir
from os.path import expanduser, exists
from shutil import copytree, rmtree
from platform import system

print("Compiling Ferx...")

if system() != "Linux":
    print("Ah boomer, it's only for Linux")
    exit(1)

home_dir = expanduser("~")
process = Popen(["/bin/sh", "-c", "cargo build --release"])
process.wait()

if process.returncode != 0:
    print("Failed to compile!")
    exit(1)

print("Compilation done... Installing Ferx")

# check if ~/.config/ferx exists
if exists(f"{home_dir}/.config/ferx"):
    print("Detected previous ferx installation")
    ans = input("Want to remove it?[y/N] ")
    if ans in "Yy":
        rmtree(f"{home_dir}/.config/ferx")
    else:
        exit(1)

# try to make ~/.config/ferx
print("Making ferx directory in ~/.config...")
try:
    mkdir(f"{home_dir}/.config/ferx")
except Exception as e:
    print(f"Failed to install Ferx. Reason: {e}")
    exit(1)

# now copy arts to ferx/arts
print("Copying arts...")
try:
    copytree("arts", f"{home_dir}/.config/ferx/arts")
except Exception as e:
    print(f"Failed to install Ferx. Reason: {e}")
    exit(1)

# now copy ferx binary to /usr/bin
try:
    print("Copying ferx binary to /usr/bin")
    process = Popen(["/bin/sudo", "cp", "target/release/ferx", "/usr/bin/"])
    process.wait()

    if process.returncode != 0:
        print("Failed to copy binary!")
        exit(1)

except Exception as e:
    print(f"Failed to install Ferx. Reason: {e}")
    exit(1)

print("Installation done!")
