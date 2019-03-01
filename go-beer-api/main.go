package main

import (
	"beer-api/internal"
	"fmt"
	"github.com/gorilla/mux"
	"github.com/sirupsen/logrus"
	"net/http"
	"os"
	"os/signal"
	"syscall"
)

const defaultPort = "8080"

func main() {
	port := ":" + defaultPort

	//Create Database
	logrus.Info("connecting to db")
	db, err := internal.NewBeerDB()
	if err != nil {
		logrus.WithField("err", err.Error()).Fatal("unable to connect to db")
	}
	logrus.Info("connected to db")

	router := mux.NewRouter()
	internal.MakeHandler(router, db)

	errs := make(chan error, 2) // This is used to handle and log the reason why the application quit.
	go func() {
		logrus.WithFields(
			logrus.Fields{
				"transport": "http",
				"port":      port,
			}).Info("server started")
		errs <- http.ListenAndServe(port, router)
	}()
	go func() {
		c := make(chan os.Signal, 1)
		signal.Notify(c, syscall.SIGINT)
		errs <- fmt.Errorf("%s", <-c)
	}()

	logrus.WithField("error", <-errs).Error("terminated")

}
