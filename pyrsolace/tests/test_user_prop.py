"""
測試 pyrsolace Msg 類別中的 set_user_prop 和 get_user_prop 功能
"""
import pytest
import pyrsolace


def test_user_prop_basic():
    """測試基本的設置和獲取 user property"""
    msg = pyrsolace.Msg(topic="test/topic", data=b"test data")

    # 設置單個 user property
    msg.set_user_prop("key1", "value1")

    # 獲取 user property
    result = msg.get_user_prop("key1")
    assert result == "value1"


def test_user_prop_multiple():
    """測試設置多個 user properties"""
    msg = pyrsolace.Msg(topic="test/topic", data=b"test data")

    # 設置多個 user properties
    msg.set_user_prop("key1", "value1")
    msg.set_user_prop("key2", "value2")
    msg.set_user_prop("key3", "value3")

    # 獲取所有 user properties
    assert msg.get_user_prop("key1") == "value1"
    assert msg.get_user_prop("key2") == "value2"
    assert msg.get_user_prop("key3") == "value3"


def test_user_prop_overwrite():
    """測試覆蓋已存在的 user property

    注意：Solace container map 不支持直接覆蓋已存在的 key。
    如果嘗試添加同一個 key，舊值會被保留。
    """
    msg = pyrsolace.Msg(topic="test/topic", data=b"test data")

    # 設置 user property
    msg.set_user_prop("key1", "value1")
    assert msg.get_user_prop("key1") == "value1"

    # 嘗試覆蓋同一個 key - 會保留舊值
    msg.set_user_prop("key1", "new_value")
    # Solace 不支持覆蓋，所以仍然是舊值
    assert msg.get_user_prop("key1") == "value1"


def test_user_prop_nonexistent_key():
    """測試獲取不存在的 key"""
    msg = pyrsolace.Msg(topic="test/topic", data=b"test data")

    # 獲取不存在的 key 應該拋出異常
    with pytest.raises(Exception):
        msg.get_user_prop("nonexistent")


def test_user_prop_empty_key():
    """測試空字串 key

    注意：Solace 底層庫不接受空字串作為 key，會返回 Fail。
    """
    msg = pyrsolace.Msg(topic="test/topic", data=b"test data")

    # 設置空字串作為 key 會失敗
    with pytest.raises(Exception) as exc_info:
        msg.set_user_prop("", "value")

    # 驗證錯誤訊息包含 Fail
    assert "Fail" in str(exc_info.value)


def test_user_prop_empty_value():
    """測試空字串 value"""
    msg = pyrsolace.Msg(topic="test/topic", data=b"test data")

    # 設置空字串作為 value
    msg.set_user_prop("key", "")

    # 獲取應該返回空字串
    result = msg.get_user_prop("key")
    assert result == ""


def test_user_prop_special_characters():
    """測試特殊字符"""
    msg = pyrsolace.Msg(topic="test/topic", data=b"test data")

    # 測試各種特殊字符
    special_cases = [
        ("key-with-dash", "value-with-dash"),
        ("key_with_underscore", "value_with_underscore"),
        ("key.with.dot", "value.with.dot"),
        ("key/with/slash", "value/with/slash"),
        ("中文key", "中文value"),
        ("key with space", "value with space"),
    ]

    for key, value in special_cases:
        msg.set_user_prop(key, value)
        result = msg.get_user_prop(key)
        assert result == value, f"Failed for key={key}, value={value}"


def test_user_prop_long_strings():
    """測試長字串"""
    msg = pyrsolace.Msg(topic="test/topic", data=b"test data")

    # 測試長 key 和長 value
    long_key = "k" * 1000
    long_value = "v" * 10000

    msg.set_user_prop(long_key, long_value)
    result = msg.get_user_prop(long_key)
    assert result == long_value


def test_user_prop_map_size():
    """測試不同的 map_size 參數"""
    msg = pyrsolace.Msg(topic="test/topic", data=b"test data")

    # 測試不同的 map_size
    for map_size in [1, 10, 50, 100]:
        msg_temp = pyrsolace.Msg(topic="test/topic", data=b"test")
        msg_temp.set_user_prop("key", "value", map_size=map_size)
        result = msg_temp.get_user_prop("key")
        assert result == "value", f"Failed for map_size={map_size}"


