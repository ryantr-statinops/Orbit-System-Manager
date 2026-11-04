package main

import (
	"embed"
	"log"
	"os"
	"path/filepath"

	"github.com/wailsapp/wails/v3/pkg/application"
)

//go:embed all:frontend/dist
var frontend embed.FS

func main() {
	cacheDir, err := os.UserCacheDir()
	if err != nil {
		log.Fatal(err)
	}
	webviewDataPath := filepath.Join(cacheDir, "OrbitSystemManager", "WebView2")
	if err := os.MkdirAll(webviewDataPath, 0o700); err != nil {
		log.Fatal(err)
	}

	app := application.New(application.Options{
		Name:        "Orbit System Manager",
		Description: "Windows system telemetry dashboard",
		Windows: application.WindowsOptions{
			WebviewUserDataPath: webviewDataPath,
		},
		Assets: application.AssetOptions{
			Handler: application.AssetFileServerFS(frontend),
		},
	})

	app.Window.NewWithOptions(application.WebviewWindowOptions{
		Title:            "Orbit System Manager",
		Width:            1440,
		Height:           900,
		MinWidth:         900,
		MinHeight:        620,
		BackgroundColour: application.NewRGB(12, 17, 26),
		URL:              "/",
	})

	if err := app.Run(); err != nil {
		log.Fatal(err)
	}
}
