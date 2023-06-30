package command

import "log"

// ValidateCommand is a Command implementation that validates factory files.
type ValidateCommand struct {
}

func (c *ValidateCommand) Run(rawArgs []string) int {
	log.Printf("Running validate command with args: %v", rawArgs)
	return 0
}
