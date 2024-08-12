/*
 *  cmd/help.go
 *
 *  Copyright (c) 2024 RadianOS Development
 *  Copyright (c) 2024 by Atiksh Sharma <rudy@system-linux.com>
 *
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; either version 2 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
)

var helpCmd = &cobra.Command{
	Use:   "help",
	Short: "Show help information",
	Run: func(cmd *cobra.Command, args []string) {
		help_banner()
		fmt.Println("Here are the available commands:")
		listCommands(rootCmd, "")
	},
}

func listCommands(cmd *cobra.Command, indent string) {

	fmt.Printf("%s%s\n", indent, cmd.Use)
	for _, subCmd := range cmd.Commands() {
		listCommands(subCmd, indent+"  ")
	}
}

func help_banner() {
	banner := `
 ____  ____   ____ _____     __ __    ___  _      ____
|    ||    \ |    / ___/    |  |  |  /  _]| |    |    \
 |  | |  D  ) |  (   \_     |  |  | /  [_ | |    |  o  )
 |  | |    /  |  |\__  |    |  _  ||    _]| |___ |   _/
 |  | |    \  |  |/  \ |    |  |  ||   [_ |     ||  |
 |  | |  .  \ |  |\    |    |  |  ||     ||     ||  |
|____||__|\_||____|\___|    |__|__||_____||_____||__|
    `
	fmt.Println(banner)
}
