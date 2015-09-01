import pytest
import time


def test_admin_help(admin_client, admin_server):
    time.sleep(1)
    admin_client.connect()
    admin_client.write('help\n')
    output = admin_client.read()

    admin_client.write("quit\n")
    assert 'Admin Console' in output
    assert 'quit' in output
    assert 'stats' in output
    admin_server.kill()


def test_admin_timers(admin_client, client, admin_server):
    time.sleep(1)
    client('some.metric:1.50|c')

    admin_client.connect()
    admin_client.write('counters\n')
    output = admin_client.read()

    assert 'some.metric' in output
    admin_server.kill()
