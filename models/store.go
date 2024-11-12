package models

import (
	modelcore "github.com/sweetrpg/model-core/models"
)

// Store model.
// This model represents a store of key-value information.
type Store struct {
	ID                string          `bson:"_id" json:"id" jsonapi:"primary,store"`
	Name              string          `json:"name" jsonapi:"attr,name"`
	Description       string          `json:"description" jsonapi:"attr,description"`
	CurrentSnapshotID string          `bson:"current_snapshot_id" json:"current_snapshot_id" jsonapi:"relation,snapshot"`
	Notes             string          `json:"notes" jsonapi:"attr,notes"`
	Tags              []modelcore.Tag `json:"tags" jsonapi:"attr,tags"`
	modelcore.Auditable
}
