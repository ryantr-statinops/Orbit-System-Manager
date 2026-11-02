package uibench

import (
	"errors"
	"testing"
)

type fakeProcess struct {
	pid         int32
	rss         uint64
	children    []processNode
	memoryErr   error
	childrenErr error
}

func (p *fakeProcess) PID() int32 { return p.pid }

func (p *fakeProcess) MemoryRSS() (uint64, error) {
	return p.rss, p.memoryErr
}

func (p *fakeProcess) Children() ([]processNode, error) {
	return p.children, p.childrenErr
}

func TestMeasureProcessTreeSumsEachProcessOnce(t *testing.T) {
	root := &fakeProcess{pid: 1, rss: 100}
	child := &fakeProcess{pid: 2, rss: 200}
	grandchild := &fakeProcess{pid: 3, rss: 300}
	root.children = []processNode{child, child}
	child.children = []processNode{grandchild}

	got := MeasureProcessTree(root)
	if got.RSSBytes != 600 || got.ProcessCount != 3 || !got.Complete {
		t.Fatalf("MeasureProcessTree() = %+v, want 600 bytes across 3 complete processes", got)
	}
}

func TestMeasureProcessTreeReportsPartialRead(t *testing.T) {
	root := &fakeProcess{pid: 1, rss: 100}
	child := &fakeProcess{pid: 2, rss: 0, memoryErr: errors.New("process exited")}
	root.children = []processNode{child}

	got := MeasureProcessTree(root)
	if got.RSSBytes != 100 || got.ProcessCount != 2 || got.Complete {
		t.Fatalf("MeasureProcessTree() = %+v, want partial 100-byte result", got)
	}
}

func TestMeasureProcessTreeReportsUnavailableRoot(t *testing.T) {
	got := MeasureProcessTree(nil)
	if got.ProcessCount != 0 || got.RSSBytes != 0 || got.Complete {
		t.Fatalf("MeasureProcessTree(nil) = %+v, want empty incomplete result", got)
	}
}
