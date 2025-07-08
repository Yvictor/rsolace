import os
import time
import pyrsolace
import msgpack
import dotenv
from concurrent.futures import ThreadPoolExecutor


def main():
    dotenv.load_dotenv()
    host = os.getenv("SOLACE_HOST")
    vpn = os.getenv("SOLACE_VPN")
    username = os.getenv("SOLACE_USERNAME")
    password = os.getenv("SOLACE_PASSWORD")
    if not host or not vpn or not username or not password:
        raise ValueError("HOST, VPN, USERNAME, PASSWORD must be set")

    client = pyrsolace.Client()
    event_receiver = client.get_event_receiver()
    msg_receiver = client.get_msg_receiver()

    def event_handle_loop(event_receiver: pyrsolace.EventReceiver):
        while True:
            event = event_receiver.recv()
            print(event)

    def msg_handle_loop(msg_receiver: pyrsolace.MsgReceiver):
        while True:
            msg = msg_receiver.recv()
            print(msg.topic)
            print(msgpack.loads(msg.data))
            del msg

    executor = ThreadPoolExecutor(max_workers=2)

    executor.submit(event_handle_loop, event_receiver)
    executor.submit(msg_handle_loop, msg_receiver)

    client.connect(
        host=host,
        vpn=vpn,
        username=username,
        password=password,
        client_name="pyrsolace-sub-sync",
    )
    client.subscribe_ext("TIC/v1/*/*/*/*", pyrsolace.SubscribeFlag.RequestConfirm)
    time.sleep(10)
    client.unsubscribe_ext("TIC/v1/*/*/*/*", pyrsolace.SubscribeFlag.RequestConfirm)
    client.disconnect()


if __name__ == "__main__":
    main()
