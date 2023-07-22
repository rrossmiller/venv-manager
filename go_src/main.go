package main

import (
	"errors"
	"flag"
	"fmt"
	"log"
	"os"
	"runtime"

	"github.com/buger/goterm"
)

var (
	home = os.Getenv("HOME")
	user = os.Getenv("USER")
	// gopath = os.Getenv("GOPATH")
	newLine     = "\n"
	VENV_PATH   string
	historyPath string
)

func init() {
	if user == "" {
		log.Fatal("$USER not set")
	}

	runtimeOS := runtime.GOOS
	if home == "" {
		switch runtimeOS {
		case "darwin":
			home = "/Users" + user
		}
	}
	VENV_PATH = home + "/.venv"
	historyPath = home + "/.venv/history"

	// make ~/.venv if it doesn't exist
	if _, err := os.Stat(VENV_PATH); errors.Is(err, os.ErrNotExist) {
		os.Mkdir(VENV_PATH, 0755)
	}
}

func main() {
	interactive := flag.Bool("i", false, "Interactive mode")
	list := flag.Bool("l", false, "List venv's")
	pth := flag.Bool("p", false, "Print the current python interpreter path")
	// createFlag := flag.Bool("c", false, "Create a new venv")
	// freezeAllFlag := flag.Bool("F", false, "Freeze the current state of all venvs")

	flag.Parse()
	args := flag.Args()

	clear := true
	switch {
	case *list:
		venvs, err := os.ReadDir(VENV_PATH)
		Check(err)

		// first entry of venv is the dir itself
		if len(venvs) < 2 {
			fmt.Println("There are no venvs to list")
			os.Exit(72) // exit codes: https://www.freebsd.org/cgi/man.cgi?query=sysexits&apropos=0&sektion=0&manpath=FreeBSD+14.0-CURRENT&arch=default&format=pdf
		}
		fmt.Println(goterm.Bold(goterm.Color("Available venv's", goterm.CYAN)))
		for _, v := range venvs {
			if v.IsDir() {
				fmt.Println(goterm.Color(v.Name(), goterm.YELLOW))
			}
		}
		os.Exit(-1)

	case *pth:
		clear = false
		WriteCmd("which python3", clear)

	case *interactive || len(args) == 0:
		InteractiveMode(args)

	case len(args) != 0:
		name := args[0]
		cmd := fmt.Sprintf("source %s/%s/bin/activate", VENV_PATH, name)

		WriteCmd(cmd, true)
	}
	Cleanup(clear)
}
