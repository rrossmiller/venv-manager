package main

import (
	"flag"
	"fmt"
	"os"
	"os/exec"
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

func main() {
	createFlag := flag.Bool("c", false, "Create a new venv")
	t := flag.Bool("p", false, "Create a new venv")

	flag.Parse()
	fmt.Println(*createFlag)
	fmt.Println(*t)
	args := flag.Args()
	fmt.Println(args)
	if len(args) == 0 && !*createFlag {
		flag.CommandLine.Usage()
		os.Exit(0)
	}
	// fmt.Println("python3 -m venv venv; source venv/bin/activate")
}

func Check(e error) {
	if e != nil {
		panic(e)
	}
}

func Err(message string, code int) {
	fmt.Println(message)
	os.Exit(code)
}

func Finish() {
	exec.Command("rzshrc").Output()
}
