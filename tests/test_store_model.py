# -*- coding: utf-8 -*-
__author__ = "Paul Schifferer <dm@sweetrpg.com>"
"""
"""

from sweetrpg_kv_objects.model.store import Store
import json
from datetime import datetime

store_json = """
{
    "_id": "this-is-ignored",
    "name": "test-store",
    "description": "A store for testing",
    "created_at": "2021-09-13T07:55:00.001",
    "created_by": "test",
    "updated_at": "2021-09-13T07:55:00.001",
    "updated_by": "test"
}
"""
store_datetime = datetime(2021, 9, 13, 7, 55, 0, 1000)


def test_store_from_json():
    j = json.loads(store_json)
    a = Store(**j)
    assert isinstance(a, Store)
    assert a.id == "this-is-ignored"
    assert a.name == "test-store"
    assert a.description == "A store for testing"
    assert a.created_at == store_datetime
    assert a.created_by == "test"
    assert a.updated_at == store_datetime
    assert a.updated_by == "test"
    assert not hasattr(a, "deleted_at") or a.deleted_at is None
    assert not hasattr(a, "deleted_by") or a.deleted_by is None
