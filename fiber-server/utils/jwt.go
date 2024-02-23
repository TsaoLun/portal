package utils

import (
	"github.com/dgrijalva/jwt-go"
	"time"
)

const WEEK_MINUTES int64 = 7 * 24 * 60

func GenerateToken(jwtKey string, username string, exp int64) (string, error) {
	// 创建一个新的 token 对象，指定签名方法和 claims
	token := jwt.NewWithClaims(jwt.SigningMethodHS512, jwt.MapClaims{
		"username": username,
		"exp":      time.Now().Add(time.Duration(exp) * time.Minute).Unix(),
	})

	// 使用指定的 secret 签名并获得完整的编码后的字符串 token
	tokenString, err := token.SignedString([]byte(jwtKey))
	if err != nil {
		return "", err
	}

	return tokenString, nil
}
