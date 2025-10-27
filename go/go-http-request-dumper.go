package main

import (
	"encoding/json"
	"io/ioutil"
	"net/http"
)

type request struct {
	Host          string      `json:"host"`
	RemoteAddr    string      `json:"remote-addr"`
	URL           string      `json:"url"`
	RequestURI    string      `json:"request-uri"`
	Proto         string      `json:"proto"`
	Method        string      `json:"method"`
	Headers       http.Header `json:"headers"`
	ContentLength int64       `json:"content-length"`
	Body          []byte      `json:"body"`
}

func handle(rw http.ResponseWriter, r *http.Request) {
	var err error
	rr := &request{}
	rr.Method = r.Method
	rr.Headers = r.Header
	rr.URL = r.URL.String()
	rr.Body, err = ioutil.ReadAll(r.Body)
	rr.Host = r.Host
	rr.Proto = r.Proto
	rr.ContentLength = r.ContentLength
	rr.RemoteAddr = r.RemoteAddr
	rr.RequestURI = r.RequestURI
	if err != nil {
		http.Error(rw, err.Error(), http.StatusInternalServerError)
		return
	}

	rrb, err := json.Marshal(rr)
	if err != nil {
		http.Error(rw, err.Error(), http.StatusInternalServerError)
		return
	}

	rw.Header().Set("Content-Type", "application/json")
	rw.Write(rrb)
}

func main() {
	http.HandleFunc("/", handle)
	http.ListenAndServe(":8000", nil)
}
