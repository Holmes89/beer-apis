(ns beer-app.home
  (:require [beer-app.components.breweries :refer [list-breweries]]))

;; -------------------------
;; Views

;; -------------------------
;; Page
(defn home-page []
  [:section.section
   [:div.container
    [:h1.title.has-text-centered "Breweries"]
    [:div.columns.is-centered    
     [list-breweries]]]])


