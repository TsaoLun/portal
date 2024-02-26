package auth

import (
	"context"
	"errors"
	"os"
)

var Username = os.Getenv("PORTAL_USERNAME")
var Password = os.Getenv("PORTAL_PASSWORD")
var JwtKey = os.Getenv("PORTAL_JWT_KEY")

// type ResorverFunc[T any, V any] func(ctx context.Context, args ...T) (V, error)
type ResolverFuncWithArgs[V any, T any] func(ctx context.Context, args T) (V, error)

type ResolverFuncWithoutArgs[V any] func(ctx context.Context) (V, error)

var ErrUnauthorized = errors.New("unauthorized")

func WrapWithArgs[V any, T any](ctx context.Context, fn ResolverFuncWithArgs[V, T], args T) (V, error) {
	user := ctx.Value("user")
	if user == nil {
		panic(ErrUnauthorized)
	}
	return fn(ctx, args)
}

func WrapWithoutArgs[V any](ctx context.Context, fn ResolverFuncWithoutArgs[V]) (V, error) {
	user := ctx.Value("user")
	if user == nil {
		panic(ErrUnauthorized)
	}
	return fn(ctx)
}
