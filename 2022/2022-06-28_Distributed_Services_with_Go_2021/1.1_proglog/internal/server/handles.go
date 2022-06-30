package server

import (
	"encoding/json"
	"net/http"
)

func (s *httpServer) handleProduce(w http.ResponseWriter, r *http.Request) {
	var (
		off uint64
		err error
		req ProduceRequest
		res ProduceResponse
	)

	if err = json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	if off, err = s.Log.Append(req.Record); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	res = ProduceResponse{Offset: off}
	if err = json.NewEncoder(w).Encode(res); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
}

func (s *httpServer) handleConsume(w http.ResponseWriter, r *http.Request) {
	var (
		err    error
		record Record
		req    ConsumeRequest
		res    ConsumeResponse
	)

	if err = json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	if record, err = s.Log.Read(req.Offset); err == ErrOffsetNotFound {
		http.Error(w, err.Error(), http.StatusNotFound)
		return
	}

	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	res = ConsumeResponse{Record: record}
	if err = json.NewEncoder(w).Encode(res); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
}
