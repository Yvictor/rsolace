"""
壓力測試 set_user_prop 和 get_user_prop 的偶發性空字串問題
"""
import pytest
import pyrsolace
import time


def test_user_prop_ct_msgpack_repeated():
    """重複測試 ct/msgpack 的設置和獲取，嘗試重現偶發性空字串問題"""
    failures = []

    for i in range(1000):
        msg = pyrsolace.Msg(topic=f"test/topic/{i}", data=b"test data")

        # 設置 ct 為 msgpack
        try:
            msg.set_user_prop("ct", "msgpack")
        except Exception as e:
            failures.append(f"Iteration {i}: set_user_prop failed: {e}")
            continue

        # 立即獲取
        try:
            result = msg.get_user_prop("ct")
            if result != "msgpack":
                failures.append(f"Iteration {i}: Expected 'msgpack', got '{result}'")
        except Exception as e:
            failures.append(f"Iteration {i}: get_user_prop failed: {e}")

    # 報告所有失敗
    if failures:
        print(f"\n發現 {len(failures)} 個失敗：")
        for failure in failures[:10]:  # 只顯示前10個
            print(f"  {failure}")
        if len(failures) > 10:
            print(f"  ... 還有 {len(failures) - 10} 個失敗")

    assert len(failures) == 0, f"發現 {len(failures)} 個失敗案例"


def test_user_prop_multiple_keys_repeated():
    """測試多個 key 的重複設置和獲取"""
    failures = []

    for i in range(500):
        msg = pyrsolace.Msg(topic=f"test/topic/{i}", data=b"test data")

        # 設置多個屬性
        test_cases = [
            ("ct", "msgpack"),
            ("encoding", "utf-8"),
            ("version", "1.0.0"),
        ]

        try:
            for key, value in test_cases:
                msg.set_user_prop(key, value)

            # 驗證所有屬性
            for key, expected_value in test_cases:
                result = msg.get_user_prop(key)
                if result != expected_value:
                    failures.append(
                        f"Iteration {i}, key '{key}': Expected '{expected_value}', got '{result}'"
                    )
        except Exception as e:
            failures.append(f"Iteration {i}: Exception: {e}")

    if failures:
        print(f"\n發現 {len(failures)} 個失敗：")
        for failure in failures[:10]:
            print(f"  {failure}")
        if len(failures) > 10:
            print(f"  ... 還有 {len(failures) - 10} 個失敗")

    assert len(failures) == 0, f"發現 {len(failures)} 個失敗案例"


def test_user_prop_with_timing_variations():
    """測試不同時間間隔下的設置和獲取"""
    failures = []

    for i in range(100):
        msg = pyrsolace.Msg(topic=f"test/topic/{i}", data=b"test data")

        try:
            msg.set_user_prop("ct", "msgpack")

            # 在不同時間點獲取
            # 立即獲取
            result1 = msg.get_user_prop("ct")
            if result1 != "msgpack":
                failures.append(f"Iteration {i}, immediate: Expected 'msgpack', got '{result1}'")

            # 稍微延遲後獲取
            time.sleep(0.0001)
            result2 = msg.get_user_prop("ct")
            if result2 != "msgpack":
                failures.append(f"Iteration {i}, after delay: Expected 'msgpack', got '{result2}'")

        except Exception as e:
            failures.append(f"Iteration {i}: Exception: {e}")

    if failures:
        print(f"\n發現 {len(failures)} 個失敗：")
        for failure in failures:
            print(f"  {failure}")

    assert len(failures) == 0, f"發現 {len(failures)} 個失敗案例"


def test_user_prop_with_different_map_sizes():
    """測試不同 map_size 參數下的行為"""
    failures = []

    map_sizes = [1, 5, 10, 20, 50, 100]

    for map_size in map_sizes:
        for i in range(100):
            msg = pyrsolace.Msg(topic=f"test/topic/{i}", data=b"test data")

            try:
                msg.set_user_prop("ct", "msgpack", map_size=map_size)
                result = msg.get_user_prop("ct")

                if result != "msgpack":
                    failures.append(
                        f"map_size={map_size}, iteration {i}: "
                        f"Expected 'msgpack', got '{result}'"
                    )
            except Exception as e:
                failures.append(f"map_size={map_size}, iteration {i}: Exception: {e}")

    if failures:
        print(f"\n發現 {len(failures)} 個失敗：")
        for failure in failures[:10]:
            print(f"  {failure}")
        if len(failures) > 10:
            print(f"  ... 還有 {len(failures) - 10} 個失敗")

    assert len(failures) == 0, f"發現 {len(failures)} 個失敗案例"


