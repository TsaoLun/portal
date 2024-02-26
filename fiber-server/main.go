package main

import (
	"context"
	"errors"

	jwtware "github.com/gofiber/contrib/jwt"
	"github.com/gofiber/fiber/v2"
	"github.com/golang-jwt/jwt/v5"
	"github.com/vektah/gqlparser/v2/gqlerror"

	"github.com/99designs/gqlgen/graphql"
	"github.com/99designs/gqlgen/graphql/handler"
	"github.com/99designs/gqlgen/graphql/playground"
	"github.com/TsaoLun/portal/fiber-server/auth"
	"github.com/TsaoLun/portal/fiber-server/graph"
	"github.com/TsaoLun/portal/fiber-server/utils"
	"github.com/gofiber/fiber/v2/middleware/adaptor"
)

func main() {
	utils.InitStorage()
	srv := handler.NewDefaultServer(graph.NewExecutableSchema(graph.Config{Resolvers: &graph.Resolver{}}))
	srv.SetRecoverFunc(func(ctx context.Context, err any) error {
		// detect ErrUnauthorized
		if errors.Is(err.(error), auth.ErrUnauthorized) {
			return &gqlerror.Error{
				Path:    graphql.GetPath(ctx),
				Message: "unauthorized",
				Extensions: map[string]any{
					"code": "EXPIRED_TOKEN",
				},
			}
		}
		return gqlerror.Errorf("internal server error")
	})

	app := fiber.New()
	app.Use(jwtware.New(jwtware.Config{
		SigningKey: jwtware.SigningKey{
			Key: []byte(auth.JwtKey),
		},
		SuccessHandler: func(c *fiber.Ctx) error {
			c.Locals("user", c.Locals("user").(*jwt.Token).Claims.(jwt.MapClaims)["user"])
			return c.Next()
		},
		ErrorHandler: func(c *fiber.Ctx, err error) error {
			return c.Next()
		},
	}))
	app.All("/", adaptor.HTTPHandlerFunc(playground.Handler("GraphQL playground", "/graphql")))
	app.All("/graphql", adaptor.HTTPHandler(srv))

	app.Listen(":8008")
}
