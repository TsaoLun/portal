package graph

// This file will be automatically regenerated based on the schema, any resolver implementations
// will be copied through when generating and any unknown code will be moved to the end.
// Code generated by github.com/99designs/gqlgen version v0.17.44

import (
	"context"
	"fmt"
	"log"
	"os"

	"github.com/TsaoLun/portal/fiber-server/graph/model"
	"github.com/TsaoLun/portal/fiber-server/utils"
)

// Set is the resolver for the set field.
func (r *mutationResolver) Set(ctx context.Context, data string) (bool, error) {
	if data == "" {
		return false, fmt.Errorf("data is empty")
	}
	utils.Set("0", data)
	return true, nil
}

// Get is the resolver for the get field.
func (r *queryResolver) Get(ctx context.Context) (string, error) {
	v, ok := utils.Get("0")
	if !ok {
		return "", fmt.Errorf("data not found")
	}
	return v, nil
}

// Login is the resolver for the login field.
func (r *queryResolver) Login(ctx context.Context, username string, password string) (*model.TokenResponse, error) {
	// read from env
	u := os.Getenv("PORTAL_USERNAME")
	p := os.Getenv("PORTAL_PASSWORD")
	jwtKey := os.Getenv("PORTAL_JWT_KEY")
	if !(username == u && password == p) || jwtKey == "" {
		log.Printf("Invalid username or password from %s\n", username)
		return &model.TokenResponse{
			Token: nil,
			Ok:    false,
		}, nil
	}
	// generate token
	token, err := utils.GenerateToken(jwtKey, username, utils.WEEK_MINUTES)
	if err != nil {
		return nil, err
	}
	return &model.TokenResponse{
		Token: &token,
		Ok:    true,
	}, nil

}

// Mutation returns MutationResolver implementation.
func (r *Resolver) Mutation() MutationResolver { return &mutationResolver{r} }

// Query returns QueryResolver implementation.
func (r *Resolver) Query() QueryResolver { return &queryResolver{r} }

type mutationResolver struct{ *Resolver }
type queryResolver struct{ *Resolver }
