package internal

import (
	"context"
	"encoding/json"
	"github.com/gorilla/mux"
	"github.com/sirupsen/logrus"
	"net/http"
	"strconv"
	"strings"
)

type handler struct {
	database    Database
}

func MakeHandler(mr *mux.Router, database Database) http.Handler {


	h := &handler{
		database: database,
	}

	mr.HandleFunc("/beer/", h.FindAllBeers).Methods("GET")
	mr.HandleFunc("/beer/{id}", h.FindBeerByID).Methods("GET")


	mr.HandleFunc("/brewery/", h.FindAllBreweries).Methods("GET")
	mr.HandleFunc("/brewery/{id}", h.FindBreweryByID).Methods("GET")
	mr.HandleFunc("/brewery/{id}/beer/", h.FindBreweryBeers).Methods("GET")

	return mr
}

func (h *handler) FindBeerByID(w http.ResponseWriter, r *http.Request) {


	ctx := r.Context()
	vars := mux.Vars(r)

	id, ok := vars["id"]
	if !ok {
		h.makeError(w, http.StatusBadRequest, "missing id", "findBeerByID")
		return
	}

	entity, err := h.database.FindBeerByID(ctx, id)
	if err != nil {
		h.makeError(w, http.StatusInternalServerError, "unable to fetch breweries from db", "findBeerByID")
		return
	}

	h.encodeResponse(ctx, w, entity)


}

func (h *handler) FindAllBeers(w http.ResponseWriter, r *http.Request) {

	ctx := r.Context()
	params := r.URL.Query()

	size := 25

	pageParams := params["page"]
	page := 0
	if len(pageParams) == 1 {
		val, err := strconv.Atoi(pageParams[0])
		if err != nil {
			logrus.WithField("err", err.Error()).Warn("unable to parse page, using 0")
		}
		page = val
	}

	entities, err := h.database.FindAllBeers(ctx, page, size)
	if err != nil {
		h.makeError(w, http.StatusInternalServerError, "unable to fetch beers from db", "findAllBeers")
		return
	}

	h.encodeResponse(ctx, w, entities)
}

func (h *handler) FindAllBreweries(w http.ResponseWriter, r *http.Request) {
	ctx := r.Context()
	params := r.URL.Query()

	size := 25

	pageParams := params["page"]
	page := 0
	if len(pageParams) == 1 {
		val, err := strconv.Atoi(pageParams[0])
		if err != nil {
			logrus.WithField("err", err.Error()).Warn("unable to parse page, using 0")
		}
		page = val
	}

	entities, err := h.database.FindAllBreweries(ctx, page, size)
	if err != nil {
		h.makeError(w, http.StatusInternalServerError, "unable to fetch breweries from db", "findAllBreweries")
		return
	}

	h.encodeResponse(ctx, w, entities)


}

func (h *handler) FindBreweryByID(w http.ResponseWriter, r *http.Request) {

	ctx := r.Context()
	vars := mux.Vars(r)

	id, ok := vars["id"]
	if !ok {
		h.makeError(w, http.StatusBadRequest, "missing id", "findBreweryByID")
		return
	}

	entity, err := h.database.FindBreweryByID(ctx, id)
	if err != nil {
		h.makeError(w, http.StatusInternalServerError, "unable to fetch breweries from db", "findBreweryByID")
		return
	}

	h.encodeResponse(ctx, w, entity)
}

func (h *handler) FindBreweryBeers(w http.ResponseWriter, r *http.Request) {
	ctx := r.Context()
	vars := mux.Vars(r)

	id, ok := vars["id"]
	if !ok {
		h.makeError(w, http.StatusBadRequest, "missing id", "findBreweryByID")
		return
	}

	entities, err := h.database.FindAllBreweryBeers(ctx, id)
	if err != nil {
		h.makeError(w, http.StatusInternalServerError, "unable to fetch breweries from db", "findBreweryBeers")
		return
	}

	h.encodeResponse(ctx, w, entities)
}

func (h *handler) makeError(w http.ResponseWriter, code int, message string, method string) {
	logrus.WithFields(
		logrus.Fields{
			"type":   code,
			"method": method,
		}).Error(strings.ToLower(message))
	http.Error(w, message, code)
}

func (h *handler) encodeResponse(ctx context.Context, w http.ResponseWriter, response interface{}) error {
	w.Header().Set("Content-Type", "application/json; charset=utf-8")
	return json.NewEncoder(w).Encode(response) //TODO check error and handle?
}
