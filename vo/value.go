package vo

import (
	modelcore "github.com/sweetrpg/model-core/vo"
)

// Value value object.
// This value object is a serializable representation of the Value model.
type ValueVO struct {
	ID       string            `json:"id" jsonapi:"primary,key"`
	Store    *StoreVO          `json:"store,omitempty" jsonapi:"relation,store,omitempty"`
	Key      *KeyVO            `json:"key,omitempty" jsonapi:"relation,key,omitempty"`
	Snapshot *SnapshotVO       `json:"snapshot,omitempty" jsonapi:"relation,snapshot,omitempty"`
	Value    string            `json:"value" jsonapi:"attr,value"`
	Notes    string            `json:"notes" jsonapi:"attr,notes"`
	Tags     []modelcore.TagVO `json:"tags" jsonapi:"attr,tags"`
	modelcore.AuditableVO
}
