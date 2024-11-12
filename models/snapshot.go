package models

import (
	modelcore "github.com/sweetrpg/model-core/models"
)

// Snapshot model.
// This model represents a snapshot of keys and values in a store.
type Snapshot struct {
	ID      string          `bson:"_id" json:"id" jsonapi:"primary,snapshot"`
	StoreID string          `bson:"store_id" json:"store_id" jsonapi:"relation,store"`
	Name    string          `json:"name" jsonapi:"attr,name"`
	Notes   string          `json:"notes" jsonapi:"attr,notes"`
	Tags    []modelcore.Tag `json:"tags" jsonapi:"attr,tags"`
	modelcore.Auditable
}
