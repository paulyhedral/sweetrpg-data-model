# -*- coding: utf-8 -*-
__author__ = "Paul Schifferer <dm@sweetrpg.com>"
"""
"""

from sweetrpg_kv_objects.model.store import Store
from sweetrpg_kv_objects.db.store.schema import StoreSchema
import json
from datetime import datetime


store_json = """
{
    "_id": "this-is-ignored",
    "name": "test-store",
    "description": "A store for testing",
    "current_snapshot_id": "1",
    "created_at": "2021-09-13T07:55:00.001",
    "created_by": "test",
    "updated_at": "2021-09-13T07:55:00.001",
    "updated_by": "test"
}
"""
store_datetime = datetime(2021, 9, 13, 7, 55, 0, 1000)
store_dict = {
    "_id": "another-id",
    "name": "test-store",
    "description": "A store for testing",
    "current_snapshot_id": "1",
    "created_at": datetime(2021, 9, 15, 7, 35, 0, 2000),
    "created_by": "test",
    "updated_at": datetime(2021, 9, 15, 7, 35, 0, 2001),
    "updated_by": "test",
    "deleted_at": datetime(2021, 9, 15, 7, 35, 0, 2001),
    "deleted_by": "test",
}


def test_load_store_from_json():
    j = json.loads(store_json)
    schema = StoreSchema()
    a = schema.load(j)
    assert a is not None
    assert isinstance(a, Store)
    assert a.id == "this-is-ignored"
    assert a.name == "test-store"
    assert a.description == "A store for testing"
    assert a.current_snapshot_id == "1"
    assert a.created_at == store_datetime
    assert a.created_by == "test"
    assert a.created_at == store_datetime
    assert a.updated_by == "test"
    assert a.deleted_at is None
    assert a.deleted_by is None


def test_load_store_from_dict():
    schema = StoreSchema()
    a = schema.load(store_dict)
    assert a is not None
    assert isinstance(a, Store)
    assert a.id == "another-id"
    assert a.name == "test-store"
    assert a.description == "A store for testing"
    assert a.current_snapshot_id == "1"
    assert a.created_at == datetime(2021, 9, 15, 7, 35, 0, 2000)
    assert a.created_by == "test"
    assert a.updated_at == datetime(2021, 9, 15, 7, 35, 0, 2001)
    assert a.updated_by == "test"
    assert a.deleted_at == datetime(2021, 9, 15, 7, 35, 0, 2001)
    assert a.deleted_by == "test"
