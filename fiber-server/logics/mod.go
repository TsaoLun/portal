package logics

import (
	"context"
	"fmt"
	"log"

	"github.com/TsaoLun/portal/fiber-server/auth"
	"github.com/TsaoLun/portal/fiber-server/graph/model"
	"github.com/TsaoLun/portal/fiber-server/utils"
)

type LoginArgs struct {
	Username string
	Password string
}

func Login(ctx context.Context, args LoginArgs) (*model.TokenResponse, error) {
	if !(args.Username == auth.Username && args.Password == auth.Password) || auth.JwtKey == "" {
		log.Printf("Invalid username or password from %s\n", args.Username)
		return &model.TokenResponse{
			Token: nil,
			Ok:    false,
		}, nil
	}
	// generate token
	token, err := utils.GenerateToken(auth.JwtKey, args.Username, utils.WEEK_MINUTES)
	if err != nil {
		return nil, err
	}
	return &model.TokenResponse{
		Token: &token,
		Ok:    true,
	}, nil
}

func Get(ctx context.Context) (string, error) {
	v, _ := utils.Get("0")
	// if !ok {
	// 	return "", fmt.Errorf("data not found")
	// }
	return v, nil
}

func Set(ctx context.Context, data string) (bool, error) {
	if data == "" {
		return false, fmt.Errorf("data is empty")
	}
	utils.Set("0", data)
	return true, nil
}
