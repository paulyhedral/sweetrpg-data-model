# -*- coding: utf-8 -*-
__author__ = "Paul Schifferer <dm@sweetrpg.com>"
"""
"""

from sweetrpg_kv_objects.model.value import Value
import json
from datetime import datetime

value_json = """
{
    "_id": "this-is-ignored",
    "key_id": "1",
    "snapshot_id": "2",
    "value": "3",
    "created_at": "2021-09-13T07:55:00.001",
    "created_by": "test",
    "updated_at": "2021-09-13T07:55:00.001",
    "updated_by": "test"
}
"""
value_datetime = datetime(2021, 9, 13, 7, 55, 0, 1000)


def test_value_from_json():
    j = json.loads(value_json)
    a = Value(**j)
    assert isinstance(a, Value)
    assert a.id == "this-is-ignored"
    assert a.key_id == "1"
    assert a.snapshot_id == "2"
    assert a.value == "3"
    assert a.created_at == value_datetime
    assert a.created_by == "test"
    assert a.updated_at == value_datetime
    assert a.updated_by == "test"
    assert not hasattr(a, "deleted_at") or a.deleted_at is None
    assert not hasattr(a, "deleted_by") or a.deleted_by is None
