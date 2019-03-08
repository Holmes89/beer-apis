(ns beer-app.core
  (:require
   [reagent.core :as r]
   [beer-app.components.breweries :refer [breweries]]))

;; -------------------------
;; Views

;; -------------------------
;; Initialize app
(defn mount-root []
  (r/render [breweries] (.getElementById js/document "app")))

(defn init! []  
  (mount-root))
