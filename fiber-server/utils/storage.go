package utils

import (
	"sync"
)

var storage *sync.Map

func InitStorage() {
	storage = &sync.Map{}
}

func Set(key string, value string) {
	storage.Store(key, value)
}

func Get(key string) (string, bool) {
	value, ok := storage.Load(key)
	if !ok {
		return "", false
	}
	return value.(string), true
}
