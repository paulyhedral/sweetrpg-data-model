# -*- coding: utf-8 -*-
__author__ = "Paul Schifferer <dm@sweetrpg.com>"
"""
"""

from sweetrpg_kv_objects.model.snapshot import Snapshot
from sweetrpg_kv_objects.db.snapshot.schema import SnapshotSchema
import json
from datetime import datetime


snapshot_json = """
{
    "_id": "this-is-ignored",
    "name": "test-snapshot",
    "value_ids": ["1", "2"],
    "created_at": "2021-09-13T07:55:00.001",
    "created_by": "test",
    "updated_at": "2021-09-13T07:55:00.001",
    "updated_by": "test"
}
"""
snapshot_datetime = datetime(2021, 9, 13, 7, 55, 0, 1000)
snapshot_dict = {
    "_id": "another-id",
    "name": "test-snapshot",
    "value_ids": ["1", "2"],
    "created_at": datetime(2021, 9, 15, 7, 35, 0, 2000),
    "created_by": "test",
    "updated_at": datetime(2021, 9, 15, 7, 35, 0, 2001),
    "updated_by": "test",
    "deleted_at": datetime(2021, 9, 15, 7, 35, 0, 2001),
    "deleted_by": "test",
}


def test_load_snapshot_from_json():
    j = json.loads(snapshot_json)
    schema = SnapshotSchema()
    a = schema.load(j)
    assert a is not None
    assert isinstance(a, Snapshot)
    assert a.id == "this-is-ignored"
    assert a.name == "test-snapshot"
    assert a.value_ids == ["1", "2"]
    assert a.created_at == snapshot_datetime
    assert a.created_by == "test"
    assert a.created_at == snapshot_datetime
    assert a.updated_by == "test"
    assert a.deleted_at is None
    assert a.deleted_by is None


def test_load_snapshot_from_dict():
    schema = SnapshotSchema()
    a = schema.load(snapshot_dict)
    assert a is not None
    assert isinstance(a, Snapshot)
    assert a.id == "another-id"
    assert a.name == "test-snapshot"
    assert a.value_ids == ["1", "2"]
    assert a.created_at == datetime(2021, 9, 15, 7, 35, 0, 2000)
    assert a.created_by == "test"
    assert a.updated_at == datetime(2021, 9, 15, 7, 35, 0, 2001)
    assert a.updated_by == "test"
    assert a.deleted_at == datetime(2021, 9, 15, 7, 35, 0, 2001)
    assert a.deleted_by == "test"
