package vo

import (
	modelcore "github.com/sweetrpg/model-core/vo"
)

// Snapshot value object.
// This value object is a serializable representation of the Snapshot model.
type SnapshotVO struct {
	ID    string            `json:"id" jsonapi:"primary,key"`
	Store *StoreVO          `json:"store,omitempty" jsonapi:"relation,store,omitempty"`
	Name  string            `json:"name" jsonapi:"attr,name"`
	Notes string            `json:"notes" jsonapi:"attr,notes"`
	Tags  []modelcore.TagVO `json:"tags" jsonapi:"attr,tags"`
	modelcore.AuditableVO
}
