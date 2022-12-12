package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
	"unicode/utf8"
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
	// if the file is greater than 500b
	if info.Size() > 500 {
		// seek to the last 100 bytes
		seekCap := 100
		s := info.Size() - int64(seekCap)
		_, err := f.Seek(s, 0)
		Check(err)

		// read the last 100 bytes
		end := make([]byte, seekCap)
		_, err = f.Read(end)
		Check(err)

		// find the last line with text
		endStrSpl := strings.Split(string(end), "\n")
		idx := 1
		endStr := strings.TrimSpace(endStrSpl[len(endStrSpl)-idx])
		for utf8.RuneCountInString(endStr) == 0 && idx < len(endStrSpl) {
			idx++
			endStr = strings.TrimSpace(endStrSpl[len(endStrSpl)-idx])
		}

		err = os.Remove(historyPath)
		Check(err)
		WriteCmd(endStr)
	}
}
