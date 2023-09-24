# -*- coding: utf-8 -*-
__author__ = "Paul Schifferer <dm@sweetrpg.com>"
"""
"""

from sweetrpg_kv_objects.model.snapshot import Snapshot
import json
from datetime import datetime

snapshot_json = """
{
    "_id": "this-is-ignored",
    "name": "test-snapshot",
    "key_ids": ["1", "2"],
    "snapshot_ids": ["1", "2"],
    "current_snapshot_id": "1",
    "created_at": "2021-09-13T07:55:00.001",
    "created_by": "test",
    "updated_at": "2021-09-13T07:55:00.001",
    "updated_by": "test"
}
"""
snapshot_datetime = datetime(2021, 9, 13, 7, 55, 0, 1000)


def test_snapshot_from_json():
    j = json.loads(snapshot_json)
    a = Snapshot(**j)
    assert isinstance(a, Snapshot)
    assert a.id == "this-is-ignored"
    assert a.name == "test-snapshot"
    assert a.key_ids == ["1", "2"]
    assert a.snapshot_ids == ["1", "2"]
    assert a.current_snapshot_id == "1"
    assert a.created_at == snapshot_datetime
    assert a.created_by == "test"
    assert a.updated_at == snapshot_datetime
    assert a.updated_by == "test"
    assert not hasattr(a, "deleted_at") or a.deleted_at is None
    assert not hasattr(a, "deleted_by") or a.deleted_by is None
