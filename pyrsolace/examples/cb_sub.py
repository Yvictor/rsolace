import os
import time
import pyrsolace
import msgpack
import dotenv


def event_callback(event: pyrsolace.Event):
    print(event)


def msg_callback(msg: pyrsolace.Msg):
    print(msg.topic)
    print(msgpack.loads(msg.data))
    del msg


def main():
    dotenv.load_dotenv()
    host = os.getenv("SOLACE_HOST")
    vpn = os.getenv("SOLACE_VPN")
    username = os.getenv("SOLACE_USERNAME")
    password = os.getenv("SOLACE_PASSWORD")
    if not host or not vpn or not username or not password:
        raise ValueError("HOST, VPN, USERNAME, PASSWORD must be set")

    client = pyrsolace.Client()
    client.set_event_callback(event_callback)
    client.set_msg_callback(msg_callback)
    client.connect(
        host=host,
        vpn=vpn,
        username=username,
        password=password,
        client_name="pyrsolace-sub-cb",
    )
    client.subscribe("TIC/v1/*/*/*/*")
    time.sleep(10)
    client.unsubscribe("TIC/v1/*/*/*/*")
    client.disconnect()


if __name__ == "__main__":
    main()
