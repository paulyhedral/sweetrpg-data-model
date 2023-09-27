# -*- coding: utf-8 -*-
__author__ = "Paul Schifferer <dm@sweetrpg.com>"
"""
"""

from sweetrpg_kv_objects.model.key import Key
from sweetrpg_kv_objects.db.key.schema import KeySchema
import json
from datetime import datetime


key_json = """
{
    "_id": "this-is-ignored",
    "store_id": "1",
    "name": "test-key",
    "description": "A key for testing",
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
key_dict = {
    "_id": "another-id",
    "store_id": "1",
    "name": "test-key",
    "description": "A key for testing",
    "type": "integer",
    "encoding": "plain",
    "expression": "",
    "created_at": datetime(2021, 9, 15, 7, 35, 0, 2000),
    "created_by": "test",
    "updated_at": datetime(2021, 9, 15, 7, 35, 0, 2001),
    "updated_by": "test",
    "deleted_at": datetime(2021, 9, 15, 7, 35, 0, 2001),
    "deleted_by": "test",
}


def test_load_key_from_json():
    j = json.loads(key_json)
    schema = KeySchema()
    a = schema.load(j)
    assert a is not None
    assert isinstance(a, Key)
    assert a.id == "this-is-ignored"
    assert a.store_id == "1"
    assert a.name == "test-key"
    assert a.description == "A key for testing"
    assert a.type == "integer"
    assert a.encoding == "plain"
    assert a.expression == ""
    assert a.created_at == key_datetime
    assert a.created_by == "test"
    assert a.created_at == key_datetime
    assert a.updated_by == "test"
    assert a.deleted_at is None
    assert a.deleted_by is None


def test_load_key_from_dict():
    schema = KeySchema()
    a = schema.load(key_dict)
    assert a is not None
    assert isinstance(a, Key)
    assert a.id == "another-id"
    assert a.store_id == "1"
    assert a.name == "test-key"
    assert a.description == "A key for testing"
    assert a.type == "integer"
    assert a.encoding == "plain"
    assert a.expression == ""
    assert a.created_at == datetime(2021, 9, 15, 7, 35, 0, 2000)
    assert a.created_by == "test"
    assert a.updated_at == datetime(2021, 9, 15, 7, 35, 0, 2001)
    assert a.updated_by == "test"
    assert a.deleted_at == datetime(2021, 9, 15, 7, 35, 0, 2001)
    assert a.deleted_by == "test"
