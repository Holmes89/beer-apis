(ns ^:figwheel-no-load beer-app.dev
  (:require
    [beer-app.core :as core]
    [devtools.core :as devtools]))


(enable-console-print!)

(devtools/install!)

(core/init!)
