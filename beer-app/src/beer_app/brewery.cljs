(ns beer-app.brewery
  (:require [beer-app.components.brewery-profile :refer [get-profile]]
            [beer-app.components.beer-list :refer [list-beers]]))

(defn brewery-page [id]
  [:div   
   [:section.section
    [:div
     [:a {:href "#/"}
      [:div.icon
       [:i.fas.fa-arrow-left] "Back"]]]
    [:div
     [get-profile id]]]
   [:section.section
    [:div
     [:div.columns.is-centered
      [list-beers id]]]]])
