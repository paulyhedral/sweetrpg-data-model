package models

import (
	modelcore "github.com/sweetrpg/model-core/models"
)

// Key model.
// This model represents a key in a store of key-value information.
type Key struct {
	ID          string          `bson:"_id" json:"id"`
	StoreID     string          `bson:"store_id" json:"store_id"`
	Name        string          `json:"name"`
	Description string          `json:"description"`
	Type        string          `json:"type"`
	Encoding    string          `json:"encoding"`
	Expression  string          `json:"expression"`
	Notes       string          `json:"notes"`
	Tags        []modelcore.Tag `json:"tags"`
	modelcore.Auditable
}
