package main

import (
	"bufio"
	"fmt"
	"govenv/gocliselect"
	"os"
	// "github.com/rrossmiller/gocliselect"
)

const (
	CREATE  = "create"
	DELETE  = "delete"
	ACIVATE = "activate"
)

func InteractiveMode(args []string) {
	menu := gocliselect.NewMenu("Pick one")
	menu.VimKeys = true

	menu.AddItem("Create", CREATE)
	menu.AddItem("Delete", DELETE)
	menu.AddItem("Acivate", ACIVATE)

	choice := menu.Display()
	gocliselect.ClearScreen(menu)

	var cmd string
	var name string

	// if the name isn't supplied, show existing venvs
	if len(args) == 0 {
		// add functionality to list venvs
		venvs, err := os.ReadDir(home + "/.venv")
		Check(err)
		venvMenu := gocliselect.NewMenu("Existing venvs")
		venvMenu.VimKeys = true

		venvMenu.AddItem("New", "new")
		for _, v := range venvs {
			if v.IsDir() {
				// make new menu with venvs as options. first option being type in a venv name
				venvMenu.AddItem(v.Name(), v.Name())
			}
		}

		name = venvMenu.Display()
		if name == "new" {
			gocliselect.ClearScreen(venvMenu)
			scanner := bufio.NewScanner(os.Stdin)
			fmt.Print("Name of venv: ")
			scanner.Scan()
			err = scanner.Err()
			Check(err)
			name = scanner.Text()
		}
	} else {
		name = args[0]
	}

	switch choice {
	case CREATE:
		fmt.Printf("Creating %s/%s\n", VENV_PATH, name)
		cmd = fmt.Sprintf("python3 -m venv %s/%s", VENV_PATH, name)
	case DELETE:
		fmt.Printf("Deleting %s/%s", VENV_PATH, name)
		err := os.Remove(fmt.Sprintf("%s/%s", VENV_PATH, name))
		Check(err)
	case ACIVATE:
		fmt.Printf("Activating %s/%s\n", VENV_PATH, name)
		cmd = fmt.Sprintf("source %s/%s/bin/activate", VENV_PATH, name)
	}
	WriteCmd(cmd)
}
