"""
Tests for pyrsolace Event and SessionEvent classes
"""
import pytest
from pyrsolace import SessionEvent


def test_session_event_constants():
    """Test that SessionEvent constants are available"""
    # Test that we can access SessionEvent constants
    assert hasattr(SessionEvent, 'UpNotice')
    assert hasattr(SessionEvent, 'DownError')
    assert hasattr(SessionEvent, 'ConnectFailedError')


def test_session_event_equality():
    """Test SessionEvent equality comparison"""
    event1 = SessionEvent.UpNotice
    event2 = SessionEvent.UpNotice
    assert event1 == event2


def test_session_event_different_types():
    """Test different SessionEvent types are not equal"""
    up_event = SessionEvent.UpNotice
    down_event = SessionEvent.DownError
    assert up_event != down_event


@pytest.mark.parametrize("event_type", [
    SessionEvent.UpNotice, 
    SessionEvent.DownError, 
    SessionEvent.ConnectFailedError, 
    SessionEvent.SubscriptionOk, 
    SessionEvent.CanSend
])
def test_session_event_types(event_type):
    """Test various SessionEvent types"""
    assert event_type is not None
    # Test that the event has a name attribute
    assert hasattr(event_type, 'name')
