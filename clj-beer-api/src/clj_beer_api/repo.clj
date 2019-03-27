(ns clj-beer-api.repo
  (:require [clojure.java.jdbc :as sql]
            [clojure.tools.logging :as log]))

(def db-spec {:classname "org.sqlite.JDBC"
              :subprotocol "sqlite"
              :subname "resources/beers.db"})

(def size 25)
(defn get-offset
  [page]
  (if (nil? page) 0 (* (read-string page) size)))

(defn find-all-breweries
  [page]
  (sql/query db-spec ["SELECT * From breweries LIMIT ?,?" (get-offset page) size]))

(defn find-brewery-by-id
  [id]
  (first (sql/query db-spec ["SELECT * FROM breweries where id = ?" id])))

(defn find-beer-by-id
  [id]
  (first (sql/query db-spec ["SELECT * FROM beers where id = ?" id])))

(defn find-all-beers
  [page]
  (sql/query db-spec ["SELECT * FROM beers LIMIT ?,?" (get-offset page) size]))

(defn find-all-brewery-beers
  [id]
  (log/info "finding beers for brewery" id)
  (sql/query db-spec ["SELECT * FROM beers WHERE brewery_id = ?" id]))
