package main

import "fmt"

func getMessageWithRetries(plan string) ([]string, error)
		allMessages := getMessageWithRetries()
		if plan == planPro {
			return allMessages[:], nil
		}

		if plan == planFree {
			return allMessages[0:2], nil
		}
		return nil, errors.New("unsupported")
}

func (f *File) Read(buf []byte) (n int, err error)
