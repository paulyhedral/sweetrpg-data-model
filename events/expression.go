package events

import "time"

// Expression event.
// This object represents a change in a key's expression.
type ExpressionEvent struct {
	ID         string    `json:"id"`
	StoreID    string    `json:"store_id"`
	Event      string    `json:"event"`
	Key        string    `json:"key"`
	OccurredAt time.Time `json:"occurred_at"`
}
