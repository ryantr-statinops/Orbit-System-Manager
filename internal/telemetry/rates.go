package telemetry

import "time"

type counterBaseline struct {
	at     time.Time
	values []uint64
}

// RateTracker belongs to one collector and is not shared between goroutines.
type RateTracker struct {
	Interval time.Duration
	previous map[string]counterBaseline
}

func (r *RateTracker) Observe(id string, counters []uint64, at time.Time) ([]float64, Status, string) {
	if r.previous == nil {
		r.previous = map[string]counterBaseline{}
	}
	prev, exists := r.previous[id]
	r.previous[id] = counterBaseline{at, append([]uint64(nil), counters...)}
	if !exists || len(prev.values) != len(counters) {
		return nil, WarmingUp, "first counter baseline"
	}
	elapsed := at.Sub(prev.at)
	if elapsed <= 0 || elapsed > 3*r.Interval {
		return nil, WarmingUp, "sampling gap; baseline reset"
	}
	rates := make([]float64, len(counters))
	for i, current := range counters {
		if current < prev.values[i] {
			return nil, WarmingUp, "counter reset"
		}
		rates[i] = float64(current-prev.values[i]) / elapsed.Seconds()
	}
	return rates, Available, ""
}

// Retain removes disappeared devices, so reappearance always warms up.
func (r *RateTracker) Retain(ids []string) {
	keep := make(map[string]bool, len(ids))
	for _, id := range ids {
		keep[id] = true
	}
	for id := range r.previous {
		if !keep[id] {
			delete(r.previous, id)
		}
	}
}

func (r *RateTracker) Reset() { r.previous = nil }
