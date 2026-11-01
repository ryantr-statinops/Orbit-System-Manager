package telemetry

import (
	"context"
	gnet "github.com/shirou/gopsutil/v4/net"
	"testing"
	"time"
)

func TestNetworkRemovalAndReappearance(t *testing.T) {
	stats := []gnet.IOCountersStat{{Name: "one", BytesRecv: 100, BytesSent: 100}, {Name: "two", BytesRecv: 20, BytesSent: 20}}
	c := NetworkCollector{Read: func(context.Context, bool) ([]gnet.IOCountersStat, error) { return stats, nil }, Rates: RateTracker{Interval: time.Second}}
	at := time.Now()
	c.Collect(context.Background(), at)
	stats = stats[:1]
	stats[0].BytesRecv += 200
	g := c.Collect(context.Background(), at.Add(time.Second))
	if len(g.Devices) != 1 || *g.Devices[0].Metrics["receive_rate"].Value != 200 {
		t.Fatal(g)
	}
	stats = append(stats, gnet.IOCountersStat{Name: "two", BytesRecv: 1000})
	g = c.Collect(context.Background(), at.Add(2*time.Second))
	if g.Devices[1].Metrics["receive_rate"].Status != WarmingUp {
		t.Fatal("reappeared counter must not spike")
	}
}
