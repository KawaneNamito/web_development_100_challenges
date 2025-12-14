package main

import (
	"context"
	"log"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/KawaneNamito/web100/internal/handler"
	"github.com/KawaneNamito/web100/internal/openapi"
	"github.com/KawaneNamito/web100/internal/repository"
	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"
	"github.com/go-chi/cors"
	"github.com/joho/godotenv"
)

func main() {
	// .envファイルから環境変数を読み込む（存在しない場合はスキップ）
	// 複数のパスを試行：カレントディレクトリ、親ディレクトリ、プロジェクトルート
	for _, path := range []string{".env", "../.env", "../../.env"} {
		if err := godotenv.Load(path); err == nil {
			break
		}
	}

	// 環境変数からデータベース接続文字列を取得
	dbURL := os.Getenv("DATABASE_URL")
	// ポート番号を取得
	port := os.Getenv("PORT")
	// データベース接続
	ctx := context.Background()
	db, err := repository.NewDB(ctx, dbURL)
	if err != nil {
		log.Fatalf("Failed to connect to database: %v", err)
	}
	defer db.Close()

	// リポジトリの初期化
	streamRepo := repository.NewStreamRepository(db)

	// ハンドラーの初期化
	h := handler.NewHandler(streamRepo)

	// ルーターの設定
	r := chi.NewRouter()
	r.Use(cors.Handler(cors.Options{
		AllowedOrigins:   []string{"*"},
		AllowedMethods:   []string{"GET", "POST", "PUT", "DELETE", "OPTIONS"},
		AllowedHeaders:   []string{"Accept", "Content-Type"},
		AllowCredentials: false,
		MaxAge:           300,
	}))
	r.Use(middleware.Logger)
	r.Use(middleware.Recoverer)
	r.Use(middleware.RequestID)

	// OpenAPIハンドラーの登録
	openapi.HandlerFromMux(h, r)

	// サーバーの設定
	srv := &http.Server{
		Addr:         ":" + port,
		Handler:      r,
		ReadTimeout:  10 * time.Second,
		WriteTimeout: 10 * time.Second,
	}

	// グレースフルシャットダウンの設定
	go func() {
		sigChan := make(chan os.Signal, 1)
		signal.Notify(sigChan, syscall.SIGINT, syscall.SIGTERM)
		<-sigChan

		log.Println("Shutting down server...")
		shutdownCtx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
		defer cancel()

		if err := srv.Shutdown(shutdownCtx); err != nil {
			log.Printf("HTTP server shutdown error: %v", err)
		}
	}()

	// サーバー起動
	log.Printf("Starting server on port %s", port)
	if err := srv.ListenAndServe(); err != http.ErrServerClosed {
		log.Fatalf("HTTP server error: %v", err)
	}

	log.Println("Server stopped")
}
