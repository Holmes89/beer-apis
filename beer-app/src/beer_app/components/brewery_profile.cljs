(ns beer-app.components.brewery-profile
  (:require [ajax.core :as ajx]
            [reagent.core :as r]))

(defonce state
  (r/atom {:brewery {}}))

(defn load-brewery!
  [id]
  (ajx/GET (str "http://localhost:8080/brewery/" id))
  {:handler (fn [brewery] (swap! state assoc :brewery brewery))
   :error-handler (fn [details] (.warn js/console (str "Failed to refresh phones from server: " details)))
   :response-format :json, :keywords? true})
