import socket
import subprocess
import pytest
import os
import os.path


@pytest.fixture
def console_server(request):
    """
    Run the statsd server in a subprocess.
    """
    return run_server(interval='1')


@pytest.fixture
def admin_server(request):
    """
    Run the statsd server in a subprocess
    with a slow flush interval
    """
    return run_server(interval='10000')


def run_server(interval=1):
    executable = os.path.join(
        os.path.dirname(__file__),
        '..',
        'target/debug/statsd')
    command = [executable, '--console', '--flush-interval', interval]
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


@pytest.fixture
def admin_client():
    """
    Get a client that connects
    to the management port.
    """
    host = '127.0.0.1'
    port = 8126
    return TcpClient(host, port)


class TcpClient(object):
    def __init__(self, host, port):
        sock = socket.socket(
            socket.AF_INET,
            socket.SOCK_STREAM)
        self.sock = sock
        self.host = host
        self.port = port

    def connect(self):
        self.sock.connect((self.host, self.port))

    def write(self, message):
        self.sock.send(message)

    def read(self, length=1024):
        return self.sock.recv(length)

    def close(self):
        self.sock.close()
