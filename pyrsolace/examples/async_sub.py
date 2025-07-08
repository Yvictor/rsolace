import os
import asyncio
import pyrsolace
import msgpack
import dotenv


async def main():
    dotenv.load_dotenv()
    host = os.getenv("SOLACE_HOST")
    vpn = os.getenv("SOLACE_VPN")
    username = os.getenv("SOLACE_USERNAME")
    password = os.getenv("SOLACE_PASSWORD")
    if not host or not vpn or not username or not password:
        raise ValueError("HOST, VPN, USERNAME, PASSWORD must be set")

    client = pyrsolace.Client()
    # pyrsolace.init_tracing_logger(pyrsolace.LogLevel.Debug)
    event_receiver = client.get_async_event_receiver()

    async def event_handle_loop():
        while True:
            event = await event_receiver.recv()
            print(event)

    asyncio.create_task(event_handle_loop())

    client.connect(
        host=host,
        vpn=vpn,
        username=username,
        password=password,
        client_name="pyrsolace-sub-async",
    )
    msg_receiver = client.get_async_msg_receiver()
    # client.subscribe("TIC/v1/*/*/*/*")
    client.subscribe_ext("TIC/v1/*/*/*/*", pyrsolace.SubscribeFlag.RequestConfirm)

    async def msg_handle_loop():
        while True:
            msg = await msg_receiver.recv()
            print(msg.topic)
            print(msgpack.loads(msg.data))
            del msg

    asyncio.create_task(msg_handle_loop())
    # await asyncio.sleep(30)
    await asyncio.sleep(10)

    # # client.unsubscribe("TIC/v1/*/*/*/*")
    client.unsubscribe_ext("TIC/v1/*/*/*/*", pyrsolace.SubscribeFlag.RequestConfirm)
    client.disconnect()


if __name__ == "__main__":
    asyncio.run(main())
