import socket
import subprocess
import pytest
import os
import os.path


@pytest.fixture
def console_server():
    """
    Run the statsd server in a subprocess.
    """
    executable = os.path.join(
        os.path.dirname(__file__),
        '..',
        'target/debug/statsd')
    command = [executable, '--console', '--flush-interval=1']
    process = subprocess.Popen(
        command,
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        shell=False,
        universal_newlines=True)
    return process


@pytest.fixture
def client():
    """
    Get a client stub that can be used
    to push metrics to the server.
    """
    host = '127.0.0.1'
    port = 8125
    sock = socket.socket(
        socket.AF_INET,
        socket.SOCK_DGRAM)
    sock.connect((host, port))
    def send(msg):
        sock.sendall(msg)
    return send
