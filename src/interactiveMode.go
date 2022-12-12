package main

import (
	"fmt"
	"os"
	"strings"

	"github.com/rrossmiller/gocliselect"
)

const (
	CREATE  = "create"
	DELETE  = "delete"
	ACIVATE = "activate"
	NEW     = "new"
)

func InteractiveMode(args []string) {
	menu := gocliselect.NewMenu("Pick one")
	menu.VimKeys = true

	menu.AddItem("Acivate", ACIVATE)
	menu.AddItem("Create", CREATE)
	menu.AddItem("Delete", DELETE)

	choice := menu.Display()
	gocliselect.ClearScreen(len(menu.MenuItems) + 1)
	CheckChoice(choice)

	var cmd string
	var name string
	// show existing venvs
	if choice != CREATE {
		// add functionality to list venvs
		venvs, err := os.ReadDir(home + "/.venv")
		Check(err)

		// first entry of venv is the dir itself
		if len(venvs) < 2 {
			fmt.Printf("There are no venvs to %s\n", strings.ToLower(choice))
			os.Exit(72) // exit codes: https://www.freebsd.org/cgi/man.cgi?query=sysexits&apropos=0&sektion=0&manpath=FreeBSD+14.0-CURRENT&arch=default&format=pdf
		}

		venvMenu := gocliselect.NewMenu("Existing venvs")
		venvMenu.VimKeys = true
		for _, v := range venvs {
			if v.IsDir() {
				// make new menu with venvs as options. first option being type in a venv name
				venvMenu.AddItem(v.Name(), v.Name())
			}
		}
		name = venvMenu.Display()
		CheckChoice(choice)

	} else {
		gocliselect.ClearScreen(len(menu.MenuItems) + 1)
		name = GetInput("Name of venv: ")
	}

	switch choice {
	case CREATE:
		// todo ask user to specify python version if more than one exists
		cmd = fmt.Sprintf("python3 -m venv %s/%s", VENV_PATH, name)
		// Ask user if they wan to activate the venv now
		activateMenu := gocliselect.NewMenu("Existing venvs")
		activateMenu.VimKeys = true
		activateMenu.AddItem("Yes, activate", "yes")
		activateMenu.AddItem("No", "no")
		act := activateMenu.Display()
		CheckChoice(act)
		if act == "yes" {
			cmd += fmt.Sprintf("; source %s/%s/bin/activate", VENV_PATH, name)
		}
		fmt.Printf("Creating %s\n", name)

	case DELETE:
		err := os.RemoveAll(fmt.Sprintf("%s/%s", VENV_PATH, name))
		Check(err)
		cmd = fmt.Sprintf("echo Deleting %s/%s\n", VENV_PATH, name)

	case ACIVATE:
		fmt.Printf("Activating %s/%s\n", VENV_PATH, name)
		cmd = fmt.Sprintf("source %s/%s/bin/activate", VENV_PATH, name)
	}
	WriteCmd(cmd)
}
