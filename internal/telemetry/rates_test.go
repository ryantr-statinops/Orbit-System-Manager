package telemetry

import (
	"testing"
	"time"
)

func TestCounterBaselines(t *testing.T) {
	r := RateTracker{Interval: time.Second}
	at := time.Unix(1, 0)
	if _, s, _ := r.Observe("a", []uint64{100}, at); s != WarmingUp {
		t.Fatal(s)
	}
	v, s, _ := r.Observe("a", []uint64{400}, at.Add(1500*time.Millisecond))
	if s != Available || v[0] != 200 {
		t.Fatalf("%v %s", v, s)
	}
	if _, s, _ := r.Observe("a", []uint64{1}, at.Add(2*time.Second)); s != WarmingUp {
		t.Fatal("reset", s)
	}
	if _, s, _ := r.Observe("a", []uint64{2}, at.Add(6*time.Second)); s != WarmingUp {
		t.Fatal("gap", s)
	}
	if _, s, _ := r.Observe("a", []uint64{3}, at.Add(6*time.Second)); s != WarmingUp {
		t.Fatal("zero time", s)
	}
	r.Retain(nil)
	if _, s, _ := r.Observe("a", []uint64{100}, at.Add(7*time.Second)); s != WarmingUp {
		t.Fatal("reappearance", s)
	}
}

func TestLargeCounterDeltaPreservesPrecision(t *testing.T) {
	r := RateTracker{Interval: time.Second}
	at := time.Now()
	r.Observe("a", []uint64{1 << 60}, at)
	v, s, _ := r.Observe("a", []uint64{(1 << 60) + 3}, at.Add(time.Second))
	if s != Available || v[0] != 3 {
		t.Fatalf("%v %s", v, s)
	}
}