def test_user_prop_null_byte_in_string():
    """測試字串中包含空字符的情況（應該拋出異常而不是 panic）"""
    msg = pyrsolace.Msg(topic="test/topic", data=b"test data")

    # 字串中包含 null byte 應該拋出異常而不是 panic
    with pytest.raises(Exception) as exc_info:
        msg.set_user_prop("key\x00with\x00null", "value")

    # 驗證是正常的異常，而不是 PanicException
    assert "PanicException" not in str(type(exc_info.value))

    # 測試 value 中包含 null byte
    with pytest.raises(Exception) as exc_info:
        msg.set_user_prop("key", "value\x00with\x00null")

    assert "PanicException" not in str(type(exc_info.value))


def test_user_prop_before_and_after_reset():
    """測試 reset 後 user properties 的行為"""
    msg = pyrsolace.Msg(topic="test/topic", data=b"test data")

    # 設置 user property
    msg.set_user_prop("key1", "value1")
    assert msg.get_user_prop("key1") == "value1"

    # Reset 訊息
    msg.reset()

    # Reset 後，user property 應該被清除，嘗試獲取應該拋出異常
    with pytest.raises(Exception):
        msg.get_user_prop("key1")


def test_user_prop_with_content_type():
    """測試實際使用場景：設置 content type

    注意：由於 Solace 不支持覆蓋已存在的 key，
    每個訊息應該使用不同的 key，或使用新的訊息對象。
    """
    # 使用分離的訊息對象來測試不同的 content type
    msg1 = pyrsolace.Msg(topic="test/topic", data=b"test data")
    msg1.set_user_prop("content-type", "application/json")
    assert msg1.get_user_prop("content-type") == "application/json"

    msg2 = pyrsolace.Msg(topic="test/topic", data=b"test data")
    msg2.set_user_prop("content-type", "application/msgpack")
    assert msg2.get_user_prop("content-type") == "application/msgpack"

    msg3 = pyrsolace.Msg(topic="test/topic", data=b"test data")
    msg3.set_user_prop("encoding", "utf-8")
    assert msg3.get_user_prop("encoding") == "utf-8"


def test_user_prop_multiple_messages():
    """測試多個訊息之間的 user properties 是獨立的"""
    msg1 = pyrsolace.Msg(topic="test/topic1", data=b"data1")
    msg2 = pyrsolace.Msg(topic="test/topic2", data=b"data2")

    # 在不同的訊息上設置不同的 user properties
    msg1.set_user_prop("key", "value1")
    msg2.set_user_prop("key", "value2")

    # 確認它們是獨立的
    assert msg1.get_user_prop("key") == "value1"
    assert msg2.get_user_prop("key") == "value2"


@pytest.mark.parametrize("key,value", [
    ("ct", "bytes/msgpack"),
    ("encoding", "utf-8"),
    ("version", "1.0.0"),
    ("timestamp", "2024-01-01T00:00:00Z"),
    ("custom-header", "custom-value"),
])
def test_user_prop_parametrized(key, value):
    """參數化測試多種 key-value 組合"""
    msg = pyrsolace.Msg(topic="test/topic", data=b"test data")
    msg.set_user_prop(key, value)
    assert msg.get_user_prop(key) == value


def test_user_prop_stress_test():
    """壓力測試：設置大量的 user properties"""
    msg = pyrsolace.Msg(topic="test/topic", data=b"test data")

    # 設置大量的 user properties
    num_props = 100
    for i in range(num_props):
        key = f"key_{i}"
        value = f"value_{i}"
        msg.set_user_prop(key, value, map_size=200)  # 使用更大的 map_size

    # 驗證所有的 properties 都能正確獲取
    for i in range(num_props):
        key = f"key_{i}"
        expected_value = f"value_{i}"
        actual_value = msg.get_user_prop(key)
        assert actual_value == expected_value, f"Failed at index {i}"
