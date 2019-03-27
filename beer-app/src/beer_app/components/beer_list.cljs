(ns beer-app.components.beer-list
  (:require [ajax.core :as ajx]
            [reagent.core :as r]))

(defonce state
  (r/atom {:beers []}))

(defn load-beers!
  [id]
  (ajx/GET (str "http://localhost:3000/brewery-beer/" id )  
      {:handler (fn [beers] (swap! state assoc :beers beers))
       :error-handler (fn [details] (.warn js/console (str "Failed to refresh phones from server: " details)))
       :response-format :json, :keywords? true}))

(defn beer-item
  [{:keys [id name] :as beer}]
  [:a.list-item name])

(defn list-beers [id]
  (load-beers! id)
  (fn []
    [:div.list.is-hoverable
     (for [beer (get-in @state [:beers])]
       ^{:key (get beer :id)} [beer-item beer])]))
