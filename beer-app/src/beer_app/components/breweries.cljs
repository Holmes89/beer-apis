(ns beer-app.components.breweries
  (:require [ajax.core :as ajx]
            [reagent.core :as r]))

(defonce state
  (r/atom {:breweries []
           :page 0
           }))

(defn load-breweries! "Fetches the list of breweries from the server and updates the state atom with it" 
  []
  (ajx/GET (str "http://localhost:8080/brewery/?page=" (get-in @state [:page]))  
      {:handler (fn [breweries] (swap! state assoc :breweries breweries))
       :error-handler (fn [details] (.warn js/console (str "Failed to refresh phones from server: " details)))
       :response-format :json, :keywords? true}))

(defn brewery-item
  [{:keys [id name address city state zip] :as brewery}]  
  [:div.brewery
   [:div.name name]
   [:div.address address]
   [:div (str city ", " state " " zip)]])

(defn next-page
  []
  (swap! state update-in [:page] inc)  
  (load-breweries!))

(defn dec-to-zero
  [x]
  (if (> x 0) (dec x) 0))

(defn prev-page
  []
  (swap! state update-in [:page] dec-to-zero)  
  (load-breweries!))

(defn list-breweries []
  (load-breweries!)
  (fn []
    [:div
     [:h1 "Brewery List"]
     [:div#breweries 
      (for [brewery (get-in @state [:breweries])]
        ^{:key (get brewery :id)}[brewery-item brewery])]
     [:div.button.prev {:on-click #(prev-page)} "Prev"]
     [:div.button.next {:on-click #(next-page)} "Next"]]))

(defn breweries []
  [list-breweries])
