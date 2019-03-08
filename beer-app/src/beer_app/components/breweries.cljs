(ns beer-app.components.breweries
  (:require [ajax.core :as ajx]
            [reagent.core :as r]))

(defonce state
  (r/atom {:breweries []
           :page 1
           }))

(defn load-breweries! "Fetches the list of breweries from the server and updates the state atom with it" 
  []
  (ajx/GET "http://localhost:8080/brewery/?page=2" 
      {:handler (fn [breweries] (swap! state assoc :breweries breweries))
       :error-handler (fn [details] (.warn js/console (str "Failed to refresh phones from server: " details)))
       :response-format :json, :keywords? true}))

(defn brewery-item
  [{:keys [id name address city state zip] :as brewery}]  
  [:div.brewery
   [:div.name name]
   [:div.address address]
   [:div (str city ", " state " " zip)]])

(defn list-breweries []
  (let [breweries (get-in @state [:breweries])]
    (fn []
      [:div
       [:h1 "Brewery List"]
       [:div#breweries 
        (for [brewery breweries]
          ^{:key (get brewery :id)}[brewery-item brewery])]])))

(defn breweries []
  (load-breweries!)
  (list-breweries))
