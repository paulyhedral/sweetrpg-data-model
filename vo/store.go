package vo

import (
	modelcore "github.com/sweetrpg/model-core/vo"
)

// Store value object.
// This value object is a serializable representation of the Store model.
type StoreVO struct {
	ID              string            `json:"id" jsonapi:"primary,key"`
	Name            string            `json:"name" jsonapi:"attr,name"`
	Description     string            `json:"description" jsonapi:"attr,description"`
	CurrentSnapshot *SnapshotVO       `json:"current_snapshot" jsonapi:"relation,snapshot"`
	Notes           string            `json:"notes" jsonapi:"attr,notes"`
	Tags            []modelcore.TagVO `json:"tags" jsonapi:"attr,tags"`
	modelcore.AuditableVO
}
