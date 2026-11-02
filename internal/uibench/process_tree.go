package uibench

import (
	"errors"

	"github.com/shirou/gopsutil/v4/process"
)

type processNode interface {
	PID() int32
	MemoryRSS() (uint64, error)
	Children() ([]processNode, error)
}

type TreeMemory struct {
	RSSBytes     uint64
	ProcessCount int
	Complete     bool
}

func MeasureProcessTree(root processNode) TreeMemory {
	result := TreeMemory{Complete: root != nil}
	if root == nil {
		return result
	}

	queue := []processNode{root}
	seen := make(map[int32]struct{})
	for len(queue) > 0 {
		current := queue[0]
		queue = queue[1:]
		pid := current.PID()
		if _, exists := seen[pid]; exists {
			continue
		}
		seen[pid] = struct{}{}
		result.ProcessCount++

		rss, err := current.MemoryRSS()
		if err != nil {
			result.Complete = false
		} else {
			result.RSSBytes += rss
		}

		children, err := current.Children()
		if err != nil {
			result.Complete = false
			continue
		}
		queue = append(queue, children...)
	}
	return result
}

type gopsutilNode struct {
	process *process.Process
}

func (n gopsutilNode) PID() int32 {
	return n.process.Pid
}

func (n gopsutilNode) MemoryRSS() (uint64, error) {
	info, err := n.process.MemoryInfo()
	if err != nil {
		return 0, err
	}
	if info == nil {
		return 0, errors.New("process memory info unavailable")
	}
	return info.RSS, nil
}

func (n gopsutilNode) Children() ([]processNode, error) {
	children, err := n.process.Children()
	if err != nil {
		return nil, err
	}
	result := make([]processNode, 0, len(children))
	for _, child := range children {
		result = append(result, gopsutilNode{process: child})
	}
	return result, nil
}

func NewProcessNode(pid int32) (processNode, error) {
	processInfo, err := process.NewProcess(pid)
	if err != nil {
		return nil, err
	}
	return gopsutilNode{process: processInfo}, nil
}
