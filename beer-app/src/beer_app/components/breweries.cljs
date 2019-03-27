(ns beer-app.components.breweries
  (:require [ajax.core :as ajx]
            [reagent.core :as r]))

(defonce state
  (r/atom {:breweries []
           :page 0
           }))

(defn load-breweries! "Fetches the list of breweries from the server and updates the state atom with it" 
  []
  (ajx/GET (str "http://localhost:3000/brewery/?page=" (get-in @state [:page]))  
      {:handler (fn [breweries] (swap! state assoc :breweries breweries))
       :error-handler (fn [details] (.warn js/console (str "Failed to refresh phones from server: " details)))
       :response-format :json, :keywords? true}))

(defn brewery-item
  [{:keys [id name city state] :as brewery}]
  [:tr
   [:td [:a {:href (str "#/brewery/" id)} name]]
   [:td city]
   [:td state]])

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
     [:table.table.is-striped
      [:thead
       [:tr
        [:td "Name"]
        [:td "City"]
        [:td "State"]]]
      [:tbody 
       (for [brewery (get-in @state [:breweries])]
         ^{:key (get brewery :id)}[brewery-item brewery])]]
     [:div.columns.is-centered
      [:div.button.prev {:on-click #(prev-page)} "Prev"]
      [:div.button.next.is-pulled-right {:on-click #(next-page)} "Next"]]]))

(defn breweries []
  [list-breweries])
