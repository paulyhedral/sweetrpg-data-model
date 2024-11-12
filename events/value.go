package events

import "time"

// Value event.
// This object represents a change in a key's value.
type ValueEvent struct {
	ID         string    `json:"id"`
	StoreID    string    `json:"store_id"`
	Event      string    `json:"event"`
	Key        string    `json:"key"`
	OccurredAt time.Time `json:"occurred_at"`
}
