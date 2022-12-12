package main

import (
	"bufio"
	"fmt"
	"os"
)

func GetInput(prompt string) string {
	scanner := bufio.NewScanner(os.Stdin)
	fmt.Print(prompt)
	scanner.Scan()
	err := scanner.Err()
	Check(err)
	return scanner.Text()
}

func WriteCmd(cmd string) {
	// append to history file
	f, err := os.OpenFile(historyPath, os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	Check(err)
	defer f.Close()

	cmd = newLine + cmd
	_, err = f.Write([]byte(cmd))
	Check(err)
}

func Check(e error) {
	if e != nil {
		panic(e)
	}
}

func CheckChoice(choice string) {
	if choice == "" {
		os.Exit(64)
	}
}
func Err(message string, code int) {
	fmt.Println(message)
	os.Exit(code)
}

func Cleanup() {
	// limit history file size
	f, err := os.OpenFile(historyPath, os.O_RDONLY, 0644)
	Check(err)
	defer f.Close()
	info, _ := f.Stat()

	if info.Size() > 1 { //1e6 {
		// fmt.Println(info.Size())

		// i, err := f.Seek(-30, 2)
		// fmt.Println(i)
		// Check(err)
		// var end []byte
		// i64, err := f.Read(end)
		// fmt.Println(i64)
		// Check(err)
		// endStr := strings.Split(string(end), "\n")
		// fmt.Println(endStr)
		// // err = os.Remove(historyPath)
		// Check(err)
	}
}
