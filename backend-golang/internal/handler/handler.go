package handler

import (
	"encoding/json"
	"net/http"

	"github.com/KawaneNamito/web100/internal/openapi"
	"github.com/KawaneNamito/web100/internal/repository"
	"github.com/google/uuid"
)

// Handler はAPIハンドラーの実装
type Handler struct {
	streamRepo repository.StreamRepository
}

// NewHandler は新しいHandlerを作成する
func NewHandler(streamRepo repository.StreamRepository) *Handler {
	return &Handler{
		streamRepo: streamRepo,
	}
}

// 確実にServerInterfaceを実装していることをコンパイル時にチェック
var _ openapi.ServerInterface = (*Handler)(nil)

// GetAPIV1Streams はYouTube配信情報の一覧を取得する
// (GET /api/v1/streams)
func (h *Handler) GetAPIV1Streams(w http.ResponseWriter, r *http.Request) {
	ctx := r.Context()

	streams, err := h.streamRepo.FindAll(ctx)
	if err != nil {
		h.writeServerError(w, err)
		return
	}

	response := make([]openapi.StreamSummary, 0, len(streams))
	for _, s := range streams {
		streamID := s.StreamID.String()
		userID := s.UserID.String()
		createdAt := s.CreatedAt
		response = append(response, openapi.StreamSummary{
			StreamID:  &streamID,
			UserID:    &userID,
			Title:     &s.Title,
			CreatedAt: &createdAt,
		})
	}

	h.writeJSON(w, http.StatusOK, response)
}

// PostAPIV1Streams はYouTube配信情報を登録する
// (POST /api/v1/streams)
func (h *Handler) PostAPIV1Streams(w http.ResponseWriter, r *http.Request) {
	ctx := r.Context()

	var req openapi.PostAPIV1StreamsJSONRequestBody
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		h.writeValidationError(w, "Invalid request body")
		return
	}

	userID, err := uuid.Parse(req.UserID)
	if err != nil {
		h.writeValidationError(w, "Invalid userId format")
		return
	}

	stream := &repository.Stream{
		StreamID:    uuid.New(),
		UserID:      userID,
		Title:       req.Title,
		Description: req.Description,
	}

	if err := h.streamRepo.Create(ctx, stream); err != nil {
		h.writeServerError(w, err)
		return
	}

	streamID := stream.StreamID.String()
	userIDStr := stream.UserID.String()
	createdAt := stream.CreatedAt
	response := openapi.Stream{
		StreamID:    &streamID,
		UserID:      &userIDStr,
		Title:       &stream.Title,
		Description: &stream.Description,
		CreatedAt:   &createdAt,
	}

	h.writeJSON(w, http.StatusCreated, response)
}

// DeleteAPIV1StreamsStreamID はYouTube配信情報を削除する
// (DELETE /api/v1/streams/{streamId})
func (h *Handler) DeleteAPIV1StreamsStreamID(w http.ResponseWriter, r *http.Request, streamID string) {
	ctx := r.Context()

	id, err := uuid.Parse(streamID)
	if err != nil {
		h.writeValidationError(w, "Invalid streamId format")
		return
	}

	if err := h.streamRepo.Delete(ctx, id); err != nil {
		h.writeServerError(w, err)
		return
	}

	w.WriteHeader(http.StatusNoContent)
}

// GetAPIV1StreamsStreamID はYouTube配信情報の詳細を取得する
// (GET /api/v1/streams/{streamId})
func (h *Handler) GetAPIV1StreamsStreamID(w http.ResponseWriter, r *http.Request, streamID string) {
	ctx := r.Context()

	id, err := uuid.Parse(streamID)
	if err != nil {
		h.writeValidationError(w, "Invalid streamId format")
		return
	}

	stream, err := h.streamRepo.FindByID(ctx, id)
	if err != nil {
		h.writeServerError(w, err)
		return
	}

	if stream == nil {
		h.writeNotFound(w, "Stream not found")
		return
	}

	streamIDStr := stream.StreamID.String()
	userIDStr := stream.UserID.String()
	createdAt := stream.CreatedAt
	response := openapi.Stream{
		StreamID:    &streamIDStr,
		UserID:      &userIDStr,
		Title:       &stream.Title,
		Description: &stream.Description,
		CreatedAt:   &createdAt,
	}

	h.writeJSON(w, http.StatusOK, response)
}

// writeJSON はJSONレスポンスを書き込む
func (h *Handler) writeJSON(w http.ResponseWriter, status int, data interface{}) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(status)
	json.NewEncoder(w).Encode(data)
}

// writeServerError はサーバーエラーレスポンスを書き込む
func (h *Handler) writeServerError(w http.ResponseWriter, err error) {
	errCode := "INTERNAL_SERVER_ERROR"
	message := "Internal server error"
	response := openapi.ServerError{
		Error:   &errCode,
		Message: &message,
	}
	h.writeJSON(w, http.StatusInternalServerError, response)
}

// writeValidationError はバリデーションエラーレスポンスを書き込む
func (h *Handler) writeValidationError(w http.ResponseWriter, message string) {
	errCode := "VALIDATION_ERROR"
	response := openapi.ValidationError{
		Error:   &errCode,
		Message: &message,
	}
	h.writeJSON(w, http.StatusBadRequest, response)
}

// writeNotFound はNotFoundレスポンスを書き込む
func (h *Handler) writeNotFound(w http.ResponseWriter, message string) {
	errCode := "NOT_FOUND"
	response := openapi.ServerError{
		Error:   &errCode,
		Message: &message,
	}
	h.writeJSON(w, http.StatusNotFound, response)
}
