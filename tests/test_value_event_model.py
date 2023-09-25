# -*- coding: utf-8 -*-
__author__ = "Paul Schifferer <dm@sweetrpg.com>"
"""
"""

from sweetrpg_kv_objects.model.value_event import ValueEvent
import json
from datetime import datetime

event_json = """
{
    "store": "test-store",
    "key": "test-key",
    "occurred": "2021-09-13T07:55:00.001"
}
"""
event_datetime = datetime(2021, 9, 13, 7, 55, 0, 1000)


def test_event_from_json():
    j = json.loads(event_json)
    a = ValueEvent(**j)
    assert isinstance(a, ValueEvent)
    assert a.store == "test-store"
    assert a.key == "test-key"
    assert a.occurred == event_datetime
