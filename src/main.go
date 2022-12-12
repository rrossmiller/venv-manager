package main

import (
	"errors"
	"flag"
	"fmt"
	"log"
	"os"
	"runtime"
)

/*
Enter or create env

```
> venv env1

Python 3.11.0
(env1) >
```

Create venv with specific version

```
> venv env2 -c 10
Python 3.10.6
(env2) >
```

delete envs (deactivate then delete if in use)

```
venv -d env2
```
*/

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
			fmt.Println(runtimeOS)
		case "windows": // checkme
			newLine = "\r\n"
		}
		home = "/USERS/" + user
	}
	VENV_PATH = home + "/.venv"
	historyPath = home + "/.venv/history"

	// make ~/.venv if it doesn't exist
	if _, err := os.Stat(VENV_PATH); errors.Is(err, os.ErrNotExist) {
		os.Mkdir(VENV_PATH, 0755)
	}
}

func main() {
	interactive := flag.Bool("i", true, "Interactive mode")
	// createFlag := flag.Bool("c", false, "Create a new venv")
	// freezeAllFlag := flag.Bool("F", false, "Freeze the current state of all venvs")

	flag.Parse()
	args := flag.Args()

	if *interactive {
		InteractiveMode(args)
	}
	Cleanup()
}
