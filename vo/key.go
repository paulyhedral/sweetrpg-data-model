package vo

import (
	modelcore "github.com/sweetrpg/model-core/vo"
)

// Key value object.
// This value object is a serializable representation of the Key model.
type KeyVO struct {
	ID          string            `json:"id" jsonapi:"primary,key"`
	Store       *StoreVO          `json:"store,omitempty" jsonapi:"relation,store,omitempty"`
	Name        string            `json:"name" jsonapi:"attr,name"`
	Description string            `json:"description" jsonapi:"attr,description"`
	Type        string            `json:"type" jsonapi:"attr,type"`
	Encoding    string            `json:"encoding" jsonapi:"attr,encoding"`
	Expression  string            `json:"expression" jsonapi:"attr,expression"`
	Notes       string            `json:"notes" jsonapi:"attr,notes"`
	Tags        []modelcore.TagVO `json:"tags" jsonapi:"attr,tags"`
	modelcore.AuditableVO
}
