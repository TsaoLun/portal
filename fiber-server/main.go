package main

import (
	"github.com/gofiber/fiber/v2"

	"github.com/99designs/gqlgen/graphql/handler"
	"github.com/99designs/gqlgen/graphql/playground"
	"github.com/TsaoLun/portal/fiber-server/graph"
	"github.com/gofiber/fiber/v2/middleware/adaptor"
)

func main() {
	srv := handler.NewDefaultServer(graph.NewExecutableSchema(graph.Config{Resolvers: &graph.Resolver{}}))

	app := fiber.New()
	app.All("/", adaptor.HTTPHandlerFunc(playground.Handler("GraphQL playground", "/query")))
	app.All("/graphql", adaptor.HTTPHandler(srv))

	app.Listen(":8008")
}
