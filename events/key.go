package events

import "time"

// Key event.
// This object represents an event on a store's key.
type KeyEvent struct {
	ID         string    `json:"id"`
	StoreID    string    `json:"store_id"`
	Event      string    `json:"event"`
	Key        string    `json:"key"`
	OccurredAt time.Time `json:"occurred_at"`
}
