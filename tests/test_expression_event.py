# -*- coding: utf-8 -*-
__author__ = "Paul Schifferer <dm@sweetrpg.com>"
"""
Test cases for ExpressionEvent.
"""

from sweetrpg_kv_objects.events.expression import ExpressionEvent
import json
from datetime import datetime

event_json = """
{
    "store": "test-store",
    "event": "test-event",
    "key": "test-key",
    "occurred_at": "2021-09-13T07:55:00.001"
}
"""
event_datetime = datetime(2021, 9, 13, 7, 55, 0, 1000)


def test_event_from_json():
    j = json.loads(event_json)
    a = ExpressionEvent(**j)
    assert isinstance(a, ExpressionEvent)
    assert a.store == "test-store"
    assert a.event == "test-event"
    assert a.key == "test-key"
    assert a.occurred_at == event_datetime
