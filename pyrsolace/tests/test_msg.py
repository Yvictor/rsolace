"""
Tests for pyrsolace Msg class
"""
import pytest
import pyrsolace


def test_msg_creation():
    """Test basic message creation"""
    msg = pyrsolace.Msg(topic="test/topic", data=b"")
    assert msg.topic == "test/topic"


def test_msg_with_data():
    """Test message creation with data"""
    data = b"hello world"
    msg = pyrsolace.Msg(topic="test/topic", data=data)
    assert msg.topic == "test/topic"
    assert msg.data == data


def test_msg_with_correlation_id():
    """Test message with correlation ID"""
    msg = pyrsolace.Msg(topic="test/topic", data=b"", corr_id="test-123")
    assert msg.topic == "test/topic"
    assert msg.corr_id == "test-123"


@pytest.mark.parametrize("topic,data,corr_id", [
    ("simple/topic", b"test data", "corr-1"),
    ("another/topic", b"different data", "corr-2"),
    ("test/reply", b"reply message", None),
])
def test_msg_various_combinations(topic, data, corr_id):
    """Test message creation with various parameter combinations"""
    msg = pyrsolace.Msg(topic=topic, data=data, corr_id=corr_id)
    assert msg.topic == topic
    assert msg.data == data
    assert msg.corr_id == corr_id


def test_msg_with_all_parameters():
    """Test message creation with all parameters"""
    msg = pyrsolace.Msg(
        topic="test/topic",
        data=b"test data",
        corr_id="test-123",
        reply_topic="test/reply",
        is_reply=False,
        eligible=True,
        cos=1,
        is_delivery_to_one=True
    )

    assert msg.topic == "test/topic"
    assert msg.data == b"test data"
    assert msg.corr_id == "test-123"
    assert msg.reply_topic == "test/reply"
