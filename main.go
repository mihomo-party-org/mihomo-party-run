//go:build windows
// +build windows

package main

import (
	"errors"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"syscall"
	"unicode/utf16"
	"unsafe"
)

var (
	user32          = syscall.NewLazyDLL("user32.dll")
	procMessageBoxW = user32.NewProc("MessageBoxW")
	mbOk            = uintptr(0)
	mbIconError     = uintptr(0x10)
	messageBoxFlags = mbOk | mbIconError
	messageBoxTitle = "Mihomo Party Runner"
	paramFileName   = "param.txt"
)

func main() {
	if err := run(); err != nil {
		showError("Error: " + err.Error())
	}
}

func run() error {
	args := os.Args
	if len(args) != 2 {
		return errors.New("invalid arguments")
	}

	exePath, err := os.Executable()
	if err != nil {
		return err
	}

	exeDir := filepath.Dir(exePath)
	paramPath := filepath.Join(exeDir, paramFileName)

	contentBytes, err := os.ReadFile(paramPath)
	if err != nil {
		contentBytes = []byte{}
	}
	content := strings.TrimSpace(string(contentBytes))

	cmd := exec.Command(args[1], content)
	err = cmd.Start()
	if err != nil {
		errorMessage := "Failed to start program\n" + args[1] + "\n" + err.Error() + "\n请尝试以管理员权限启动软件"
		return errors.New(errorMessage)
	}

	return nil
}

func UnicodeString(s string) *uint16 {
	encoded := utf16.Encode([]rune(s + "\x00"))
	return &encoded[0]
}

func showError(message string) {
	modaless, _ := syscall.UTF16PtrFromString(messageBoxTitle)
	msg, _ := syscall.UTF16PtrFromString(message)
	procMessageBoxW.Call(0,
		uintptr(unsafe.Pointer(msg)),
		uintptr(unsafe.Pointer(modaless)),
		messageBoxFlags)
}
