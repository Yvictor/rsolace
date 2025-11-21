"""
測試自動初始化功能
"""
import pyrsolace


def test_msg_creation_without_explicit_client():
    """
    測試：在不創建 Client 的情況下創建 Msg
    應該自動初始化
    """
    print("\n測試不創建 Client 直接創建 Msg...")

    # 不創建 Client，直接創建 Msg
    msg = pyrsolace.Msg(topic="test/topic", data=b"test data")

    print(f"✓ Msg 創建成功：topic={msg.topic}")
    assert msg.topic == "test/topic"
    assert msg.data == b"test data"


def test_set_user_prop_without_explicit_client():
    """
    測試：在不創建 Client 的情況下使用 set_user_prop
    應該自動初始化
    """
    print("\n測試不創建 Client 直接使用 set_user_prop...")

    # 不創建 Client
    msg = pyrsolace.Msg(topic="test/topic", data=b"test data")

    # 嘗試 set_user_prop
    try:
        result = msg.set_user_prop("ct", "msgpack", 10)
        print(f"set_user_prop 返回: {result}")

        value = msg.get_user_prop("ct")
        print(f"get_user_prop 返回: '{value}'")

        assert value == "msgpack", f"期望 'msgpack'，但得到 '{value}'"
        print("✓ set_user_prop 和 get_user_prop 都成功（自動初始化有效）")

    except Exception as e:
        print(f"✗ 失敗: {e}")
        raise


def test_multiple_msgs_without_client():
    """
    測試：創建多個 Msg 不需要 Client
    """
    print("\n測試創建多個 Msg...")

    msgs = []
    for i in range(10):
        msg = pyrsolace.Msg(topic=f"test/topic/{i}", data=f"data{i}".encode())
        msg.set_user_prop("index", str(i), 10)
        msgs.append(msg)

    print(f"✓ 成功創建 {len(msgs)} 個消息")

    # 驗證
    for i, msg in enumerate(msgs):
        assert msg.topic == f"test/topic/{i}"
        assert msg.get_user_prop("index") == str(i)

    print("✓ 所有消息的 user properties 都正確")


if __name__ == "__main__":
    print("=" * 60)
    print("測試自動初始化功能")
    print("=" * 60)

    test_msg_creation_without_explicit_client()
    test_set_user_prop_without_explicit_client()
    test_multiple_msgs_without_client()

    print("\n" + "=" * 60)
    print("✓ 所有自動初始化測試通過！")
    print("=" * 60)
