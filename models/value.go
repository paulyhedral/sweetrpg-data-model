package models

import (
	modelcore "github.com/sweetrpg/model-core/models"
)

// Value model.
// This model represents a store of key-value information.
type Value struct {
	ID         string          `bson:"_id" json:"id" jsonapi:"primary,value"`
	StoreID    string          `bson:"store_id" json:"store_id" jsonapi:"relation,store"`
	KeyID      string          `bson:"key_id" json:"key_id" jsonapi:"relation,key"`
	SnapshotID string          `bson:"snapshot_id" json:"snapshot_id" jsonapi:"relation,snapshot"`
	Value      string          `json:"value" jsonapi:"attr,value"`
	Notes      string          `json:"notes" jsonapi:"attr,notes"`
	Tags       []modelcore.Tag `json:"tags" jsonapi:"attr,tags"`
	modelcore.Auditable
}
