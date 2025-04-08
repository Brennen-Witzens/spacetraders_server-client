package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"log"

	//"log/slog"
	"net/http"
	"os"

	"github.com/joho/godotenv"
)

func hello(w http.ResponseWriter, req *http.Request) {

	fmt.Fprint(w, "Hello")
}

// TODO: This does need to change, the endpoint here would be a different style of endpoint
// This would be an endpoint created via graphql, protobuf, openapi and then within it would call a function that makes the request to the spacetraders endpoint
// This would be a public route, since we don't need token at the moment
func registerAccount(w http.ResponseWriter, req *http.Request) {

}

func main() {

	err := godotenv.Load(".env")

	if err != nil {
		log.Fatal("Failed to load .env file: %s", err)
	}

	accountToken := os.Getenv("ACCOUNT_TOKEN")

	body := map[string]string{"symbol": "Garfunckel", "faction": "COSMIC"}
	json_data, err := json.Marshal(body)
	if err != nil {
		log.Fatal(err)
	}

	client := &http.Client{}
	req, err := http.NewRequest("POST", "https://api.spacetraders.io/v2/register", bytes.NewBuffer(json_data))
	req.Header.Add("Content-Type", "application/json")

	authHeader := "Bearer " + accountToken
	req.Header.Add("Authorization", authHeader)

	if err != nil {
		log.Fatal(err)
	}

	resp, err := client.Do(req)

	if err != nil {
		log.Fatal(err)
	}

	defer resp.Body.Close()
	respBody, _ := io.ReadAll(resp.Body)

	fmt.Println(resp)
	fmt.Println(string(respBody))

	//http.HandleFunc("/hello", hello)
	//http.ListenAndServe(":8080", nil)

}
