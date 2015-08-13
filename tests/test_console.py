import time


def test_console_output(client, console_server):
    time.sleep(1)
    client('some.metric:1.23|c')
    time.sleep(2)

    console_server.kill()
    output = console_server.stdout.read()
    assert 'Flushing metrics' in output
    assert 'some.metric: 1.23' in output