def test_user_prop_empty_vs_missing():
    """區分空字串和缺失的 key"""
    failures = []

    for i in range(100):
        msg = pyrsolace.Msg(topic=f"test/topic/{i}", data=b"test data")

        try:
            # 設置空字串
            msg.set_user_prop("empty_key", "")
            result = msg.get_user_prop("empty_key")

            if result != "":
                failures.append(
                    f"Iteration {i}, empty_key: Expected '', got '{result}'"
                )

            # 設置非空值
            msg.set_user_prop("ct", "msgpack")
            result2 = msg.get_user_prop("ct")

            if result2 != "msgpack":
                failures.append(
                    f"Iteration {i}, ct: Expected 'msgpack', got '{result2}'"
                )

            # 確認 empty_key 仍然是空字串
            result3 = msg.get_user_prop("empty_key")
            if result3 != "":
                failures.append(
                    f"Iteration {i}, empty_key recheck: Expected '', got '{result3}'"
                )

        except Exception as e:
            failures.append(f"Iteration {i}: Exception: {e}")

    if failures:
        print(f"\n發現 {len(failures)} 個失敗：")
        for failure in failures[:10]:
            print(f"  {failure}")
        if len(failures) > 10:
            print(f"  ... 還有 {len(failures) - 10} 個失敗")

    assert len(failures) == 0, f"發現 {len(failures)} 個失敗案例"


def test_user_prop_special_values():
    """測試特殊值，包括可能導致問題的值"""
    failures = []

    special_values = [
        "msgpack",
        "bytes/msgpack",
        "application/msgpack",
        "m",  # 單字符
        "msgpack" * 100,  # 長字串
        "msgpack\n",  # 包含換行
        "msgpack ",  # 結尾空格
        " msgpack",  # 開頭空格
        "msg pack",  # 中間空格
        "中文msgpack",  # 混合中文
    ]

    for value in special_values:
        for i in range(50):
            msg = pyrsolace.Msg(topic=f"test/topic/{i}", data=b"test data")

            try:
                msg.set_user_prop("ct", value)
                result = msg.get_user_prop("ct")

                if result != value:
                    failures.append(
                        f"Value '{value[:20]}...', iteration {i}: "
                        f"Expected '{value[:20]}...', got '{result[:20] if result else 'EMPTY'}...'"
                    )
            except Exception as e:
                failures.append(f"Value '{value[:20]}...', iteration {i}: Exception: {e}")

    if failures:
        print(f"\n發現 {len(failures)} 個失敗：")
        for failure in failures[:10]:
            print(f"  {failure}")
        if len(failures) > 10:
            print(f"  ... 還有 {len(failures) - 10} 個失敗")

    assert len(failures) == 0, f"發現 {len(failures)} 個失敗案例"


@pytest.mark.parametrize("iteration", range(20))
def test_user_prop_ct_msgpack_parametrized(iteration):
    """參數化測試：多次運行相同測試來捕獲偶發性問題"""
    msg = pyrsolace.Msg(topic=f"test/topic/{iteration}", data=b"test data")

    # 設置 ct 為 msgpack
    msg.set_user_prop("ct", "msgpack")

    # 獲取並驗證
    result = msg.get_user_prop("ct")
    assert result == "msgpack", f"Iteration {iteration}: Expected 'msgpack', got '{result}'"

    # 再次獲取確認
    result2 = msg.get_user_prop("ct")
    assert result2 == "msgpack", f"Iteration {iteration} (2nd get): Expected 'msgpack', got '{result2}'"


if __name__ == "__main__":
    # 可以直接運行這個文件來快速測試
    print("運行壓力測試...")

    print("\n測試 1: ct/msgpack 重複測試 (1000次)")
    test_user_prop_ct_msgpack_repeated()
    print("✓ 通過")

    print("\n測試 2: 多個 key 重複測試 (500次)")
    test_user_prop_multiple_keys_repeated()
    print("✓ 通過")

    print("\n測試 3: 時間間隔測試 (100次)")
    test_user_prop_with_timing_variations()
    print("✓ 通過")

    print("\n測試 4: 不同 map_size 測試")
    test_user_prop_with_different_map_sizes()
    print("✓ 通過")

    print("\n測試 5: 空字串 vs 缺失 key 測試")
    test_user_prop_empty_vs_missing()
    print("✓ 通過")

    print("\n測試 6: 特殊值測試")
    test_user_prop_special_values()
    print("✓ 通過")

    print("\n所有壓力測試通過！")
