(ns beer-app.components.beer-list
  (:require [ajax.core :as ajx]
            [reagent.core :as r]))
(defonce state
  (r/atom {:beers []}))

(defn load-beers! "Fetches the list of breweries from the server and updates the state atom with it" 
  [id]
  (ajx/GET (str "http://localhost:8080/brewery/?page=" (get-in @state [:page]))  
      {:handler (fn [breweries] (swap! state assoc :breweries breweries))
       :error-handler (fn [details] (.warn js/console (str "Failed to refresh phones from server: " details)))
       :response-format :json, :keywords? true}))
