# -*- coding: utf-8 -*-
__author__ = "Paul Schifferer <dm@sweetrpg.com>"
"""
"""

from sweetrpg_kv_objects.model.value import Value
from sweetrpg_kv_objects.db.value.schema import ValueSchema
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
value_dict = {
    "_id": "another-id",
    "key_id": "1",
    "snapshot_id": "2",
    "value": "3",
    "created_at": datetime(2021, 9, 15, 7, 35, 0, 2000),
    "created_by": "test",
    "updated_at": datetime(2021, 9, 15, 7, 35, 0, 2001),
    "updated_by": "test",
    "deleted_at": datetime(2021, 9, 15, 7, 35, 0, 2001),
    "deleted_by": "test",
}


def test_load_value_from_json():
    j = json.loads(value_json)
    schema = ValueSchema()
    a = schema.load(j)
    assert a is not None
    assert isinstance(a, Value)
    assert a.id == "this-is-ignored"
    assert a.key_id == "1"
    assert a.snapshot_id == "2"
    assert a.value == "3"
    assert a.created_at == value_datetime
    assert a.created_by == "test"
    assert a.created_at == value_datetime
    assert a.updated_by == "test"
    assert a.deleted_at is None
    assert a.deleted_by is None


def test_load_value_from_dict():
    schema = ValueSchema()
    a = schema.load(value_dict)
    assert a is not None
    assert isinstance(a, Value)
    assert a.id == "another-id"
    assert a.key_id == "1"
    assert a.snapshot_id == "2"
    assert a.value == "3"
    assert a.created_at == datetime(2021, 9, 15, 7, 35, 0, 2000)
    assert a.created_by == "test"
    assert a.updated_at == datetime(2021, 9, 15, 7, 35, 0, 2001)
    assert a.updated_by == "test"
    assert a.deleted_at == datetime(2021, 9, 15, 7, 35, 0, 2001)
    assert a.deleted_by == "test"
