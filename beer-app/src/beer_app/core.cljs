(ns beer-app.core
  (:require-macros [secretary.core :refer [defroute]])
  (:import [goog History]
           [goog.history EventType])
  (:require
   [beer-app.home :refer [home-page]]
   [beer-app.brewery :refer [brewery-page]]
   [secretary.core :as secretary]
   [goog.events :as events]
   [goog.history.EventType :as EventType]
   [reagent.core :as r]
   [reagent.session :as session]))

(defn page []
  [(session/get :current-page)])

;; -------------------------
;; Routes
(secretary/set-config! :prefix "#")

(defroute "/" []
  (session/put! :current-page home-page))

(defroute "/home" []
  (session/put! :current-page home-page))

(defroute "/brewery/:id" [id]
  (session/put! :current-page #(brewery-page id)))

(doto (History.)
  (events/listen EventType.NAVIGATE #(secretary/dispatch! (.-token %)))
  (.setEnabled true))
;; -------------------------
;; Initialize app


(defn mount-root []
  (r/render [#'page] (.getElementById js/document "app")))
(defn init! []  
  (mount-root))
