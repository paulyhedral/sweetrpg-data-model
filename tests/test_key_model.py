# -*- coding: utf-8 -*-
__author__ = "Paul Schifferer <dm@sweetrpg.com>"
"""
"""

from sweetrpg_kv_objects.model.key import Key
import json
from datetime import datetime

key_json = """
{
    "_id": "this-is-ignored",
    "name": "test-key",
    "description": "A key for testing",
    "value_ids": ["1", "2"],
    "type": "integer",
    "encoding": "plain",
    "expression": "",
    "created_at": "2021-09-13T07:55:00.001",
    "created_by": "test",
    "updated_at": "2021-09-13T07:55:00.001",
    "updated_by": "test"
}
"""
key_datetime = datetime(2021, 9, 13, 7, 55, 0, 1000)


def test_key_from_json():
    j = json.loads(key_json)
    a = Key(**j)
    assert isinstance(a, Key)
    assert a.id == "this-is-ignored"
    assert a.name == "test-key"
    assert a.description == "A key for testing"
    assert a.value_ids == ["1", "2"]
    assert a.type == "integer"
    assert a.encoding == "plain"
    assert a.expression == ""
    assert a.created_at == key_datetime
    assert a.created_by == "test"
    assert a.updated_at == key_datetime
    assert a.updated_by == "test"
    assert not hasattr(a, "deleted_at") or a.deleted_at is None
    assert not hasattr(a, "deleted_by") or a.deleted_by is None
