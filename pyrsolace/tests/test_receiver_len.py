"""
測試 receiver 的 len() 方法
"""
import pytest
import pyrsolace
import asyncio


def test_msg_receiver_len():
    """測試 MsgReceiver 的 len() 方法"""
    client = pyrsolace.Client()

    # 獲取 receiver
    msg_receiver = client.get_msg_receiver()

    # 初始應該是空的
    initial_len = msg_receiver.len()
    print(f"初始 queue 長度: {initial_len}")
    assert initial_len == 0, f"初始長度應該是 0，實際是 {initial_len}"

    # 使用 len() == 0 來判斷是否為空
    assert msg_receiver.len() == 0, "初始應該是空的"

    print("✓ MsgReceiver len() 測試通過")


def test_event_receiver_len():
    """測試 EventReceiver 的 len() 方法"""
    client = pyrsolace.Client()

    # 獲取 receiver
    event_receiver = client.get_event_receiver()

    # 初始應該是空的
    initial_len = event_receiver.len()
    print(f"初始 event queue 長度: {initial_len}")
    assert initial_len == 0, f"初始長度應該是 0，實際是 {initial_len}"
    assert event_receiver.len() == 0, "初始應該是空的"

    print("✓ EventReceiver len() 測試通過")


def test_async_msg_receiver_len():
    """測試 AsyncMsgReceiver 的 len() 方法"""
    client = pyrsolace.Client()

    # 獲取 async receiver
    async_msg_receiver = client.get_async_msg_receiver()

    # 初始應該是空的
    initial_len = async_msg_receiver.len()
    print(f"初始 async queue 長度: {initial_len}")
    assert initial_len == 0, f"初始長度應該是 0，實際是 {initial_len}"
    assert async_msg_receiver.len() == 0, "初始應該是空的"

    print("✓ AsyncMsgReceiver len() 測試通過")


def test_async_event_receiver_len():
    """測試 AsyncEventReceiver 的 len() 方法"""
    client = pyrsolace.Client()

    # 獲取 async receiver
    async_event_receiver = client.get_async_event_receiver()

    # 初始應該是空的
    initial_len = async_event_receiver.len()
    print(f"初始 async event queue 長度: {initial_len}")
    assert initial_len == 0, f"初始長度應該是 0，實際是 {initial_len}"
    assert async_event_receiver.len() == 0, "初始應該是空的"

    print("✓ AsyncEventReceiver len() 測試通過")


def test_receiver_types():
    """測試所有類型的 receiver"""
    client = pyrsolace.Client()

    # 測試所有 receiver 類型
    receivers = {
        "msg_receiver": client.get_msg_receiver(),
        "request_receiver": client.get_request_receiver(),
        "p2p_receiver": client.get_p2p_receiver(),
        "event_receiver": client.get_event_receiver(),
        "async_msg_receiver": client.get_async_msg_receiver(),
        "async_request_receiver": client.get_async_request_receiver(),
        "async_p2p_receiver": client.get_async_p2p_receiver(),
        "async_event_receiver": client.get_async_event_receiver(),
    }

    for name, receiver in receivers.items():
        # 檢查是否有 len() 方法
        assert hasattr(receiver, 'len'), f"{name} 應該有 len() 方法"

        # 調用方法
        length = receiver.len()
        is_empty = (length == 0)

        print(f"{name}: len={length}, is_empty={is_empty}")

        # 初始應該是空的
        assert is_empty == True, f"{name} 初始應該是空的"
        assert length == 0, f"{name} 初始長度應該是 0"

    print("✓ 所有 receiver 類型測試通過")


@pytest.mark.asyncio
async def test_async_receiver_usage_example():
    """展示如何使用 AsyncMsgReceiver 的 len() 方法"""
    client = pyrsolace.Client()

    # 獲取 async receiver
    receiver = client.get_async_msg_receiver()

    print("\n使用範例：")
    print(f"當前 queue 中有 {receiver.len()} 個消息")

    if receiver.len() == 0:
        print("Queue 是空的，沒有待處理的消息")
    else:
        print(f"Queue 中還有 {receiver.len()} 個消息待處理")

    # 模擬監控 queue 狀態
    for i in range(3):
        queue_len = receiver.len()
        print(f"檢查 {i+1}: queue 長度 = {queue_len}")
        await asyncio.sleep(0.1)

    print("✓ 使用範例完成")


if __name__ == "__main__":
    print("=" * 60)
    print("測試 Receiver len() 方法")
    print("=" * 60)

    print("\n測試 1: MsgReceiver")
    print("-" * 60)
    test_msg_receiver_len()

    print("\n測試 2: EventReceiver")
    print("-" * 60)
    test_event_receiver_len()

    print("\n測試 3: AsyncMsgReceiver")
    print("-" * 60)
    test_async_msg_receiver_len()

    print("\n測試 4: AsyncEventReceiver")
    print("-" * 60)
    test_async_event_receiver_len()

    print("\n測試 5: 所有 Receiver 類型")
    print("-" * 60)
    test_receiver_types()

    print("\n測試 6: 使用範例")
    print("-" * 60)
    asyncio.run(test_async_receiver_usage_example())

    print("\n" + "=" * 60)
    print("所有測試完成！")
    print("=" * 60)
